mod config;
mod input;
mod language;
mod leetcode;
mod progress;
mod pty;
mod tips;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use std::{
    fs,
    io,
    path::PathBuf,
    process::Command,
    time::Duration,
};

use crate::config::AppPaths;
use crate::input::key_to_bytes;
use crate::language::Language;
use crate::leetcode::{LeetCodeClient, Problem, TestMode};
use crate::progress::{ProblemStatus, Progress};
use crate::pty::PtyManager;

enum AppState {
    Home(HomeState),
    Question(QuestionState),
}

#[derive(PartialEq, Clone)]
enum FilterFocus {
    ProblemList,
    Search,
    ListFilter,
    Category,
    Difficulty,
    Progress,
}

struct HomeState {
    all_problems: Vec<Problem>,
    filtered_problems: Vec<Problem>,
    list_state: ListState,
    search_query: String,
    list_filters: Vec<String>,
    selected_list: usize, // 0 = All, 1 = Blind 75, 2 = NeetCode 150
    categories: Vec<String>,
    selected_category: usize, // 0 = All
    difficulties: Vec<String>,
    selected_difficulty: usize, // 0 = All
    progress_filters: Vec<String>,
    selected_progress: usize, // 0 = All
    filter_focus: FilterFocus,
    selected_language: Language,
}

struct QuestionState {
    problem: Problem,
    problem_text: String,
    language: Language,
    focus: Focus,
    scroll_offset: u16,
    results_scroll_offset: u16,
    pty: PtyManager,
    solution_file: PathBuf,
    test_output: Option<String>,
    show_results: bool,
    tip_system: tips::TipSystem,
}

enum Focus {
    Question,
    Editor,
    Results,
}

enum QuestionAction {
    ResetTemplate,
}

struct App {
    state: AppState,
    should_quit: bool,
    terminal_width: u16,
    terminal_height: u16,
    paths: AppPaths,
    progress: Progress,
}

impl App {
    fn new(terminal_width: u16, terminal_height: u16) -> Result<Self> {
        let paths = AppPaths::new()?;
        let client = LeetCodeClient::new();
        let problems = client.get_problems()?;

        // Load progress
        let progress = Progress::load(&paths.progress_file()).unwrap_or_default();

        // Extract unique categories
        let mut categories: Vec<String> = problems
            .iter()
            .map(|p| p.category.clone())
            .filter(|c| !c.is_empty())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        categories.sort();
        categories.insert(0, "All".to_string());

        let list_filters = vec![
            "NeetCode 150".to_string(),
            "Blind 75".to_string(),
        ];

        let difficulties = vec![
            "All".to_string(),
            "Easy".to_string(),
            "Medium".to_string(),
            "Hard".to_string(),
        ];

        let progress_filters = vec![
            "All".to_string(),
            "Not Started".to_string(),
            "Started".to_string(),
            "Completed".to_string(),
        ];

        let mut list_state = ListState::default();
        if !problems.is_empty() {
            list_state.select(Some(0));
        }

        let filtered_problems = problems.clone();

        Ok(App {
            state: AppState::Home(HomeState {
                all_problems: problems,
                filtered_problems,
                list_state,
                search_query: String::new(),
                list_filters,
                selected_list: 0,
                categories,
                selected_category: 0,
                difficulties,
                selected_difficulty: 0,
                progress_filters,
                selected_progress: 0,
                filter_focus: FilterFocus::ProblemList,
                selected_language: Language::default(),
            }),
            should_quit: false,
            terminal_width,
            terminal_height,
            paths,
            progress,
        })
    }

    fn open_question(&mut self, problem: Problem, language: Language) -> Result<()> {
        let client = LeetCodeClient::new();
        let problem_text = client.format_problem(&problem);

        // Use XDG-compliant path for solutions with language extension
        let solution_file = self.paths.solution_file(problem.id, language);

        // Only generate boilerplate if file doesn't exist (don't overwrite user's work!)
        if !solution_file.exists() {
            let boilerplate = client.generate_boilerplate(&problem, language);
            fs::write(&solution_file, &boilerplate)?;
        }

        // Calculate editor size (2/3 of width, accounting for borders)
        let editor_cols = ((self.terminal_width as f32 * 0.62) as u16).saturating_sub(2);
        let editor_rows = self.terminal_height.saturating_sub(2);

        let pty = PtyManager::new(editor_rows, editor_cols, solution_file.clone())?;

        self.state = AppState::Question(QuestionState {
            problem,
            problem_text,
            language,
            focus: Focus::Editor,
            scroll_offset: 0,
            results_scroll_offset: 0,
            pty,
            solution_file,
            test_output: None,
            show_results: false,
            tip_system: tips::TipSystem::new(),
        });

        Ok(())
    }

    fn reset_template(&mut self) -> Result<()> {
        if let AppState::Question(question) = &mut self.state {
            let client = LeetCodeClient::new();
            let boilerplate = client.generate_boilerplate(&question.problem, question.language);
            fs::write(&question.solution_file, &boilerplate)?;

            // Send :e! to reload the file in the editor (works for vim/neovim)
            question.pty.send_key(b"\x1b")?; // Escape to normal mode
            question.pty.send_key(b":e!\r")?; // Reload file
        }
        Ok(())
    }

    fn run_tests(solution_file: &PathBuf, problem: &Problem, lang: Language) -> Result<String> {
        Self::run_tests_with_mode(solution_file, problem, lang, TestMode::Run)
    }

    fn run_tests_with_mode(solution_file: &PathBuf, problem: &Problem, lang: Language, mode: TestMode) -> Result<String> {
        // Read user's solution
        let solution_code = fs::read_to_string(solution_file)?;

        // Generate test runner with the solution embedded
        let client = LeetCodeClient::new();
        let test_code = client.generate_test_runner_with_mode(problem, &solution_code, lang, mode);

        match lang {
            Language::JavaScript => Self::run_js_tests(solution_file, &test_code),
            Language::Python => Self::run_python_tests(solution_file, &test_code),
            Language::C => Self::run_c_tests(solution_file, &test_code),
            Language::Cpp => Self::run_cpp_tests(solution_file, &test_code),
        }
    }

    fn run_js_tests(solution_file: &PathBuf, test_code: &str) -> Result<String> {
        let temp_file = solution_file.with_extension("test.js");
        fs::write(&temp_file, test_code)?;

        // Run tests with Bun (faster) or fallback to Node
        let output = Command::new("bun")
            .arg("run")
            .arg(&temp_file)
            .output()
            .or_else(|_| Command::new("node").arg(&temp_file).output())?;

        let _ = fs::remove_file(&temp_file);

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() {
            Ok(format!("{}\n\nErrors:\n{}", stdout, stderr))
        } else {
            Ok(stdout.to_string())
        }
    }

    fn run_python_tests(solution_file: &PathBuf, test_code: &str) -> Result<String> {
        let temp_file = solution_file.with_extension("test.py");
        fs::write(&temp_file, test_code)?;

        let output = Command::new("python3")
            .arg(&temp_file)
            .output()
            .or_else(|_| Command::new("python").arg(&temp_file).output())?;

        let _ = fs::remove_file(&temp_file);

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() {
            Ok(format!("{}\n\nErrors:\n{}", stdout, stderr))
        } else {
            Ok(stdout.to_string())
        }
    }

    fn run_c_tests(solution_file: &PathBuf, test_code: &str) -> Result<String> {
        let temp_file = solution_file.with_extension("test.c");
        let binary_file = solution_file.with_extension("test_bin");

        fs::write(&temp_file, test_code)?;

        // Compile
        let compile_output = Command::new("gcc")
            .arg("-O2")
            .arg("-o")
            .arg(&binary_file)
            .arg(&temp_file)
            .arg("-lm")
            .output()?;

        if !compile_output.status.success() {
            let _ = fs::remove_file(&temp_file);
            let stderr = String::from_utf8_lossy(&compile_output.stderr);
            return Ok(format!("Compilation Error:\n{}", stderr));
        }

        // Run
        let run_output = Command::new(&binary_file).output()?;

        // Cleanup
        let _ = fs::remove_file(&temp_file);
        let _ = fs::remove_file(&binary_file);

        let stdout = String::from_utf8_lossy(&run_output.stdout);
        let stderr = String::from_utf8_lossy(&run_output.stderr);

        if !stderr.is_empty() {
            Ok(format!("{}\n\nRuntime Errors:\n{}", stdout, stderr))
        } else {
            Ok(stdout.to_string())
        }
    }

    fn run_cpp_tests(solution_file: &PathBuf, test_code: &str) -> Result<String> {
        let temp_file = solution_file.with_extension("test.cpp");
        let binary_file = solution_file.with_extension("test_bin");

        fs::write(&temp_file, test_code)?;

        // Compile with C++17
        let compile_output = Command::new("g++")
            .arg("-std=c++17")
            .arg("-O2")
            .arg("-o")
            .arg(&binary_file)
            .arg(&temp_file)
            .output()?;

        if !compile_output.status.success() {
            let _ = fs::remove_file(&temp_file);
            let stderr = String::from_utf8_lossy(&compile_output.stderr);
            return Ok(format!("Compilation Error:\n{}", stderr));
        }

        // Run
        let run_output = Command::new(&binary_file).output()?;

        // Cleanup
        let _ = fs::remove_file(&temp_file);
        let _ = fs::remove_file(&binary_file);

        let stdout = String::from_utf8_lossy(&run_output.stdout);
        let stderr = String::from_utf8_lossy(&run_output.stderr);

        if !stderr.is_empty() {
            Ok(format!("{}\n\nRuntime Errors:\n{}", stdout, stderr))
        } else {
            Ok(stdout.to_string())
        }
    }

    fn back_to_home(&mut self) -> Result<()> {
        if let AppState::Question(question) = &self.state {
            // Preserve the language selection
            let selected_language = question.language;

            // Get problems list again
            let client = LeetCodeClient::new();
            let problems = client.get_problems()?;

            // Extract unique categories
            let mut categories: Vec<String> = problems
                .iter()
                .map(|p| p.category.clone())
                .filter(|c| !c.is_empty())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();
            categories.sort();
            categories.insert(0, "All".to_string());

            let list_filters = vec![
                "NeetCode 150".to_string(),
                "Blind 75".to_string(),
            ];

            let difficulties = vec![
                "All".to_string(),
                "Easy".to_string(),
                "Medium".to_string(),
                "Hard".to_string(),
            ];

            let progress_filters = vec![
                "All".to_string(),
                "Not Started".to_string(),
                "Started".to_string(),
                "Completed".to_string(),
            ];

            let mut list_state = ListState::default();
            if !problems.is_empty() {
                list_state.select(Some(0));
            }

            let filtered_problems = problems.clone();

            self.state = AppState::Home(HomeState {
                all_problems: problems,
                filtered_problems,
                list_state,
                search_query: String::new(),
                list_filters,
                selected_list: 0,
                categories,
                selected_category: 0,
                difficulties,
                selected_difficulty: 0,
                progress_filters,
                selected_progress: 0,
                filter_focus: FilterFocus::ProblemList,
                selected_language,
            });
        }
        Ok(())
    }

    fn handle_input(&mut self, event: Event) -> Result<()> {
        // Handle global hotkeys
        if let Event::Key(key) = &event {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                match key.code {
                    KeyCode::Char('c') => {
                        self.should_quit = true;
                        return Ok(());
                    }
                    KeyCode::Char('h') => {
                        if matches!(self.state, AppState::Question(_)) {
                            self.back_to_home()?;
                            return Ok(());
                        }
                    }
                    KeyCode::Char('r') => {
                        // Run tests (quick mode - 3-5 tests)
                        if let AppState::Question(question) = &mut self.state {
                            // First, save the file in Neovim
                            // Send Escape to ensure normal mode, then :w to save
                            question.pty.send_key(b"\x1b:w\r")?;

                            // Give Neovim a moment to save the file
                            std::thread::sleep(Duration::from_millis(100));

                            // Mark as started when running tests
                            self.progress.mark_started(question.problem.id);

                            let output = Self::run_tests(&question.solution_file, &question.problem, question.language)?;

                            // Check if all tests passed and mark as completed
                            if output.contains("All tests passed") {
                                self.progress.mark_completed(question.problem.id);
                            }

                            // Save progress
                            let _ = self.progress.save(&self.paths.progress_file());

                            question.test_output = Some(output);
                            question.results_scroll_offset = 0;
                            question.show_results = true;
                            question.focus = Focus::Results;
                            return Ok(());
                        }
                    }
                    KeyCode::Char('s') => {
                        // Submit tests (full mode - 50-200 tests)
                        if let AppState::Question(question) = &mut self.state {
                            // First, save the file in Neovim
                            question.pty.send_key(b"\x1b:w\r")?;

                            // Give Neovim a moment to save the file
                            std::thread::sleep(Duration::from_millis(100));

                            // Mark as started when running tests
                            self.progress.mark_started(question.problem.id);

                            let output = Self::run_tests_with_mode(&question.solution_file, &question.problem, question.language, TestMode::Submit)?;

                            // Check if all tests passed and mark as completed
                            if output.contains("All tests passed") {
                                self.progress.mark_completed(question.problem.id);
                            }

                            // Save progress
                            let _ = self.progress.save(&self.paths.progress_file());

                            question.test_output = Some(output);
                            question.results_scroll_offset = 0;
                            question.show_results = true;
                            question.focus = Focus::Results;
                            return Ok(());
                        }
                    }
                    _ => {}
                }
            }
        }

        // Handle state-specific input
        let should_open_question = if let AppState::Home(home) = &mut self.state {
            Self::handle_home_input_internal(home, &self.progress, &event)?
                .map(|problem| (problem, home.selected_language))
        } else {
            None
        };

        if let Some((problem, language)) = should_open_question {
            self.open_question(problem, language)?;
            return Ok(());
        }

        if let AppState::Question(question) = &mut self.state {
            if let Some(action) = Self::handle_question_input_internal(question, &event, &mut self.terminal_width, &mut self.terminal_height)? {
                match action {
                    QuestionAction::ResetTemplate => {
                        self.reset_template()?;
                    }
                }
            }
        }

        Ok(())
    }

    fn apply_filters(home: &mut HomeState, progress: &Progress) {
        // First, update categories based on list filter (NeetCode 150 / Blind 75)
        let list_filtered_problems: Vec<_> = home.all_problems.iter()
            .filter(|p| {
                match home.selected_list {
                    1 => p.blind75, // Blind 75 only
                    _ => true,      // NeetCode 150 (all)
                }
            })
            .collect();

        // Extract categories from the list-filtered problems
        let mut new_categories: Vec<String> = list_filtered_problems
            .iter()
            .map(|p| p.category.clone())
            .filter(|c| !c.is_empty())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        new_categories.sort();
        new_categories.insert(0, "All".to_string());

        // If current category is no longer valid, reset to "All"
        let current_category = home.categories.get(home.selected_category).cloned();
        home.categories = new_categories;
        if let Some(cat) = current_category {
            if let Some(idx) = home.categories.iter().position(|c| c == &cat) {
                home.selected_category = idx;
            } else {
                home.selected_category = 0; // Reset to "All"
            }
        } else {
            home.selected_category = 0;
        }

        let category_filter = if home.selected_category == 0 {
            None
        } else {
            Some(&home.categories[home.selected_category])
        };

        let difficulty_filter = if home.selected_difficulty == 0 {
            None
        } else {
            Some(&home.difficulties[home.selected_difficulty])
        };

        let search_lower = home.search_query.to_lowercase();

        home.filtered_problems = home
            .all_problems
            .iter()
            .filter(|p| {
                // Category filter
                if let Some(cat) = category_filter {
                    if &p.category != cat {
                        return false;
                    }
                }

                // Difficulty filter
                if let Some(diff) = difficulty_filter {
                    if &p.difficulty != diff {
                        return false;
                    }
                }

                // List filter (NeetCode 150 / Blind 75)
                match home.selected_list {
                    0 => { // NeetCode 150 (all problems, no filter needed)
                    }
                    1 => { // Blind 75 only
                        if !p.blind75 {
                            return false;
                        }
                    }
                    _ => {}
                }

                // Progress filter
                if home.selected_progress != 0 {
                    let status = progress.get_status(p.id);
                    match home.selected_progress {
                        1 => { // Not Started
                            if status.is_some() {
                                return false;
                            }
                        }
                        2 => { // Started
                            if status != Some(ProblemStatus::Started) {
                                return false;
                            }
                        }
                        3 => { // Completed
                            if status != Some(ProblemStatus::Completed) {
                                return false;
                            }
                        }
                        _ => {}
                    }
                }

                // Search filter
                if !search_lower.is_empty() {
                    let title_lower = p.title.to_lowercase();
                    let id_str = p.id.to_string();
                    if !title_lower.contains(&search_lower) && !id_str.contains(&search_lower) {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect();

        // Reset selection
        if home.filtered_problems.is_empty() {
            home.list_state.select(None);
        } else {
            home.list_state.select(Some(0));
        }
    }

    fn handle_home_input_internal(home: &mut HomeState, progress: &Progress, event: &Event) -> Result<Option<Problem>> {
        if let Event::Key(key) = event {
            // Tab to cycle focus between filters
            if key.code == KeyCode::Tab {
                home.filter_focus = match home.filter_focus {
                    FilterFocus::ProblemList => FilterFocus::Search,
                    FilterFocus::Search => FilterFocus::ListFilter,
                    FilterFocus::ListFilter => FilterFocus::Category,
                    FilterFocus::Category => FilterFocus::Difficulty,
                    FilterFocus::Difficulty => FilterFocus::Progress,
                    FilterFocus::Progress => FilterFocus::ProblemList,
                };
                return Ok(None);
            }

            // Backtab (Shift+Tab) to cycle backwards
            if key.code == KeyCode::BackTab {
                home.filter_focus = match home.filter_focus {
                    FilterFocus::ProblemList => FilterFocus::Progress,
                    FilterFocus::Search => FilterFocus::ProblemList,
                    FilterFocus::ListFilter => FilterFocus::Search,
                    FilterFocus::Category => FilterFocus::ListFilter,
                    FilterFocus::Difficulty => FilterFocus::Category,
                    FilterFocus::Progress => FilterFocus::Difficulty,
                };
                return Ok(None);
            }

            match home.filter_focus {
                FilterFocus::ProblemList => {
                    match key.code {
                        KeyCode::Up => {
                            if !home.filtered_problems.is_empty() {
                                let i = match home.list_state.selected() {
                                    Some(i) => {
                                        if i == 0 {
                                            home.filtered_problems.len() - 1
                                        } else {
                                            i - 1
                                        }
                                    }
                                    None => 0,
                                };
                                home.list_state.select(Some(i));
                            }
                        }
                        KeyCode::Down => {
                            if !home.filtered_problems.is_empty() {
                                let i = match home.list_state.selected() {
                                    Some(i) => {
                                        if i >= home.filtered_problems.len() - 1 {
                                            0
                                        } else {
                                            i + 1
                                        }
                                    }
                                    None => 0,
                                };
                                home.list_state.select(Some(i));
                            }
                        }
                        KeyCode::Enter => {
                            if let Some(i) = home.list_state.selected() {
                                if i < home.filtered_problems.len() {
                                    let problem = home.filtered_problems[i].clone();
                                    return Ok(Some(problem));
                                }
                            }
                        }
                        KeyCode::Char('/') => {
                            // Quick shortcut to jump to search
                            home.filter_focus = FilterFocus::Search;
                        }
                        KeyCode::Char('l') | KeyCode::Char('L') => {
                            // Cycle to next language
                            home.selected_language = home.selected_language.next();
                        }
                        _ => {}
                    }
                }
                FilterFocus::Search => {
                    match key.code {
                        KeyCode::Char(c) => {
                            home.search_query.push(c);
                            Self::apply_filters(home, progress);
                        }
                        KeyCode::Backspace => {
                            home.search_query.pop();
                            Self::apply_filters(home, progress);
                        }
                        KeyCode::Esc => {
                            home.search_query.clear();
                            Self::apply_filters(home, progress);
                            home.filter_focus = FilterFocus::ProblemList;
                        }
                        KeyCode::Enter => {
                            home.filter_focus = FilterFocus::ProblemList;
                        }
                        _ => {}
                    }
                }
                FilterFocus::ListFilter => {
                    match key.code {
                        KeyCode::Left => {
                            let len = home.list_filters.len();
                            home.selected_list = (home.selected_list + len - 1) % len;
                            Self::apply_filters(home, progress);
                        }
                        KeyCode::Right => {
                            home.selected_list = (home.selected_list + 1) % home.list_filters.len();
                            Self::apply_filters(home, progress);
                        }
                        KeyCode::Enter => {
                            home.filter_focus = FilterFocus::ProblemList;
                        }
                        _ => {}
                    }
                }
                FilterFocus::Category => {
                    match key.code {
                        KeyCode::Left => {
                            let len = home.categories.len();
                            home.selected_category = (home.selected_category + len - 1) % len;
                            Self::apply_filters(home, progress);
                        }
                        KeyCode::Right => {
                            home.selected_category = (home.selected_category + 1) % home.categories.len();
                            Self::apply_filters(home, progress);
                        }
                        KeyCode::Enter => {
                            home.filter_focus = FilterFocus::ProblemList;
                        }
                        _ => {}
                    }
                }
                FilterFocus::Difficulty => {
                    match key.code {
                        KeyCode::Left => {
                            let len = home.difficulties.len();
                            home.selected_difficulty = (home.selected_difficulty + len - 1) % len;
                            Self::apply_filters(home, progress);
                        }
                        KeyCode::Right => {
                            home.selected_difficulty = (home.selected_difficulty + 1) % home.difficulties.len();
                            Self::apply_filters(home, progress);
                        }
                        KeyCode::Enter => {
                            home.filter_focus = FilterFocus::ProblemList;
                        }
                        _ => {}
                    }
                }
                FilterFocus::Progress => {
                    match key.code {
                        KeyCode::Left => {
                            let len = home.progress_filters.len();
                            home.selected_progress = (home.selected_progress + len - 1) % len;
                            Self::apply_filters(home, progress);
                        }
                        KeyCode::Right => {
                            home.selected_progress = (home.selected_progress + 1) % home.progress_filters.len();
                            Self::apply_filters(home, progress);
                        }
                        KeyCode::Enter => {
                            home.filter_focus = FilterFocus::ProblemList;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(None)
    }

    fn handle_question_input_internal(
        question: &mut QuestionState,
        event: &Event,
        terminal_width: &mut u16,
        terminal_height: &mut u16,
    ) -> Result<Option<QuestionAction>> {
        match event {
            Event::Key(key) => {
                // Escape to close results panel
                if key.code == KeyCode::Esc && question.show_results {
                    question.show_results = false;
                    question.focus = Focus::Editor;
                    return Ok(None);
                }

                // Ctrl+Q to switch focus (only between Question and Editor)
                if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('q') {
                    if question.show_results {
                        question.show_results = false;
                    }
                    question.focus = match question.focus {
                        Focus::Question => Focus::Editor,
                        Focus::Editor => Focus::Question,
                        Focus::Results => Focus::Editor,
                    };
                    return Ok(None);
                }

                // Handle input based on focus
                match question.focus {
                    Focus::Editor => {
                        if let Some(bytes) = key_to_bytes(&key) {
                            question.pty.send_key(&bytes)?;
                        }
                    }
                    Focus::Results => {
                        // Handle scrolling in results panel
                        match key.code {
                            KeyCode::Up | KeyCode::Char('k') => {
                                if question.results_scroll_offset > 0 {
                                    question.results_scroll_offset -= 1;
                                }
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                question.results_scroll_offset += 1;
                            }
                            KeyCode::PageUp => {
                                question.results_scroll_offset = question.results_scroll_offset.saturating_sub(10);
                            }
                            KeyCode::PageDown => {
                                question.results_scroll_offset += 10;
                            }
                            KeyCode::Home | KeyCode::Char('g') => {
                                question.results_scroll_offset = 0;
                            }
                            KeyCode::End | KeyCode::Char('G') => {
                                // Scroll to end - will be clamped in render
                                question.results_scroll_offset = u16::MAX;
                            }
                            _ => {}
                        }
                    }
                    Focus::Question => {
                        match key.code {
                            KeyCode::Up => {
                                if question.scroll_offset > 0 {
                                    question.scroll_offset -= 1;
                                }
                            }
                            KeyCode::Down => {
                                question.scroll_offset += 1;
                            }
                            KeyCode::PageUp => {
                                question.scroll_offset = question.scroll_offset.saturating_sub(10);
                            }
                            KeyCode::PageDown => {
                                question.scroll_offset += 10;
                            }
                            KeyCode::Home => {
                                question.scroll_offset = 0;
                            }
                            // Shift+R to reset template (regenerate boilerplate)
                            KeyCode::Char('R') => {
                                return Ok(Some(QuestionAction::ResetTemplate));
                            }
                            _ => {}
                        }
                    }
                }
            }
            Event::Resize(cols, rows) => {
                *terminal_width = *cols;
                *terminal_height = *rows;

                // Recalculate editor size (2/3 of width)
                let editor_cols = (*cols as f32 * 0.62) as u16;
                question.pty.resize(rows.saturating_sub(2), editor_cols.saturating_sub(2))?;
            }
            _ => {}
        }

        Ok(None)
    }

    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        match &mut self.state {
            AppState::Home(home) => Self::render_home(terminal, home, &self.progress),
            AppState::Question(question) => Self::render_question(terminal, question),
        }
    }

    fn render_home(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, home: &mut HomeState, progress: &Progress) -> Result<()> {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),  // Title
                    Constraint::Length(3),  // Filter bar row 1
                    Constraint::Length(3),  // Filter bar row 2
                    Constraint::Min(0),     // Problem list
                    Constraint::Length(2),  // Help text
                ])
                .split(f.area());

            // Title with count and progress stats
            let completed = progress.count_by_status(ProblemStatus::Completed);
            let started = progress.count_by_status(ProblemStatus::Started);
            let list_name = home.list_filters.get(home.selected_list).map(|s| s.as_str()).unwrap_or("NeetCode 150");
            let title = Paragraph::new(format!(
                "leetTUI {} - {} of {} | Completed: {} | In Progress: {}",
                list_name,
                home.filtered_problems.len(),
                home.all_problems.len(),
                completed,
                started
            ))
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
            f.render_widget(title, chunks[0]);

            // Filter bar row 1: Search and List
            let filter_row1 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(50),  // Search
                    Constraint::Percentage(50),  // List
                ])
                .split(chunks[1]);

            // Search box
            let search_style = if home.filter_focus == FilterFocus::Search {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let search_text = if home.search_query.is_empty() && home.filter_focus != FilterFocus::Search {
                "/ to search...".to_string()
            } else {
                format!("{}_", home.search_query)
            };
            let search_block = Block::default()
                .borders(Borders::ALL)
                .border_style(search_style)
                .title(Span::styled(" Search ", search_style));
            let search = Paragraph::new(search_text).block(search_block);
            f.render_widget(search, filter_row1[0]);

            // List selector (Blind 75 / NeetCode 150)
            let list_style = if home.filter_focus == FilterFocus::ListFilter {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let default_list = "All".to_string();
            let list_name = home.list_filters.get(home.selected_list).unwrap_or(&default_list);
            let list_block = Block::default()
                .borders(Borders::ALL)
                .border_style(list_style)
                .title(Span::styled(" List ", list_style));
            let list_widget = Paragraph::new(format!("< {} >", list_name))
                .block(list_block)
                .alignment(Alignment::Center);
            f.render_widget(list_widget, filter_row1[1]);

            // Filter bar row 2: Category, Difficulty, Progress
            let filter_row2 = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(34),  // Category
                    Constraint::Percentage(33),  // Difficulty
                    Constraint::Percentage(33),  // Progress
                ])
                .split(chunks[2]);

            // Category selector
            let cat_style = if home.filter_focus == FilterFocus::Category {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let default_cat = "All".to_string();
            let cat_text = format!(
                "< {} >",
                home.categories.get(home.selected_category).unwrap_or(&default_cat)
            );
            let cat_block = Block::default()
                .borders(Borders::ALL)
                .border_style(cat_style)
                .title(Span::styled(" Category ", cat_style));
            let category = Paragraph::new(cat_text)
                .block(cat_block)
                .alignment(Alignment::Center);
            f.render_widget(category, filter_row2[0]);

            // Difficulty selector
            let diff_style = if home.filter_focus == FilterFocus::Difficulty {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let default_diff = "All".to_string();
            let diff_name = home.difficulties.get(home.selected_difficulty).unwrap_or(&default_diff);
            let diff_color = match diff_name.as_str() {
                "Easy" => Color::Green,
                "Medium" => Color::Yellow,
                "Hard" => Color::Red,
                _ => Color::White,
            };
            let diff_block = Block::default()
                .borders(Borders::ALL)
                .border_style(diff_style)
                .title(Span::styled(" Difficulty ", diff_style));
            let difficulty = Paragraph::new(Line::from(vec![
                Span::raw("< "),
                Span::styled(diff_name, Style::default().fg(diff_color)),
                Span::raw(" >"),
            ]))
            .block(diff_block)
            .alignment(Alignment::Center);
            f.render_widget(difficulty, filter_row2[1]);

            // Progress selector
            let prog_style = if home.filter_focus == FilterFocus::Progress {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            let default_prog = "All".to_string();
            let prog_name = home.progress_filters.get(home.selected_progress).unwrap_or(&default_prog);
            let prog_color = match prog_name.as_str() {
                "Completed" => Color::Green,
                "Started" => Color::Yellow,
                "Not Started" => Color::DarkGray,
                _ => Color::White,
            };
            let prog_block = Block::default()
                .borders(Borders::ALL)
                .border_style(prog_style)
                .title(Span::styled(" Progress ", prog_style));
            let prog_widget = Paragraph::new(Line::from(vec![
                Span::raw("< "),
                Span::styled(prog_name, Style::default().fg(prog_color)),
                Span::raw(" >"),
            ]))
            .block(prog_block)
            .alignment(Alignment::Center);
            f.render_widget(prog_widget, filter_row2[2]);

            // Problem list
            let list_style = if home.filter_focus == FilterFocus::ProblemList {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::White)
            };

            let items: Vec<ListItem> = home
                .filtered_problems
                .iter()
                .map(|p| {
                    let difficulty_color = match p.difficulty.as_str() {
                        "Easy" => Color::Green,
                        "Medium" => Color::Yellow,
                        "Hard" => Color::Red,
                        _ => Color::White,
                    };

                    // Progress indicator
                    let (status_icon, status_color) = match progress.get_status(p.id) {
                        Some(ProblemStatus::Completed) => ("[x]", Color::Green),
                        Some(ProblemStatus::Started) => ("[-]", Color::Yellow),
                        None => ("[ ]", Color::DarkGray),
                    };

                    let cat_display = if p.category.is_empty() {
                        String::new()
                    } else {
                        format!(" ({})", p.category)
                    };

                    let content = Line::from(vec![
                        Span::styled(
                            format!("{:>2}. ", p.position),
                            Style::default().fg(Color::Rgb(120, 140, 170)),
                        ),
                        Span::styled(
                            format!("{} ", status_icon),
                            Style::default().fg(status_color),
                        ),
                        Span::raw(&p.title),
                        Span::styled(cat_display, Style::default().fg(Color::Rgb(120, 140, 170))),
                        Span::raw(" "),
                        Span::styled(
                            format!("[{}]", p.difficulty),
                            Style::default().fg(difficulty_color),
                        ),
                    ]);
                    ListItem::new(content)
                })
                .collect();

            let list_title = if home.filter_focus == FilterFocus::ProblemList {
                " Problems (Focused) "
            } else {
                " Problems "
            };

            let list = List::new(items)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(list_style)
                    .title(Span::styled(list_title, list_style)))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[3], &mut home.list_state);

            // Help text with language indicator
            let lang_color = match home.selected_language {
                crate::language::Language::JavaScript => Color::Yellow,
                crate::language::Language::Python => Color::Blue,
                crate::language::Language::C => Color::Magenta,
                crate::language::Language::Cpp => Color::Cyan,
            };
            let help = Paragraph::new(Line::from(vec![
                Span::styled(
                    format!("[{}] ", home.selected_language.short_name()),
                    Style::default().fg(lang_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "L: Language | Tab: Focus | ↑↓: Nav | ←→: Filter | /: Search | Enter: Select | Ctrl+C: Quit",
                    Style::default().fg(Color::Rgb(120, 140, 170)),
                ),
            ]))
            .alignment(Alignment::Center);
            f.render_widget(help, chunks[4]);
        })?;

        Ok(())
    }

    fn render_question(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, question: &mut QuestionState) -> Result<()> {
        // Update tip system each frame
        question.tip_system.update();

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(38), Constraint::Percentage(62)])
                .split(f.area());

            // Split question pane into main area and tip area
            let question_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(4)])
                .split(chunks[0]);

            // Render Question pane
            let question_focused = matches!(question.focus, Focus::Question);
            let question_block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    if question_focused {
                        " Question (Focused) "
                    } else {
                        " Question "
                    },
                    Style::default()
                        .fg(if question_focused {
                            Color::Cyan
                        } else {
                            Color::White
                        })
                        .add_modifier(if question_focused {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        }),
                ));

            let question_text: Vec<Line> = question
                .problem_text
                .lines()
                .skip(question.scroll_offset as usize)
                .map(|line| {
                    if line.starts_with("Problem ") {
                        // Problem title - bright cyan, bold
                        Line::from(Span::styled(
                            line.to_string(),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                    } else if line.starts_with("Difficulty:") {
                        // Difficulty - colored by level
                        let color = if line.contains("Easy") {
                            Color::Green
                        } else if line.contains("Medium") {
                            Color::Yellow
                        } else if line.contains("Hard") {
                            Color::Red
                        } else {
                            Color::White
                        };
                        Line::from(Span::styled(
                            line.to_string(),
                            Style::default()
                                .fg(color)
                                .add_modifier(Modifier::BOLD),
                        ))
                    } else if line.starts_with("Example ") {
                        // Example headers - ochre yellow
                        Line::from(Span::styled(
                            line.to_string(),
                            Style::default()
                                .fg(Color::Rgb(204, 136, 34))
                                .add_modifier(Modifier::BOLD),
                        ))
                    } else if line.starts_with("Keyboard Shortcuts:") {
                        // Keyboard shortcuts header
                        Line::from(Span::styled(
                            line.to_string(),
                            Style::default()
                                .fg(Color::Yellow),
                        ))
                    } else {
                        Line::from(line.to_string())
                    }
                })
                .collect();

            let question_paragraph = Paragraph::new(question_text)
                .block(question_block)
                .wrap(Wrap { trim: false });

            f.render_widget(question_paragraph, question_chunks[0]);

            // Render tip with fade effect
            let opacity = question.tip_system.opacity();
            let tip_text = question.tip_system.current_tip();

            // Fade from background color (dark) to visible color (light cyan-ish)
            // At opacity 0: blend with dark background (~30-40)
            // At opacity 1: bright visible text (~160-200)
            let bg_base = 35.0_f32;
            let fg_r = 160.0_f32;
            let fg_g = 180.0_f32;
            let fg_b = 200.0_f32;

            let r = (bg_base + opacity * (fg_r - bg_base)) as u8;
            let g = (bg_base + opacity * (fg_g - bg_base)) as u8;
            let b = (bg_base + opacity * (fg_b - bg_base)) as u8;
            let tip_color = Color::Rgb(r, g, b);

            let tip_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(100, 110, 130)))
                .title(Span::styled(" Tip ", Style::default().fg(Color::Rgb(140, 150, 170))));

            let tip_paragraph = Paragraph::new(Span::styled(
                tip_text,
                Style::default().fg(tip_color),
            ))
            .block(tip_block)
            .wrap(Wrap { trim: true });

            f.render_widget(tip_paragraph, question_chunks[1]);

            // Render Editor pane
            let editor_focused = matches!(question.focus, Focus::Editor);
            let editor_block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    if editor_focused {
                        " Neovim (Focused) - Ctrl+R: Run | Ctrl+S: Submit "
                    } else {
                        " Neovim "
                    },
                    Style::default()
                        .fg(if editor_focused {
                            Color::Cyan
                        } else {
                            Color::White
                        })
                        .add_modifier(if editor_focused {
                            Modifier::BOLD
                        } else {
                            Modifier::empty()
                        }),
                ));

            if let Ok(parser) = question.pty.parser.read() {
                let screen = tui_term::widget::PseudoTerminal::new(parser.screen())
                    .block(editor_block);
                f.render_widget(screen, chunks[1]);
            }

            // Render test results overlay if showing
            if question.show_results {
                let area = f.area();

                // Dim the background by rendering a dark overlay
                let dim_block = Block::default()
                    .style(Style::default().bg(Color::Rgb(0, 0, 0)));
                f.render_widget(dim_block, area);

                let popup_width = area.width.saturating_sub(10).min(100);
                let popup_height = area.height.saturating_sub(4);
                let popup_x = (area.width - popup_width) / 2;
                let popup_y = (area.height - popup_height) / 2;

                let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

                // Clear the area behind the popup
                f.render_widget(Clear, popup_area);

                // Render the results
                let results_text = question.test_output.as_deref().unwrap_or("No output");
                let total_lines = results_text.lines().count();
                let visible_lines = popup_height.saturating_sub(2) as usize; // Account for borders

                // Clamp scroll offset to valid range
                let max_scroll = total_lines.saturating_sub(visible_lines);
                let scroll_offset = (question.results_scroll_offset as usize).min(max_scroll);

                let results_lines: Vec<Line> = results_text
                    .lines()
                    .skip(scroll_offset)
                    .map(|line| {
                        if line.contains("PASSED") || line.contains("passed") {
                            Line::from(Span::styled(line, Style::default().fg(Color::Green)))
                        } else if line.contains("FAILED") || line.contains("failed") || line.contains("ERROR") {
                            Line::from(Span::styled(line, Style::default().fg(Color::Red)))
                        } else if line.contains("All tests passed") {
                            Line::from(Span::styled(line, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)))
                        } else {
                            Line::from(line.to_string())
                        }
                    })
                    .collect();

                // Build title with scroll indicator
                let scroll_info = if total_lines > visible_lines {
                    format!(" [{}/{}] ", scroll_offset + 1, total_lines.saturating_sub(visible_lines) + 1)
                } else {
                    String::new()
                };

                let results_block = Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(
                        format!(" Test Results{} (↑↓/jk: scroll, Esc: close) ", scroll_info),
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    ))
                    .style(Style::default().bg(Color::Rgb(30, 35, 40)));

                let results_paragraph = Paragraph::new(results_lines)
                    .block(results_block)
                    .wrap(Wrap { trim: false });

                f.render_widget(results_paragraph, popup_area);
            }
        })?;

        Ok(())
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Get terminal size
    let size = terminal.size()?;

    // Create app with proper size
    let mut app = App::new(size.width, size.height)?;

    // Main loop
    loop {
        app.render(&mut terminal)?;

        if app.should_quit {
            break;
        }

        if event::poll(Duration::from_millis(16))? {
            let event = event::read()?;
            app.handle_input(event)?;
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

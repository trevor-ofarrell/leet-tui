mod config;
mod input;
mod leetcode;
mod pty;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
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
use crate::leetcode::{LeetCodeClient, Problem};
use crate::pty::PtyManager;

enum AppState {
    Home(HomeState),
    Question(QuestionState),
}

struct HomeState {
    problems: Vec<Problem>,
    list_state: ListState,
}

struct QuestionState {
    problem: Problem,
    problem_text: String,
    focus: Focus,
    scroll_offset: u16,
    pty: PtyManager,
    solution_file: PathBuf,
    test_output: Option<String>,
    show_results: bool,
}

enum Focus {
    Question,
    Editor,
    Results,
}

struct App {
    state: AppState,
    should_quit: bool,
    terminal_width: u16,
    terminal_height: u16,
    paths: AppPaths,
}

impl App {
    fn new(terminal_width: u16, terminal_height: u16) -> Result<Self> {
        let paths = AppPaths::new()?;
        let client = LeetCodeClient::new();
        let problems = client.get_problems()?;

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Ok(App {
            state: AppState::Home(HomeState {
                problems,
                list_state,
            }),
            should_quit: false,
            terminal_width,
            terminal_height,
            paths,
        })
    }

    fn open_question(&mut self, problem: Problem) -> Result<()> {
        let client = LeetCodeClient::new();
        let problem_text = client.format_problem(&problem);

        // Use XDG-compliant path for solutions
        let solution_file = self.paths.solution_file(problem.id);

        // Only generate boilerplate if file doesn't exist (don't overwrite user's work!)
        if !solution_file.exists() {
            let boilerplate = client.generate_boilerplate(&problem);
            fs::write(&solution_file, &boilerplate)?;
        }

        // Calculate editor size (2/3 of width, accounting for borders)
        let editor_cols = ((self.terminal_width as f32 * 0.67) as u16).saturating_sub(2);
        let editor_rows = self.terminal_height.saturating_sub(2);

        let pty = PtyManager::new(editor_rows, editor_cols, solution_file.clone())?;

        self.state = AppState::Question(QuestionState {
            problem,
            problem_text,
            focus: Focus::Editor,
            scroll_offset: 0,
            pty,
            solution_file,
            test_output: None,
            show_results: false,
        });

        Ok(())
    }

    fn run_tests(solution_file: &PathBuf, problem: &Problem) -> Result<String> {
        // Read user's solution
        let solution_code = fs::read_to_string(solution_file)?;

        // Generate test runner with the solution embedded
        let client = LeetCodeClient::new();
        let test_code = client.generate_test_runner(problem, &solution_code);

        // Write to temp file
        let temp_file = solution_file.with_extension("test.js");
        fs::write(&temp_file, &test_code)?;

        // Run tests with Bun (faster) or fallback to Node
        let output = Command::new("bun")
            .arg("run")
            .arg(&temp_file)
            .output()
            .or_else(|_| {
                Command::new("node")
                    .arg(&temp_file)
                    .output()
            })?;

        // Clean up temp file
        let _ = fs::remove_file(&temp_file);

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stderr.is_empty() {
            Ok(format!("{}\n\nErrors:\n{}", stdout, stderr))
        } else {
            Ok(stdout.to_string())
        }
    }

    fn back_to_home(&mut self) -> Result<()> {
        if let AppState::Question(_) = &self.state {
            // Get problems list again
            let client = LeetCodeClient::new();
            let problems = client.get_problems()?;

            let mut list_state = ListState::default();
            list_state.select(Some(0));

            self.state = AppState::Home(HomeState {
                problems,
                list_state,
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
                        // Run tests
                        if let AppState::Question(question) = &mut self.state {
                            // First, save the file in Neovim
                            // Send Escape to ensure normal mode, then :w to save
                            question.pty.send_key(b"\x1b:w\r")?;

                            // Give Neovim a moment to save the file
                            std::thread::sleep(Duration::from_millis(100));

                            let output = Self::run_tests(&question.solution_file, &question.problem)?;
                            question.test_output = Some(output);
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
            Self::handle_home_input_internal(home, &event)?
        } else {
            None
        };

        if let Some(problem) = should_open_question {
            self.open_question(problem)?;
            return Ok(());
        }

        if let AppState::Question(question) = &mut self.state {
            Self::handle_question_input_internal(question, &event, &mut self.terminal_width, &mut self.terminal_height)?;
        }

        Ok(())
    }

    fn handle_home_input_internal(home: &mut HomeState, event: &Event) -> Result<Option<Problem>> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Up => {
                    let i = match home.list_state.selected() {
                        Some(i) => {
                            if i == 0 {
                                home.problems.len() - 1
                            } else {
                                i - 1
                            }
                        }
                        None => 0,
                    };
                    home.list_state.select(Some(i));
                }
                KeyCode::Down => {
                    let i = match home.list_state.selected() {
                        Some(i) => {
                            if i >= home.problems.len() - 1 {
                                0
                            } else {
                                i + 1
                            }
                        }
                        None => 0,
                    };
                    home.list_state.select(Some(i));
                }
                KeyCode::Enter => {
                    if let Some(i) = home.list_state.selected() {
                        let problem = home.problems[i].clone();
                        return Ok(Some(problem));
                    }
                }
                _ => {}
            }
        }
        Ok(None)
    }

    fn handle_question_input_internal(
        question: &mut QuestionState,
        event: &Event,
        terminal_width: &mut u16,
        terminal_height: &mut u16,
    ) -> Result<()> {
        match event {
            Event::Key(key) => {
                // Escape to close results panel
                if key.code == KeyCode::Esc && question.show_results {
                    question.show_results = false;
                    question.focus = Focus::Editor;
                    return Ok(());
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
                    return Ok(());
                }

                // Handle input based on focus
                match question.focus {
                    Focus::Editor => {
                        if let Some(bytes) = key_to_bytes(&key) {
                            question.pty.send_key(&bytes)?;
                        }
                    }
                    Focus::Results => {
                        // Results panel just shows output, Esc closes it
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
                            _ => {}
                        }
                    }
                }
            }
            Event::Resize(cols, rows) => {
                *terminal_width = *cols;
                *terminal_height = *rows;

                // Recalculate editor size (2/3 of width)
                let editor_cols = (*cols as f32 * 0.67) as u16;
                question.pty.resize(rows.saturating_sub(2), editor_cols.saturating_sub(2))?;
            }
            _ => {}
        }

        Ok(())
    }

    fn render(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        match &mut self.state {
            AppState::Home(home) => Self::render_home(terminal, home),
            AppState::Question(question) => Self::render_question(terminal, question),
        }
    }

    fn render_home(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, home: &mut HomeState) -> Result<()> {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(f.area());

            // Title
            let title = Paragraph::new("LeetCode TUI - Problem List")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .alignment(Alignment::Center);
            f.render_widget(title, chunks[0]);

            // Problem list
            let items: Vec<ListItem> = home
                .problems
                .iter()
                .map(|p| {
                    let difficulty_color = match p.difficulty.as_str() {
                        "Easy" => Color::Green,
                        "Medium" => Color::Yellow,
                        "Hard" => Color::Red,
                        _ => Color::White,
                    };

                    let content = Line::from(vec![
                        Span::styled(
                            format!("{:>4}. ", p.id),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Span::raw(&p.title),
                        Span::raw(" "),
                        Span::styled(
                            format!("[{}]", p.difficulty),
                            Style::default().fg(difficulty_color),
                        ),
                    ]);
                    ListItem::new(content)
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL).title(" Problems "))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, chunks[1], &mut home.list_state);

            // Help text
            let help = Paragraph::new("↑/↓: Navigate | Enter: Select | Ctrl+C: Quit")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            f.render_widget(help, chunks[2]);
        })?;

        Ok(())
    }

    fn render_question(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, question: &mut QuestionState) -> Result<()> {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(33), Constraint::Percentage(67)])
                .split(f.area());

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
                        // Example headers - bright magenta
                        Line::from(Span::styled(
                            line.to_string(),
                            Style::default()
                                .fg(Color::Magenta)
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

            f.render_widget(question_paragraph, chunks[0]);

            // Render Editor pane
            let editor_focused = matches!(question.focus, Focus::Editor);
            let editor_block = Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    if editor_focused {
                        " Neovim (Focused) - Ctrl+R: Run Tests "
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
                let popup_width = area.width.saturating_sub(10).min(80);
                let popup_height = area.height.saturating_sub(6).min(50);
                let popup_x = (area.width - popup_width) / 2;
                let popup_y = (area.height - popup_height) / 2;

                let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

                // Clear the area behind the popup
                f.render_widget(Clear, popup_area);

                // Render the results
                let results_text = question.test_output.as_deref().unwrap_or("No output");

                let results_lines: Vec<Line> = results_text
                    .lines()
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

                let results_block = Block::default()
                    .borders(Borders::ALL)
                    .title(Span::styled(
                        " Test Results (Esc to close) ",
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

    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
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
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

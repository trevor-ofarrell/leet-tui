use anyhow::Result;
use std::fs;
use std::path::PathBuf;

use crate::language::Language;

/// Application paths following XDG Base Directory spec
pub struct AppPaths {
    /// Config directory (~/.config/leet-tui/)
    pub config_dir: PathBuf,
    /// Data directory (~/.local/share/leet-tui/)
    pub data_dir: PathBuf,
    /// Cache directory (~/.cache/leet-tui/)
    pub cache_dir: PathBuf,
    /// Solutions directory (~/.local/share/leet-tui/solutions/)
    pub solutions_dir: PathBuf,
}

impl AppPaths {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("leet-tui");

        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("leet-tui");

        let cache_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("leet-tui");

        let solutions_dir = data_dir.join("solutions");

        let paths = AppPaths {
            config_dir,
            data_dir,
            cache_dir,
            solutions_dir,
        };

        // Create directories if they don't exist
        paths.ensure_dirs()?;

        Ok(paths)
    }

    /// Create all necessary directories
    pub fn ensure_dirs(&self) -> Result<()> {
        fs::create_dir_all(&self.config_dir)?;
        fs::create_dir_all(&self.data_dir)?;
        fs::create_dir_all(&self.cache_dir)?;
        fs::create_dir_all(&self.solutions_dir)?;
        Ok(())
    }

    /// Get path for a specific problem's solution file in a given language
    pub fn solution_file(&self, problem_id: u32, lang: Language) -> PathBuf {
        self.solutions_dir
            .join(format!("problem_{}.{}", problem_id, lang.extension()))
    }

    /// Check which languages have existing solutions for a problem
    pub fn get_existing_solutions(&self, problem_id: u32) -> Vec<Language> {
        Language::all()
            .iter()
            .filter(|lang| self.solution_file(problem_id, **lang).exists())
            .copied()
            .collect()
    }

    /// Get path for progress tracking file
    pub fn progress_file(&self) -> PathBuf {
        self.data_dir.join("progress.json")
    }
}

impl Default for AppPaths {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| AppPaths {
            config_dir: PathBuf::from(".config/leet-tui"),
            data_dir: PathBuf::from(".local/share/leet-tui"),
            cache_dir: PathBuf::from(".cache/leet-tui"),
            solutions_dir: PathBuf::from(".local/share/leet-tui/solutions"),
        })
    }
}

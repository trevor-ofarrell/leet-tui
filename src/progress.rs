use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProblemStatus {
    Started,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemProgress {
    pub status: ProblemStatus,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Progress {
    #[serde(flatten)]
    pub problems: HashMap<u32, ProblemProgress>,
}

impl Progress {
    pub fn load(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let progress: Progress = serde_json::from_str(&content)?;
            Ok(progress)
        } else {
            Ok(Progress::default())
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(&self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_status(&self, problem_id: u32) -> Option<ProblemStatus> {
        self.problems.get(&problem_id).map(|p| p.status)
    }

    pub fn mark_started(&mut self, problem_id: u32) {
        // Only mark as started if not already completed
        if let Some(existing) = self.problems.get(&problem_id) {
            if existing.status == ProblemStatus::Completed {
                return;
            }
        }
        self.problems.insert(problem_id, ProblemProgress {
            status: ProblemStatus::Started,
            last_updated: Utc::now(),
        });
    }

    pub fn mark_completed(&mut self, problem_id: u32) {
        self.problems.insert(problem_id, ProblemProgress {
            status: ProblemStatus::Completed,
            last_updated: Utc::now(),
        });
    }

    pub fn count_by_status(&self, status: ProblemStatus) -> usize {
        self.problems.values().filter(|p| p.status == status).count()
    }
}

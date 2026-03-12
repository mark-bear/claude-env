use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "associated_plan")]
    pub associated_plan: String,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "projects")]
pub struct Projects {
    #[serde(rename = "project", default)]
    pub projects: Vec<Project>,
}

impl Projects {
    pub fn new() -> Self {
        Self { projects: Vec::new() }
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub fn get_project(&self, path: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.path == path)
    }

    pub fn get_project_mut(&mut self, path: &str) -> Option<&mut Project> {
        self.projects.iter_mut().find(|p| p.path == path)
    }

    pub fn remove_project(&mut self, path: &str) -> Option<Project> {
        let pos = self.projects.iter().position(|p| p.path == path)?;
        Some(self.projects.remove(pos))
    }

    pub fn list_all(&self) -> &[Project] {
        &self.projects
    }
}

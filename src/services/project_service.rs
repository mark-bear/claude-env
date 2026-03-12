use anyhow::{Context, Result};
use chrono::Utc;
use uuid::Uuid;
use crate::models::{Project, Projects};
use crate::storage::{
    get_projects_path,
    load_xml,
    save_xml,
};

pub struct ProjectService;

impl ProjectService {
    pub fn associate_project(path: &str, plan_id: &str, name: Option<String>) -> Result<Project> {
        let mut projects = Self::load_all()?;

        // Check if project already exists at this path
        if projects.get_project(path).is_some() {
            anyhow::bail!("Project already associated with path: {}", path);
        }

        let project_name = name.unwrap_or_else(|| {
            // Extract project name from path
            path.split('/')
                .last()
                .unwrap_or("unnamed")
                .to_string()
        });

        let id = format!("proj-{}", Uuid::new_v4().simple());

        let project = Project {
            id,
            path: path.to_string(),
            name: project_name,
            associated_plan: plan_id.to_string(),
            created_at: Utc::now(),
        };

        projects.add_project(project.clone());
        Self::save_all(&projects)?;

        Ok(project)
    }

    pub fn dissociate_project(path: &str) -> Result<()> {
        let mut projects = Self::load_all()?;
        projects.remove_project(path)
            .context(format!("No project found at path: {}", path))?;

        Self::save_all(&projects)?;
        Ok(())
    }

    pub fn list_projects() -> Result<Vec<Project>> {
        let projects = Self::load_all()?;
        Ok(projects.list_all().to_vec())
    }

    pub fn get_project(path: &str) -> Result<Project> {
        let projects = Self::load_all()?;
        projects.get_project(path)
            .context(format!("No project found at path: {}", path))
            .map(|p| p.clone())
    }

    fn load_all() -> Result<Projects> {
        let path = get_projects_path()?;
        load_xml(path).context("Failed to load projects")
    }

    fn save_all(projects: &Projects) -> Result<()> {
        let path = get_projects_path()?;
        save_xml(path, projects).context("Failed to save projects")
    }
}

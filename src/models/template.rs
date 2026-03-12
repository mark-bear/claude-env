use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::plan::PlanContent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "content")]
    pub content: PlanContent,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "templates")]
pub struct Templates {
    #[serde(rename = "template", default)]
    pub templates: Vec<Template>,
}

impl Templates {
    pub fn new() -> Self {
        Self { templates: Vec::new() }
    }

    pub fn add_template(&mut self, template: Template) {
        self.templates.push(template);
    }

    pub fn get_template(&self, id: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.id == id)
    }

    pub fn remove_template(&mut self, id: &str) -> Option<Template> {
        let pos = self.templates.iter().position(|t| t.id == id)?;
        Some(self.templates.remove(pos))
    }
}

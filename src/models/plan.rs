use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "content")]
    pub content: PlanContent,
    #[serde(rename = "template_ref")]
    pub template_ref: Option<String>,
    #[serde(rename = "current_version")]
    pub current_version: u32,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "tags", default)]
    pub tags: PlanTags,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanContent {
    #[serde(rename = "step", default)]
    pub steps: Vec<String>,
}

impl PlanContent {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanTags {
    #[serde(rename = "tag", default)]
    pub tags: Vec<String>,
}

impl PlanTags {
    pub fn new() -> Self {
        Self { tags: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "plans")]
pub struct Plans {
    #[serde(rename = "plan", default)]
    pub plans: Vec<Plan>,
}

impl Plans {
    pub fn new() -> Self {
        Self { plans: Vec::new() }
    }

    pub fn add_plan(&mut self, plan: Plan) {
        self.plans.push(plan);
    }

    pub fn get_plan(&self, id: &str) -> Option<&Plan> {
        self.plans.iter().find(|p| p.id == id)
    }

    pub fn get_plan_mut(&mut self, id: &str) -> Option<&mut Plan> {
        self.plans.iter_mut().find(|p| p.id == id)
    }

    pub fn remove_plan(&mut self, id: &str) -> Option<Plan> {
        let pos = self.plans.iter().position(|p| p.id == id)?;
        Some(self.plans.remove(pos))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanSnapshot {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "version")]
    pub version: u32,
    #[serde(rename = "content")]
    pub content: PlanContent,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
}

impl PlanSnapshot {
    pub fn new(id: String, version: u32, content: PlanContent) -> Self {
        Self {
            id,
            version,
            content,
            created_at: Utc::now(),
        }
    }
}

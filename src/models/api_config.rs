use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "api_key")]
    pub api_key: String,
    #[serde(rename = "base_url")]
    pub base_url: String,
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "api_configs")]
pub struct ApiConfigs {
    #[serde(rename = "api_config", default)]
    pub configs: Vec<ApiConfig>,
}

impl ApiConfigs {
    pub fn new() -> Self {
        Self { configs: Vec::new() }
    }

    pub fn add_config(&mut self, config: ApiConfig) {
        self.configs.push(config);
    }

    pub fn get_config(&self, id: &str) -> Option<&ApiConfig> {
        self.configs.iter().find(|c| c.id == id)
    }

    pub fn get_active_config(&self) -> Option<&ApiConfig> {
        self.configs.iter().find(|c| c.is_active)
    }

    pub fn remove_config(&mut self, id: &str) -> Option<ApiConfig> {
        let pos = self.configs.iter().position(|c| c.id == id)?;
        Some(self.configs.remove(pos))
    }

    pub fn activate_config(&mut self, id: &str) -> bool {
        // Deactivate all configs first
        for config in &mut self.configs {
            config.is_active = false;
        }
        // Activate the specified config
        if let Some(config) = self.configs.iter_mut().find(|c| c.id == id) {
            config.is_active = true;
            true
        } else {
            false
        }
    }
}

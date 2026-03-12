use anyhow::{Context, Result};
use chrono::Utc;
use crate::models::{ApiConfig, ApiConfigs};
use crate::storage::{
    get_api_configs_path,
    load_xml,
    save_xml,
};
use crate::services::ClaudeCodeService;

pub struct ApiService;

impl ApiService {
    pub fn add_config(name: &str, api_key: &str, base_url: &str) -> Result<ApiConfig> {
        let mut configs = Self::load_all()?;

        // Check if name already exists
        if configs.configs.iter().any(|c| c.name == name) {
            anyhow::bail!("API config with name '{}' already exists", name);
        }

        // Generate a simple ID (first word of name, lowercased)
        let id = name
            .split_whitespace()
            .next()
            .unwrap_or("default")
            .to_lowercase();

        let config = ApiConfig {
            id,
            name: name.to_string(),
            api_key: api_key.to_string(),
            base_url: base_url.to_string(),
            is_active: configs.configs.is_empty(), // First config is active by default
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let is_first = configs.configs.is_empty();
        configs.add_config(config.clone());
        Self::save_all(&configs)?;

        // If this is the first config, sync to Claude Code
        if is_first {
            if let Err(e) = ClaudeCodeService::sync_api_config(&config.api_key, &config.base_url) {
                eprintln!("Warning: Failed to sync to Claude Code settings: {}", e);
            }
        }

        Ok(config)
    }

    pub fn list_configs() -> Result<Vec<ApiConfig>> {
        let configs = Self::load_all()?;
        Ok(configs.configs)
    }

    pub fn get_config(id: Option<&str>) -> Result<ApiConfig> {
        let configs = Self::load_all()?;

        let config = if let Some(id) = id {
            configs.get_config(id)
                .context(format!("API config '{}' not found", id))?
        } else {
            configs.get_active_config()
                .context("No active API config found")?
        };

        Ok(config.clone())
    }

    pub fn activate_config(id: &str) -> Result<ApiConfig> {
        let mut configs = Self::load_all()?;

        if !configs.activate_config(id) {
            anyhow::bail!("API config '{}' not found", id);
        }

        // Get the activated config
        let config = configs.get_config(id)
            .context("Failed to get activated config")?
            .clone();

        Self::save_all(&configs)?;

        // Sync to Claude Code settings
        if let Err(e) = ClaudeCodeService::sync_api_config(&config.api_key, &config.base_url) {
            eprintln!("Warning: Failed to sync to Claude Code settings: {}", e);
        }

        Ok(config)
    }

    pub fn delete_config(id: &str) -> Result<()> {
        let mut configs = Self::load_all()?;
        let config = configs.get_config(id);

        if config.is_none() {
            anyhow::bail!("API config '{}' not found", id);
        }

        let was_active = config.map(|c| c.is_active).unwrap_or(false);
        configs.remove_config(id);
        Self::save_all(&configs)?;

        // If we deleted the active config and there are others, activate the first one
        if was_active {
            if let Some(new_active) = configs.get_active_config() {
                // Sync the new active config to Claude Code
                if let Err(e) = ClaudeCodeService::sync_api_config(
                    &new_active.api_key,
                    &new_active.base_url,
                ) {
                    eprintln!("Warning: Failed to sync to Claude Code settings: {}", e);
                }
            } else if !configs.configs.is_empty() {
                let first_id = configs.configs[0].id.clone();
                configs.activate_config(&first_id);
                Self::save_all(&configs)?;
                // Sync the new active config
                if let Some(new_active) = configs.get_active_config() {
                    if let Err(e) = ClaudeCodeService::sync_api_config(
                        &new_active.api_key,
                        &new_active.base_url,
                    ) {
                        eprintln!("Warning: Failed to sync to Claude Code settings: {}", e);
                    }
                }
            } else {
                // No configs left, clear Claude Code settings
                if let Err(e) = ClaudeCodeService::clear_config() {
                    eprintln!("Warning: Failed to clear Claude Code settings: {}", e);
                }
            }
        }

        Ok(())
    }

    pub fn clear_all() -> Result<()> {
        let configs = ApiConfigs::new();
        Self::save_all(&configs)?;
        Ok(())
    }

    fn load_all() -> Result<ApiConfigs> {
        let path = get_api_configs_path()?;
        load_xml(path).context("Failed to load API configs")
    }

    fn save_all(configs: &ApiConfigs) -> Result<()> {
        let path = get_api_configs_path()?;
        save_xml(path, configs).context("Failed to save API configs")
    }
}

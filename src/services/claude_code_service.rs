use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

const CLAUDE_SETTINGS_FILE: &str = ".claude/settings.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClaudeSettings {
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabledPlugins: Option<HashMap<String, bool>>,
}

impl ClaudeSettings {
    pub fn load() -> Result<Self> {
        let path = get_settings_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&path)
            .context(format!("Failed to read settings file: {:?}", path))?;

        let settings: ClaudeSettings = serde_json::from_str(&content)
            .context("Failed to parse settings.json")?;

        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let path = get_settings_path()?;

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .context(format!("Failed to create directory: {:?}", parent))?;
            }
        }

        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize settings")?;

        std::fs::write(&path, content)
            .context(format!("Failed to write settings file: {:?}", path))?;

        Ok(())
    }

    pub fn set_api_config(&mut self, api_key: &str, base_url: &str) {
        self.env.insert("ANTHROPIC_AUTH_TOKEN".to_string(), api_key.to_string());
        self.env.insert("ANTHROPIC_BASE_URL".to_string(), base_url.to_string());
    }

    pub fn get_api_config(&self) -> Option<(String, String)> {
        let key = self.env.get("ANTHROPIC_AUTH_TOKEN")?;
        let url = self.env.get("ANTHROPIC_BASE_URL")?;
        Some((key.clone(), url.clone()))
    }

    pub fn clear_api_config(&mut self) {
        self.env.remove("ANTHROPIC_AUTH_TOKEN");
        self.env.remove("ANTHROPIC_BASE_URL");
    }
}

pub struct ClaudeCodeService;

impl ClaudeCodeService {
    /// Sync the active API config to Claude Code settings
    pub fn sync_api_config(api_key: &str, base_url: &str) -> Result<()> {
        let mut settings = ClaudeSettings::load()?;
        settings.set_api_config(api_key, base_url);
        settings.save()?;
        Ok(())
    }

    /// Get current API config from Claude Code settings
    pub fn get_current_config() -> Result<Option<(String, String)>> {
        let settings = ClaudeSettings::load()?;
        Ok(settings.get_api_config())
    }

    /// Clear API config from Claude Code settings
    pub fn clear_config() -> Result<()> {
        let mut settings = ClaudeSettings::load()?;
        settings.clear_api_config();
        settings.save()?;
        Ok(())
    }

    /// Check if settings file exists
    pub fn settings_exists() -> bool {
        get_settings_path().map(|p| p.exists()).unwrap_or(false)
    }

    /// Get settings file path
    pub fn get_settings_path() -> Result<PathBuf> {
        get_settings_path()
    }
}

fn get_settings_path() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Could not determine home directory")?;
    Ok(home.join(CLAUDE_SETTINGS_FILE))
}

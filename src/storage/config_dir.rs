use anyhow::{Context, Result};
use std::path::PathBuf;
use dirs::home_dir;

pub const CONFIG_DIR_NAME: &str = ".claude-env";
pub const VERSIONS_DIR_NAME: &str = "versions";
pub const API_CONFIGS_FILE: &str = "api_configs.xml";
pub const PLANS_FILE: &str = "plans.xml";
pub const TEMPLATES_FILE: &str = "templates.xml";
pub const PROJECTS_FILE: &str = "projects.xml";

pub fn get_config_dir() -> Result<PathBuf> {
    let home = home_dir().context("Could not determine home directory")?;
    let config_dir = home.join(CONFIG_DIR_NAME);
    Ok(config_dir)
}

pub fn get_versions_dir() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join(VERSIONS_DIR_NAME))
}

pub fn get_api_configs_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join(API_CONFIGS_FILE))
}

pub fn get_plans_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join(PLANS_FILE))
}

pub fn get_templates_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join(TEMPLATES_FILE))
}

pub fn get_projects_path() -> Result<PathBuf> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join(PROJECTS_FILE))
}

pub fn get_version_path(plan_id: &str, version: u32) -> Result<PathBuf> {
    let versions_dir = get_versions_dir()?;
    let filename = format!("{}_v{}.xml", plan_id, version);
    Ok(versions_dir.join(filename))
}

pub fn ensure_config_dir_exists() -> Result<()> {
    let config_dir = get_config_dir()?;
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;
    }

    let versions_dir = get_versions_dir()?;
    if !versions_dir.exists() {
        std::fs::create_dir_all(&versions_dir)
            .context("Failed to create versions directory")?;
    }

    Ok(())
}

pub fn init() -> Result<()> {
    ensure_config_dir_exists()?;

    // Initialize empty config files if they don't exist
    let api_configs_path = get_api_configs_path()?;
    if !api_configs_path.exists() {
        write_xml_file(&api_configs_path, r#"<?xml version="1.0" encoding="UTF-8"?>
<api_configs>
</api_configs>"#)?;
    }

    let plans_path = get_plans_path()?;
    if !plans_path.exists() {
        write_xml_file(&plans_path, r#"<?xml version="1.0" encoding="UTF-8"?>
<plans>
</plans>"#)?;
    }

    let templates_path = get_templates_path()?;
    if !templates_path.exists() {
        write_xml_file(&templates_path, r#"<?xml version="1.0" encoding="UTF-8"?>
<templates>
</templates>"#)?;
    }

    let projects_path = get_projects_path()?;
    if !projects_path.exists() {
        write_xml_file(&projects_path, r#"<?xml version="1.0" encoding="UTF-8"?>
<projects>
</projects>"#)?;
    }

    Ok(())
}

fn write_xml_file(path: &PathBuf, content: &str) -> Result<()> {
    std::fs::write(path, content)
        .context(format!("Failed to write XML file: {:?}", path))?;
    Ok(())
}

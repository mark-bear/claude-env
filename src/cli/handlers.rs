use anyhow::Result;
use tabled::Table;
use tabled_derive::Tabled;
use crate::services::*;

#[derive(Tabled)]
struct ApiConfigRow {
    #[tabled(rename = "ID")]
    id: String,
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Base URL")]
    base_url: String,
    #[tabled(rename = "Models")]
    models: String,
    #[tabled(rename = "Active")]
    is_active: String,
    #[tabled(rename = "Created At")]
    created_at: String,
}

// API Handlers
pub fn handle_api_add(name: &str, api_key: &str, base_url: &str, model: Option<String>) -> Result<()> {
    let config = ApiService::add_config(name, api_key, base_url, model.as_deref())?;
    println!("API config added successfully: {}", config.name);
    println!("ID: {}", config.id);
    if !config.models.is_empty() {
        println!("Models: {}", config.models.join(", "));
        if let Some(m) = &config.active_model {
            println!("Active Model: {}", m);
        }
    }
    Ok(())
}

pub fn handle_api_list() -> Result<()> {
    let configs = ApiService::list_configs()?;

    if configs.is_empty() {
        println!("No API configurations found.");
        return Ok(());
    }

    let rows: Vec<ApiConfigRow> = configs
        .into_iter()
        .map(|c| {
            let models_str = if c.models.is_empty() {
                "-".to_string()
            } else if c.models.len() == 1 {
                c.models[0].clone()
            } else {
                format!("{} ({}/{})", c.active_model.as_deref().unwrap_or("-"), c.models.len(), c.models.len())
            };
            ApiConfigRow {
                id: c.id,
                name: c.name,
                base_url: c.base_url,
                models: models_str,
                is_active: if c.is_active { "✓".to_string() } else { "✗".to_string() },
                created_at: c.created_at.format("%Y-%m-%d %H:%M").to_string(),
            }
        })
        .collect();

    println!("\n{}", Table::new(rows));
    Ok(())
}

pub fn handle_api_get(id: Option<&str>) -> Result<()> {
    let config = ApiService::get_config(id)?;

    println!("\nAPI Configuration:");
    println!("  ID:            {}", config.id);
    println!("  Name:          {}", config.name);
    println!("  Base URL:      {}", config.base_url);
    println!("  API Key:       {}***", &config.api_key[..8]);
    println!("  Active:        {}", if config.is_active { "Yes" } else { "No" });
    if config.models.is_empty() {
        println!("  Models:        (none)");
    } else {
        println!("  Models ({}):", config.models.len());
        for model in &config.models {
            let marker = if config.active_model.as_deref() == Some(model) {
                " *"
            } else {
                ""
            };
            println!("    - {}{}", model, marker);
        }
    }
    println!("  Created At:    {}", config.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("  Updated At:    {}", config.updated_at.format("%Y-%m-%d %H:%M:%S UTC"));

    Ok(())
}

pub fn handle_api_activate(id: &str) -> Result<()> {
    let config = ApiService::activate_config(id)?;
    println!("API config '{}' is now active.", id);

    // Show sync status
    match ClaudeCodeService::get_settings_path() {
        Ok(path) => {
            println!("Synced to Claude Code settings: {}", path.display());
            println!("  API Key: {}...", &config.api_key[..config.api_key.len().min(12)]);
            println!("  Base URL: {}", config.base_url);
            if let Some(model) = &config.active_model {
                println!("  Model: {}", model);
            }
        }
        Err(e) => {
            eprintln!("Warning: Could not determine Claude Code settings path: {}", e);
        }
    }

    Ok(())
}

pub fn handle_api_delete(id: &str) -> Result<()> {
    ApiService::delete_config(id)?;
    println!("API config '{}' deleted successfully.", id);
    Ok(())
}

pub fn handle_api_clear() -> Result<()> {
    ApiService::clear_all()?;
    println!("All API configurations cleared.");
    Ok(())
}

// Plan Handlers
pub fn handle_plan_create(name: &str, template: Option<String>, description: String) -> Result<()> {
    let plan = PlanService::create_plan(name, template, description)?;
    println!("Plan created successfully!");
    println!("  ID:   {}", plan.id);
    println!("  Name: {}", plan.name);
    Ok(())
}

pub fn handle_plan_list() -> Result<()> {
    let plans = PlanService::list_plans()?;

    if plans.is_empty() {
        println!("No plans found.");
        return Ok(());
    }

    println!("\nPlans:");
    for plan in plans {
        println!("  {} - {} (v{})", plan.id, plan.name, plan.current_version);
        println!("    {}", plan.description);
        if let Some(template) = plan.template_ref {
            println!("    Template: {}", template);
        }
        println!();
    }

    Ok(())
}

pub fn handle_plan_view(plan_id: &str) -> Result<()> {
    let plan = PlanService::get_plan(plan_id)?;

    println!("\nPlan: {}", plan.name);
    println!("ID:        {}", plan.id);
    println!("Version:   {}", plan.current_version);
    println!("Updated:   {}", plan.updated_at.format("%Y-%m-%d %H:%M:%S UTC"));

    if let Some(template) = plan.template_ref {
        println!("Template:  {}", template);
    }

    println!("\nDescription:");
    println!("  {}", plan.description);

    println!("\nSteps:");
    if plan.content.steps.is_empty() {
        println!("  (No steps yet)");
    } else {
        for (i, step) in plan.content.steps.iter().enumerate() {
            println!("  {}. {}", i + 1, step);
        }
    }

    if !plan.tags.tags.is_empty() {
        println!("\nTags: {}", plan.tags.tags.join(", "));
    }

    Ok(())
}

pub fn handle_plan_update(plan_id: &str, name: Option<String>, description: Option<String>) -> Result<()> {
    let plan = PlanService::update_plan(plan_id, name, description)?;
    println!("Plan updated successfully!");
    println!("  ID:   {}", plan.id);
    println!("  Name: {}", plan.name);
    Ok(())
}

pub fn handle_plan_delete(plan_id: &str) -> Result<()> {
    PlanService::delete_plan(plan_id)?;
    println!("Plan '{}' deleted successfully.", plan_id);
    Ok(())
}

pub fn handle_plan_add_step(plan_id: &str, step_content: &str) -> Result<()> {
    let plan = PlanService::add_step(plan_id, step_content)?;
    println!("Step added to plan '{}' (now v{})", plan_id, plan.current_version);
    Ok(())
}

pub fn handle_plan_version_history(plan_id: &str) -> Result<()> {
    let versions = PlanService::get_version_history(plan_id)?;

    if versions.is_empty() {
        println!("No versions found for plan '{}'.", plan_id);
        return Ok(());
    }

    println!("\nVersion history for plan '{}':", plan_id);
    for version in versions {
        println!("  Version {}", version);
    }

    Ok(())
}

pub fn handle_plan_version_rollback(plan_id: &str, version: u32) -> Result<()> {
    let plan = PlanService::rollback_plan(plan_id, version)?;
    println!("Plan '{}' rolled back to version {} (now at v{})", plan_id, version, plan.current_version);
    Ok(())
}

// Template Handlers
pub fn handle_template_create(name: &str, file: &str) -> Result<()> {
    let template = TemplateService::create_template(name, file)?;
    println!("Template created successfully!");
    println!("  ID:   {}", template.id);
    println!("  Name: {}", template.name);
    Ok(())
}

pub fn handle_template_list() -> Result<()> {
    let templates = TemplateService::list_templates()?;

    if templates.is_empty() {
        println!("No templates found.");
        return Ok(());
    }

    println!("\nTemplates:");
    for template in templates {
        println!("  {} - {}", template.id, template.name);
        println!("    {}", template.description);
        println!();
    }

    Ok(())
}

pub fn handle_template_view(template_id: &str) -> Result<()> {
    let template = TemplateService::get_template(template_id)?;

    println!("\nTemplate: {}", template.name);
    println!("ID:        {}", template.id);
    println!("Created:   {}", template.created_at.format("%Y-%m-%d %H:%M:%S UTC"));

    println!("\nDescription:");
    println!("  {}", template.description);

    println!("\nSteps:");
    if template.content.steps.is_empty() {
        println!("  (No steps)");
    } else {
        for (i, step) in template.content.steps.iter().enumerate() {
            println!("  {}. {}", i + 1, step);
        }
    }

    Ok(())
}

pub fn handle_template_delete(template_id: &str) -> Result<()> {
    TemplateService::delete_template(template_id)?;
    println!("Template '{}' deleted successfully.", template_id);
    Ok(())
}

// Project Handlers
pub fn handle_project_associate(path: &str, plan_id: &str, name: Option<String>) -> Result<()> {
    let project = ProjectService::associate_project(path, plan_id, name)?;
    println!("Project associated successfully!");
    println!("  ID:   {}", project.id);
    println!("  Name: {}", project.name);
    println!("  Path: {}", project.path);
    println!("  Plan: {}", project.associated_plan);
    Ok(())
}

pub fn handle_project_dissociate(path: &str) -> Result<()> {
    ProjectService::dissociate_project(path)?;
    println!("Project at '{}' dissociated successfully.", path);
    Ok(())
}

pub fn handle_project_list() -> Result<()> {
    let projects = ProjectService::list_projects()?;

    if projects.is_empty() {
        println!("No projects found.");
        return Ok(());
    }

    println!("\nProjects:");
    for project in projects {
        println!("  {}", project.name);
        println!("    Path:   {}", project.path);
        println!("    Plan:   {}", project.associated_plan);
        println!("    Since:  {}", project.created_at.format("%Y-%m-%d %H:%M"));
        println!();
    }

    Ok(())
}

pub fn handle_project_view(path: &str) -> Result<()> {
    let project = ProjectService::get_project(path)?;

    println!("\nProject: {}", project.name);
    println!("ID:              {}", project.id);
    println!("Path:            {}", project.path);
    println!("Associated Plan: {}", project.associated_plan);
    println!("Created At:      {}", project.created_at.format("%Y-%m-%d %H:%M:%S UTC"));

    Ok(())
}

// Environment Handlers
pub fn handle_env_enter(path: &str) -> Result<()> {
    let project = ProjectService::get_project(path)?;
    let api_config = ApiService::get_config(None)?;
    let plan = PlanService::get_plan(&project.associated_plan)?;

    println!("# Environment configuration for: {}", project.name);
    println!("# Generated by claude-env");
    println!("# Plan: {} ({})", plan.name, project.associated_plan);
    println!();
    println!("export ANTHROPIC_API_KEY=\"{}\"", api_config.api_key);
    println!("export ANTHROPIC_BASE_URL=\"{}\"", api_config.base_url);
    if let Some(model) = &api_config.active_model {
        println!("export ANTHROPIC_MODEL=\"{}\"", model);
    }
    println!("export CLAUDE_ENV_PLAN=\"{}\"", project.associated_plan);
    println!("export CLAUDE_ENV_PLAN_NAME=\"{}\"", plan.name);
    println!("cd \"{}\"", project.path);
    println!();
    println!("echo \"Environment loaded for project: {}\"", project.name);
    println!("echo \"Active plan: {} ({})\"", plan.name, project.associated_plan);
    println!("echo \"Plan has {} steps\"", plan.content.steps.len());

    Ok(())
}

// Init Handler
pub fn handle_init() -> Result<()> {
    crate::storage::init()?;
    println!("Claude environment initialized successfully!");
    println!("Configuration directory: ~/.claude-env/");
    Ok(())
}

// Sync Handler - Sync active API config to Claude Code settings
pub fn handle_api_sync() -> Result<()> {
    let config = ApiService::get_config(None)?;

    println!("Syncing active API config to Claude Code settings...");
    println!("  Config: {} ({})", config.name, config.id);
    println!("  Base URL: {}", config.base_url);
    if let Some(model) = &config.active_model {
        println!("  Model: {}", model);
    }

    ClaudeCodeService::sync_api_config(&config.api_key, &config.base_url, config.active_model.as_deref())?;

    match ClaudeCodeService::get_settings_path() {
        Ok(path) => println!("\nSuccessfully synced to: {}", path.display()),
        Err(_) => println!("\nSuccessfully synced to Claude Code settings"),
    }

    println!("\nClaude Code will use this configuration for subsequent sessions.");
    Ok(())
}

// Model Management Handlers
pub fn handle_api_model_add(api_id: &str, model: &str) -> Result<()> {
    let config = ApiService::add_model(api_id, model)?;
    println!("Model '{}' added to API config '{}'.", model, api_id);
    if config.models.len() == 1 {
        println!("This is the first model, so it's automatically selected as active.");
    } else {
        println!("Available models ({}):", config.models.len());
        for m in &config.models {
            let marker = if config.active_model.as_deref() == Some(m) { " *" } else { "" };
            println!("  - {}{}", m, marker);
        }
    }
    Ok(())
}

pub fn handle_api_model_remove(api_id: &str, model: &str) -> Result<()> {
    let config = ApiService::remove_model(api_id, model)?;
    println!("Model '{}' removed from API config '{}'.", model, api_id);
    if config.models.is_empty() {
        println!("No models remaining in this config.");
    } else {
        println!("Remaining models ({}):", config.models.len());
        for m in &config.models {
            let marker = if config.active_model.as_deref() == Some(m) { " *" } else { "" };
            println!("  - {}{}", m, marker);
        }
        if let Some(active_model) = &config.active_model {
            println!("\nActive model is now: {}", active_model);
        }
    }
    Ok(())
}

pub fn handle_api_model_select(api_id: &str, model: &str) -> Result<()> {
    let config = ApiService::select_model(api_id, model)?;
    println!("Model '{}' is now the active model for API config '{}'.", model, api_id);
    println!("\nAll available models:");
    for m in &config.models {
        let marker = if config.active_model.as_deref() == Some(m) { " *" } else { "" };
        println!("  - {}{}", m, marker);
    }
    Ok(())
}

pub fn handle_api_model_list(api_id: &str) -> Result<()> {
    let (models, active_model) = ApiService::list_models(api_id)?;

    if models.is_empty() {
        println!("No models configured for API config '{}'.", api_id);
        println!("\nUse 'claude-env api model add {} <model>' to add a model.", api_id);
    } else {
        println!("\nModels for API config '{}':", api_id);
        for model in &models {
            let marker = if active_model.as_deref() == Some(model) { " * (active)" } else { "" };
            println!("  {}{}", model, marker);
        }
        println!("\n* indicates the currently active model");
    }

    Ok(())
}

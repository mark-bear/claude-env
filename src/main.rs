use clap::{Parser, Subcommand};
use anyhow::Result;

mod models;
mod storage;
mod services;
mod cli;

use cli::*;

#[derive(Parser)]
#[command(
    name = "claude-env",
    version = "0.1.2",
    about = "Manage Claude AI coding environments",
    long_about = r#"
claude-env - A CLI tool for managing Claude AI coding environments

This tool helps you organize API configurations, project plans, templates,
and quickly switch between different working environments.

EXAMPLES:
    # Initialize the environment
    claude-env init

    # Add an API configuration
    claude-env api add mykey "sk-ant-xxx" --base-url "https://api.anthropic.com"

    # Create a development plan
    claude-env plan create "My Project" --description "Project description"

    # Associate a project with a plan
    claude-env project associate /path/to/project plan-id --name "My Project"

    # Enter the environment (use with eval)
    eval $(claude-env env enter /path/to/project)
"#
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage API configurations
    ///
    /// Add, list, activate, and delete API configurations.
    /// Multiple configurations can be stored, with one active at a time.
    Api {
        #[command(subcommand)]
        action: ApiCommands,
    },
    /// Manage development plans
    ///
    /// Create and manage project plans with version control.
    /// Plans can include multiple steps and can be rolled back to previous versions.
    Plan {
        #[command(subcommand)]
        action: PlanCommands,
    },
    /// Manage plan templates
    ///
    /// Create reusable templates from XML files.
    /// Templates can be used as starting points for new plans.
    Template {
        #[command(subcommand)]
        action: TemplateCommands,
    },
    /// Manage project associations
    ///
    /// Associate local project directories with plans.
    /// This links a project path to a specific plan and API configuration.
    Project {
        #[command(subcommand)]
        action: ProjectCommands,
    },
    /// Environment setup commands
    ///
    /// Generate shell commands to set up environment variables.
    /// Use with 'eval' to enter a project's environment.
    Env {
        #[command(subcommand)]
        action: EnvCommands,
    },
    /// Initialize claude-env configuration
    ///
    /// Creates the configuration directory (~/.claude-env/) and
    /// initializes empty configuration files.
    Init,
}

#[derive(Subcommand)]
enum ApiCommands {
    /// Add a new API configuration
    ///
    /// Creates a new API configuration with the given name and key.
    /// The first configuration added will be automatically activated.
    ///
    /// EXAMPLES:
    ///     claude-env api add mykey "sk-ant-api03-xxx"
    ///     claude-env api add work "sk-ant-api03-xxx" --base-url "https://api.anthropic.com"
    Add {
        /// Display name for this configuration (e.g., "personal", "work")
        name: String,
        /// Anthropic API key (starts with "sk-ant-")
        api_key: String,
        /// Base URL for the Anthropic API [default: https://api.anthropic.com]
        #[arg(long)]
        base_url: Option<String>,
    },
    /// List all API configurations
    ///
    /// Displays a table of all configured API keys.
    /// The active configuration is marked with a checkmark.
    List,
    /// Get details of a specific API configuration
    ///
    /// Shows full details including masked API key.
    /// If no ID is provided, shows the active configuration.
    ///
    /// EXAMPLES:
    ///     claude-env api get
    ///     claude-env api get mykey
    Get {
        /// Configuration ID to display (defaults to active config)
        id: Option<String>,
    },
    /// Activate an API configuration
    ///
    /// Sets the specified configuration as active.
    /// Only one configuration can be active at a time.
    ///
    /// EXAMPLE:
    ///     claude-env api activate mykey
    Activate {
        /// ID of the configuration to activate
        id: String,
    },
    /// Delete an API configuration
    ///
    /// Permanently removes the specified configuration.
    /// If the deleted config was active, another will be activated automatically.
    ///
    /// EXAMPLE:
    ///     claude-env api delete oldkey
    Delete {
        /// ID of the configuration to delete
        id: String,
    },
    /// Remove all API configurations
    ///
    /// Deletes all stored API configurations. Use with caution.
    Clear,
    /// Sync active config to Claude Code
    ///
    /// Manually syncs the currently active API configuration
    /// to Claude Code's settings.json file.
    ///
    /// EXAMPLE:
    ///     claude-env api sync
    Sync,
}

#[derive(Subcommand)]
enum PlanCommands {
    /// Create a new development plan
    ///
    /// Creates a plan that can be associated with projects.
    /// Optionally use a template to pre-populate steps.
    ///
    /// EXAMPLES:
    ///     claude-env plan create "My App"
    ///     claude-env plan create "Web Project" --description "Build a web app"
    ///     claude-env plan create "API" --template rust-api
    Create {
        /// Name for the plan
        name: String,
        /// Template ID to use for initial steps
        #[arg(long)]
        template: Option<String>,
        /// Description of the plan's purpose
        #[arg(long)]
        description: Option<String>,
    },
    /// List all plans
    ///
    /// Shows all plans with their current version and description.
    List,
    /// View plan details
    ///
    /// Displays full plan information including all steps.
    ///
    /// EXAMPLE:
    ///     claude-env plan view plan-xxx
    View {
        /// Plan ID to display
        plan_id: String,
    },
    /// Update plan metadata
    ///
    /// Modify the name and/or description of an existing plan.
    ///
    /// EXAMPLES:
    ///     claude-env plan update plan-xxx --name "New Name"
    ///     claude-env plan update plan-xxx --description "Updated description"
    Update {
        /// Plan ID to update
        plan_id: String,
        /// New name for the plan
        #[arg(long)]
        name: Option<String>,
        /// New description for the plan
        #[arg(long)]
        description: Option<String>,
    },
    /// Delete a plan
    ///
    /// Permanently removes a plan. This does not affect
    /// associated projects, but they will lose their plan reference.
    ///
    /// EXAMPLE:
    ///     claude-env plan delete plan-xxx
    Delete {
        /// Plan ID to delete
        plan_id: String,
    },
    /// Add a step to a plan
    ///
    /// Appends a new step to the plan and increments the version.
    /// Previous versions are saved for rollback.
    ///
    /// EXAMPLE:
    ///     claude-env plan add-step plan-xxx "Implement user authentication"
    AddStep {
        /// Plan ID to modify
        plan_id: String,
        /// Content/description of the step
        step_content: String,
    },
    /// Plan version management
    ///
    /// View version history or rollback to a previous version.
    Version {
        #[command(subcommand)]
        action: VersionCommands,
    },
}

#[derive(Subcommand)]
enum VersionCommands {
    /// Show version history for a plan
    ///
    /// Lists all available versions of a plan.
    ///
    /// EXAMPLE:
    ///     claude-env plan version history plan-xxx
    History {
        /// Plan ID to show history for
        plan_id: String,
    },
    /// Rollback to a previous version
    ///
    /// Restores the plan to a specific version.
    /// A new version is created with the old content.
    ///
    /// EXAMPLE:
    ///     claude-env plan version rollback plan-xxx 3
    Rollback {
        /// Plan ID to rollback
        plan_id: String,
        /// Version number to restore
        version: u32,
    },
}

#[derive(Subcommand)]
enum TemplateCommands {
    /// Create a template from an XML file
    ///
    /// Reads an XML file containing steps and description,
    /// creates a reusable template for new plans.
    ///
    /// EXAMPLE:
    ///     claude-env template create "Rust API" --file ./templates/rust-api.xml
    Create {
        /// Name for the template
        name: String,
        /// Path to the XML template file
        #[arg(long)]
        file: String,
    },
    /// List all templates
    ///
    /// Shows all available templates with descriptions.
    List,
    /// View template details
    ///
    /// Displays the full content of a template including all steps.
    ///
    /// EXAMPLE:
    ///     claude-env template view rust-api
    View {
        /// Template ID to display
        template_id: String,
    },
    /// Delete a template
    ///
    /// Permanently removes a template.
    ///
    /// EXAMPLE:
    ///     claude-env template delete old-template
    Delete {
        /// Template ID to delete
        template_id: String,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    /// Link a project directory to a plan
    ///
    /// Associates a local directory with a plan and API configuration.
    /// The directory name is used as the project name if not specified.
    ///
    /// EXAMPLES:
    ///     claude-env project associate ~/projects/my-app plan-xxx
    ///     claude-env project associate ~/projects/my-app plan-xxx --name "My Application"
    Associate {
        /// Path to the project directory
        path: String,
        /// Plan ID to associate with this project
        plan_id: String,
        /// Display name for the project (defaults to directory name)
        #[arg(long)]
        name: Option<String>,
    },
    /// Unlink a project directory
    ///
    /// Removes the association between a directory and its plan.
    ///
    /// EXAMPLE:
    ///     claude-env project dissociate ~/projects/my-app
    Dissociate {
        /// Path to the project directory
        path: String,
    },
    /// List all project associations
    ///
    /// Shows all projects with their associated plans.
    List,
    /// View project details
    ///
    /// Displays information about a specific project association.
    ///
    /// EXAMPLE:
    ///     claude-env project view ~/projects/my-app
    View {
        /// Path to the project directory
        path: String,
    },
}

#[derive(Subcommand)]
enum EnvCommands {
    /// Generate environment setup for a project
    ///
    /// Outputs shell commands to set environment variables.
    /// Use with 'eval' to enter the environment, or redirect to a file.
    ///
    /// Sets the following variables:
    ///   - ANTHROPIC_API_KEY
    ///   - ANTHROPIC_BASE_URL
    ///   - CLAUDE_ENV_PLAN
    ///   - CLAUDE_ENV_PLAN_NAME
    ///
    /// EXAMPLES:
    ///     eval $(claude-env env enter ~/projects/my-app)
    ///     claude-env env enter ~/projects/my-app > .env
    Enter {
        /// Path to the project directory
        path: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Api { action } => match action {
            ApiCommands::Add { name, api_key, base_url } => {
                let base_url = base_url.unwrap_or_else(|| "https://api.anthropic.com".to_string());
                handle_api_add(&name, &api_key, &base_url)?;
            }
            ApiCommands::List => {
                handle_api_list()?;
            }
            ApiCommands::Get { id } => {
                handle_api_get(id.as_deref())?;
            }
            ApiCommands::Activate { id } => {
                handle_api_activate(&id)?;
            }
            ApiCommands::Delete { id } => {
                handle_api_delete(&id)?;
            }
            ApiCommands::Clear => {
                handle_api_clear()?;
            }
            ApiCommands::Sync => {
                handle_api_sync()?;
            }
        },
        Commands::Plan { action } => match action {
            PlanCommands::Create { name, template, description } => {
                let description = description.unwrap_or_else(|| "No description".to_string());
                handle_plan_create(&name, template, description)?;
            }
            PlanCommands::List => {
                handle_plan_list()?;
            }
            PlanCommands::View { plan_id } => {
                handle_plan_view(&plan_id)?;
            }
            PlanCommands::Update { plan_id, name, description } => {
                handle_plan_update(&plan_id, name, description)?;
            }
            PlanCommands::Delete { plan_id } => {
                handle_plan_delete(&plan_id)?;
            }
            PlanCommands::AddStep { plan_id, step_content } => {
                handle_plan_add_step(&plan_id, &step_content)?;
            }
            PlanCommands::Version { action } => match action {
                VersionCommands::History { plan_id } => {
                    handle_plan_version_history(&plan_id)?;
                }
                VersionCommands::Rollback { plan_id, version } => {
                    handle_plan_version_rollback(&plan_id, version)?;
                }
            },
        },
        Commands::Template { action } => match action {
            TemplateCommands::Create { name, file } => {
                handle_template_create(&name, &file)?;
            }
            TemplateCommands::List => {
                handle_template_list()?;
            }
            TemplateCommands::View { template_id } => {
                handle_template_view(&template_id)?;
            }
            TemplateCommands::Delete { template_id } => {
                handle_template_delete(&template_id)?;
            }
        },
        Commands::Project { action } => match action {
            ProjectCommands::Associate { path, plan_id, name } => {
                handle_project_associate(&path, &plan_id, name)?;
            }
            ProjectCommands::Dissociate { path } => {
                handle_project_dissociate(&path)?;
            }
            ProjectCommands::List => {
                handle_project_list()?;
            }
            ProjectCommands::View { path } => {
                handle_project_view(&path)?;
            }
        },
        Commands::Env { action } => match action {
            EnvCommands::Enter { path } => {
                handle_env_enter(&path)?;
            }
        },
        Commands::Init => {
            handle_init()?;
        }
    }

    Ok(())
}

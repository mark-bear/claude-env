# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Claude Environment Manager (claude-env) is a CLI tool for managing Claude AI coding environments. It helps developers organize API configurations, project plans, templates, and switch between different working environments.

## Build and Test Commands

```bash
# Build release version
cargo build --release

# Run tests
cargo test

# Run specific test
cargo test test_name

# Lint
cargo clippy

# Format code
cargo fmt

# Run the CLI (development)
cargo run -- <command>

# Run the CLI (release)
./target/release/claude-env <command>
```

## Architecture

The codebase follows a layered architecture:

```
src/
├── main.rs           # Entry point, clap command definitions
├── cli/
│   ├── mod.rs        # Re-exports handlers
│   └── handlers.rs   # CLI command handlers (output formatting)
├── services/         # Business logic layer
│   ├── api_service.rs       # API config CRUD, activation
│   ├── plan_service.rs      # Plans with version control
│   ├── template_service.rs  # XML template parsing
│   ├── project_service.rs   # Project-path associations
│   └── claude_code_service.rs # Sync to ~/.claude/settings.json
├── models/           # Data structures (Serde-serializable)
│   ├── api_config.rs
│   ├── plan.rs
│   ├── template.rs
│   └── project.rs
└── storage/
    ├── config_dir.rs  # Config directory setup (~/.claude-env/)
    └── xml_storage.rs # XML serialization helpers (quick-xml)
```

### Data Flow

1. **CLI** parses commands via clap in `main.rs`
2. **Handlers** in `cli/handlers.rs` call services and format output
3. **Services** contain business logic and coordinate storage
4. **Storage** persists data as XML files in `~/.claude-env/`

### Configuration Storage

All user data is stored in `~/.claude-env/`:
- `api_configs.xml` - API configurations (includes optional model field)
- `plans.xml` - Development plans
- `templates.xml` - Reusable plan templates
- `projects.xml` - Project-path associations
- `versions/` - Plan version history

### Environment Variables

The `env enter` command generates shell commands to set:
- `ANTHROPIC_API_KEY` - Active API key
- `ANTHROPIC_BASE_URL` - API endpoint URL
- `ANTHROPIC_MODEL` - Default model (if configured)
- `CLAUDE_ENV_PLAN` - Associated plan ID
- `CLAUDE_ENV_PLAN_NAME` - Plan name

### Claude Code Integration

The `claude_code_service.rs` syncs API configs to `~/.claude/settings.json`, setting:
- `ANTHROPIC_AUTH_TOKEN`
- `ANTHROPIC_BASE_URL`
- `ANTHROPIC_MODEL` (optional, when model is configured)

This allows `api activate` to automatically configure Claude Code sessions.

### API Configuration Features

API configs support:
- **name**: Display name for the configuration
- **api_key**: Anthropic API key
- **base_url**: API endpoint URL (default: https://api.anthropic.com)
- **model**: Default model to use (optional)

Commands:
- `api add <name> <key> [--base-url URL] [--model MODEL]` - Add new config
- `api list` - List all configs
- `api get [id]` - Show config details (active if no ID)
- `api activate <id>` - Set config as active and sync to Claude Code
- `api delete <id>` - Remove config
- `api clear` - Remove all configs
- `api sync` - Manually sync active config to Claude Code settings

## Key Dependencies

- **clap** - CLI argument parsing with derive macros
- **quick-xml** - XML serialization/deserialization
- **serde** - Serialization framework
- **serde_json** - JSON serialization (for Claude Code settings)
- **tabled** - Table output formatting
- **anyhow/thiserror** - Error handling
- **chrono** - Timestamps
- **uuid** - Unique IDs
- **dirs** - Home directory resolution

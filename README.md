# Claude Environment Manager (claude-env)

一个用于管理 Claude AI 编码环境的命令行工具。帮助开发者组织 API 配置、项目计划、模板，并快速切换不同的工作环境。

## 功能特性

- **API 配置管理**: 管理多个 Claude API 配置，支持快速切换
- **计划管理**: 创建和管理项目开发计划，支持版本控制和回滚
- **模板系统**: 从 XML 文件创建可复用的计划模板
- **项目关联**: 将本地项目路径与计划关联
- **环境切换**: 一键输出环境变量配置，快速进入工作状态

## 安装

### 从源码构建

```bash
# 克隆仓库
git clone <repository-url>
cd claude-env

# 构建发布版本
cargo build --release

# 将二进制文件添加到 PATH
sudo cp target/release/claude-env /usr/local/bin/
```

### 依赖要求

- Rust 1.70 或更高版本
- Linux/macOS/Windows

## 快速开始

### 1. 初始化环境

```bash
claude-env init
```

这会在 `~/.claude-env/` 目录下创建配置文件。

### 2. 添加 API 配置

```bash
claude-env api add "My API Key" "sk-ant-api03-xxx" --base-url "https://api.anthropic.com"
```

### 3. 创建计划

```bash
claude-env plan create "My Project Plan" --description "A sample development plan"
```

### 4. 关联项目

```bash
claude-env project associate /path/to/your/project plan-xxx --name "My Project"
```

### 5. 进入工作环境

```bash
eval $(claude-env env enter /path/to/your/project)
```

## 命令详解

### `init` - 初始化

```bash
claude-env init
```

初始化配置目录 `~/.claude-env/`，创建必要的 XML 配置文件。

---

### `api` - API 配置管理

#### 添加配置
```bash
claude-env api add <NAME> <API_KEY> [--base-url <URL>]
```

示例：
```bash
claude-env api add "Production" "sk-ant-api03-..."
claude-env api add "Staging" "sk-ant-api03-..." --base-url "https://staging.api.anthropic.com"
```

#### 列出所有配置
```bash
claude-env api list
```

输出示例：
```
┌──────────────┬─────────────┬──────────────────────────────┬────────┬─────────────────┐
│ ID           │ Name        │ Base URL                     │ Active │ Created At      │
├──────────────┼─────────────┼──────────────────────────────┼────────┼─────────────────┤
│ production   │ Production  │ https://api.anthropic.com    │ ✓      │ 2024-01-15 09:30│
│ staging      │ Staging     │ https://staging.api.anth...  │ ✗      │ 2024-01-15 09:35│
└──────────────┴─────────────┴──────────────────────────────┴────────┴─────────────────┘
```

#### 查看配置详情
```bash
claude-env api get [ID]
```

不带 ID 时显示当前激活的配置。

#### 激活配置
```bash
claude-env api activate <ID>
```

#### 删除配置
```bash
claude-env api delete <ID>
```

#### 清空所有配置
```bash
claude-env api clear
```

---

### `plan` - 计划管理

#### 创建计划
```bash
claude-env plan create <NAME> [--template <TEMPLATE_ID>] [--description <DESC>]
```

示例：
```bash
claude-env plan create "Web App v2" --description "Develop the new web application"
claude-env plan create "API Project" --template "rust-api"
```

#### 列出所有计划
```bash
claude-env plan list
```

#### 查看计划详情
```bash
claude-env plan view <PLAN_ID>
```

#### 更新计划
```bash
claude-env plan update <PLAN_ID> [--name <NEW_NAME>] [--description <NEW_DESC>]
```

#### 删除计划
```bash
claude-env plan delete <PLAN_ID>
```

#### 添加步骤
```bash
claude-env plan add-step <PLAN_ID> "<STEP_CONTENT>"
```

示例：
```bash
claude-env plan add-step plan-xxx "Set up project structure"
claude-env plan add-step plan-xxx "Implement user authentication"
```

每添加一个步骤，计划的版本号会自动递增。

#### 版本历史
```bash
claude-env plan version history <PLAN_ID>
```

#### 版本回滚
```bash
claude-env plan version rollback <PLAN_ID> <VERSION_NUMBER>
```

示例：
```bash
claude-env plan version rollback plan-xxx 3
```

---

### `template` - 模板管理

#### 从 XML 文件创建模板
```bash
claude-env template create <NAME> --file <XML_FILE_PATH>
```

XML 文件格式示例：
```xml
<?xml version="1.0" encoding="UTF-8"?>
<template>
    <description>A template for Rust API projects</description>
    <steps>
        <step>Initialize Cargo project</step>
        <step>Set up basic dependencies (tokio, axum, serde)</step>
        <step>Create project structure</step>
        <step>Implement health check endpoint</step>
        <step>Add error handling middleware</step>
    </steps>
</template>
```

创建模板：
```bash
claude-env template create "Rust API" --file ./templates/rust-api.xml
```

#### 列出所有模板
```bash
claude-env template list
```

#### 查看模板详情
```bash
claude-env template view <TEMPLATE_ID>
```

#### 删除模板
```bash
claude-env template delete <TEMPLATE_ID>
```

---

### `project` - 项目管理

#### 关联项目
```bash
claude-env project associate <PATH> <PLAN_ID> [--name <PROJECT_NAME>]
```

示例：
```bash
claude-env project associate ~/projects/my-app plan-xxx --name "My Application"
```

#### 取消关联
```bash
claude-env project dissociate <PATH>
```

#### 列出所有项目
```bash
claude-env project list
```

#### 查看项目详情
```bash
claude-env project view <PATH>
```

---

### `env` - 环境命令

#### 进入项目环境
```bash
claude-env env enter <PATH>
```

这个命令会输出一系列 shell 命令，用于设置环境变量：

```bash
eval $(claude-env env enter ~/projects/my-app)
```

输出的环境变量包括：
- `ANTHROPIC_API_KEY` - API 密钥
- `ANTHROPIC_BASE_URL` - API 基础 URL
- `CLAUDE_ENV_PLAN` - 当前计划的 ID
- `CLAUDE_ENV_PLAN_NAME` - 当前计划的名称

也可以将输出保存到文件：
```bash
claude-env env enter ~/projects/my-app > .env
cd ~/projects/my-app
```

## 完整工作流示例

### 场景：管理多个项目

```bash
# 1. 初始化
claude-env init

# 2. 添加 API 配置
claude-env api add "Personal" "sk-ant-api03-xxxxxxxx"
claude-env api add "Work" "sk-ant-api03-yyyyyyyy"

# 3. 创建模板
cat > ~/templates/web-app.xml << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<template>
    <description>Standard web application template</description>
    <steps>
        <step>Set up project repository</step>
        <step>Configure CI/CD pipeline</step>
        <step>Set up development environment</step>
        <step>Implement core features</step>
        <step>Add tests</step>
        <step>Deploy to staging</step>
    </steps>
</template>
EOF

claude-env template create "Web App" --file ~/templates/web-app.xml

# 4. 创建计划
claude-env plan create "E-commerce Platform" --template "web-app" --description "Build an e-commerce platform"

# 5. 关联项目
claude-env project associate ~/work/ecommerce plan-xxx --name "E-commerce"

# 6. 开始工作
cd ~/work/ecommerce
eval $(claude-env env enter .)
```

## 配置文件

所有配置存储在 `~/.claude-env/` 目录下：

```
~/.claude-env/
├── api_configs.xml    # API 配置
├── plans.xml          # 计划列表
├── templates.xml      # 模板列表
├── projects.xml       # 项目关联
└── versions/          # 计划版本历史
    ├── plan-xxx_v1.xml
    ├── plan-xxx_v2.xml
    └── ...
```

### 配置文件格式

#### api_configs.xml
```xml
<?xml version="1.0" encoding="UTF-8"?>
<api_configs>
    <api_config>
        <id>personal</id>
        <name>Personal</name>
        <api_key>sk-ant-api03-...</api_key>
        <base_url>https://api.anthropic.com</base_url>
        <is_active>true</is_active>
        <created_at>2024-01-15T09:30:00Z</created_at>
        <updated_at>2024-01-15T09:30:00Z</updated_at>
    </api_config>
</api_configs>
```

#### plans.xml
```xml
<?xml version="1.0" encoding="UTF-8"?>
<plans>
    <plan>
        <id>plan-xxx</id>
        <name>My Project</name>
        <description>Project description</description>
        <content>
            <step>Step 1</step>
            <step>Step 2</step>
        </content>
        <template_ref>web-app</template_ref>
        <current_version>2</current_version>
        <created_at>2024-01-15T09:30:00Z</created_at>
        <updated_at>2024-01-15T10:00:00Z</updated_at>
        <tags></tags>
    </plan>
</plans>
```

## 环境变量

当使用 `claude-env env enter` 时，会设置以下环境变量：

| 变量名 | 说明 |
|--------|------|
| `ANTHROPIC_API_KEY` | 当前激活的 API 密钥 |
| `ANTHROPIC_BASE_URL` | API 基础 URL |
| `CLAUDE_ENV_PLAN` | 关联计划的 ID |
| `CLAUDE_ENV_PLAN_NAME` | 关联计划的名称 |

## 与 Claude Code 集成

将以下函数添加到你的 shell 配置文件（`.bashrc` 或 `.zshrc`）：

```bash
# 快速进入项目环境
claude-work() {
    local project_path="${1:-.}"
    eval $(claude-env env enter "$project_path")
}

# 显示当前环境
claude-status() {
    echo "API Key: ${ANTHROPIC_API_KEY:0:12}..."
    echo "Base URL: ${ANTHROPIC_BASE_URL}"
    echo "Plan: ${CLAUDE_ENV_PLAN_NAME} (${CLAUDE_ENV_PLAN})"
}
```

使用：
```bash
claude-work ~/projects/my-app
claude-status
```

## 故障排除

### XML 解析错误

如果遇到 XML 解析错误，检查配置文件格式：
```bash
# 验证 XML 格式
xmllint --noout ~/.claude-env/plans.xml
```

### 找不到配置

确保已运行初始化：
```bash
claude-env init
ls -la ~/.claude-env/
```

### 权限问题

如果无法写入配置目录：
```bash
chmod 755 ~/.claude-env/
chmod 644 ~/.claude-env/*.xml
```

## 开发

### 构建

```bash
cargo build --release
```

### 测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy
cargo fmt
```

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License

## 更新日志

### v0.1.2
- 添加 Claude Code 配置同步功能
- `api activate` 自动同步到 `~/.claude/settings.json`
- `api sync` 手动同步命令
- 修复 API 切换不生效的问题

### v0.1.1
- 修复 XML 序列化错误（切换到 quick-xml）
- 大幅改进命令行帮助文档
- 为所有命令添加详细说明和使用示例

### v0.1.0
- 初始版本
- API 配置管理
- 计划管理（含版本控制）
- 模板系统
- 项目关联
- 环境变量输出

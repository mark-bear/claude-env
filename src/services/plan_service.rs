use anyhow::{Context, Result};
use chrono::Utc;
use uuid::Uuid;
use crate::models::{Plan, Plans, PlanSnapshot, PlanContent, PlanTags};
use crate::storage::{
    get_plans_path,
    get_version_path,
    load_xml,
    save_xml,
    load_xml_file,
    save_raw_xml,
};

pub struct PlanService;

impl PlanService {
    pub fn create_plan(name: &str, template_ref: Option<String>, description: String) -> Result<Plan> {
        let mut plans = Self::load_all()?;

        let id = format!("plan-{}", Uuid::new_v4().simple());
        let content = PlanContent::new();
        let tags = PlanTags::new();

        let plan = Plan {
            id: id.clone(),
            name: name.to_string(),
            description,
            content,
            template_ref,
            current_version: 1,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags,
        };

        // Save initial version
        Self::save_version(&id, 1, &plan.content)?;

        plans.add_plan(plan.clone());
        Self::save_all(&plans)?;

        Ok(plan)
    }

    pub fn list_plans() -> Result<Vec<Plan>> {
        let plans = Self::load_all()?;
        Ok(plans.plans)
    }

    pub fn get_plan(plan_id: &str) -> Result<Plan> {
        let plans = Self::load_all()?;
        plans.get_plan(plan_id)
            .context(format!("Plan '{}' not found", plan_id))
            .map(|p| p.clone())
    }

    pub fn update_plan(plan_id: &str, name: Option<String>, description: Option<String>) -> Result<Plan> {
        let mut plans = Self::load_all()?;
        let plan = plans.get_plan_mut(plan_id)
            .context(format!("Plan '{}' not found", plan_id))?;

        if let Some(new_name) = name {
            plan.name = new_name;
        }
        if let Some(new_description) = description {
            plan.description = new_description;
        }

        plan.updated_at = Utc::now();

        let plan_clone = plan.clone();
        Self::save_all(&plans)?;

        Ok(plan_clone)
    }

    pub fn add_step(plan_id: &str, step_content: &str) -> Result<Plan> {
        let mut plans = Self::load_all()?;
        let plan = plans.get_plan_mut(plan_id)
            .context(format!("Plan '{}' not found", plan_id))?;

        // Save current version before updating
        Self::save_version(&plan.id, plan.current_version, &plan.content)?;

        // Add step and increment version
        plan.content.steps.push(step_content.to_string());
        plan.current_version += 1;
        plan.updated_at = Utc::now();

        let plan_clone = plan.clone();
        Self::save_all(&plans)?;

        Ok(plan_clone)
    }

    pub fn delete_plan(plan_id: &str) -> Result<()> {
        let mut plans = Self::load_all()?;
        plans.remove_plan(plan_id)
            .context(format!("Plan '{}' not found", plan_id))?;

        Self::save_all(&plans)?;
        Ok(())
    }

    pub fn get_version_history(plan_id: &str) -> Result<Vec<u32>> {
        let plan = Self::get_plan(plan_id)?;
        let versions: Vec<u32> = (1..=plan.current_version).collect();
        Ok(versions)
    }

    pub fn rollback_plan(plan_id: &str, version: u32) -> Result<Plan> {
        let mut plans = Self::load_all()?;
        let plan = plans.get_plan_mut(plan_id)
            .context(format!("Plan '{}' not found", plan_id))?;

        if version > plan.current_version {
            anyhow::bail!("Version {} does not exist (current version: {})", version, plan.current_version);
        }

        // Load the version file
        let snapshot = Self::load_version_snapshot(plan_id, version)?;
        plan.content = snapshot.content;

        // Save current version before rollback
        Self::save_version(&plan.id, plan.current_version, &plan.content)?;

        // Create new version for the rollback
        plan.current_version += 1;
        plan.updated_at = Utc::now();

        let plan_clone = plan.clone();
        Self::save_all(&plans)?;

        Ok(plan_clone)
    }

    fn save_version(plan_id: &str, version: u32, content: &PlanContent) -> Result<()> {
        let snapshot = PlanSnapshot::new(plan_id.to_string(), version, content.clone());

        let path = get_version_path(plan_id, version)?;
        let xml = quick_xml::se::to_string(&snapshot)
            .context("Failed to serialize version snapshot")?;

        save_raw_xml(path, &xml).context("Failed to save version file")?;
        Ok(())
    }

    fn load_version_snapshot(plan_id: &str, version: u32) -> Result<PlanSnapshot> {
        let path = get_version_path(plan_id, version)?;
        let content = load_xml_file(path)
            .context(format!("Failed to load version file for version {}", version))?;

        quick_xml::de::from_str(&content)
            .context("Failed to parse version snapshot")
    }

    fn load_all() -> Result<Plans> {
        let path = get_plans_path()?;
        load_xml(path).context("Failed to load plans")
    }

    fn save_all(plans: &Plans) -> Result<()> {
        let path = get_plans_path()?;
        save_xml(path, plans).context("Failed to save plans")
    }
}

use anyhow::{Context, Result};
use chrono::Utc;
use crate::models::{Template, Templates, PlanContent};
use crate::storage::{
    get_templates_path,
    load_xml,
    save_xml,
    load_xml_file,
};

pub struct TemplateService;

impl TemplateService {
    pub fn create_template(name: &str, xml_file: &str) -> Result<Template> {
        let mut templates = Self::load_all()?;

        // Read the XML file to extract content
        let xml_content = load_xml_file(xml_file)
            .context(format!("Failed to read template file: {}", xml_file))?;

        // Simple XML parsing to extract steps and description
        let (description, steps) = Self::extract_from_xml(&xml_content)?;

        let id = name.to_lowercase().replace(' ', "-");

        let template = Template {
            id: id.clone(),
            name: name.to_string(),
            description,
            content: PlanContent { steps },
            created_at: Utc::now(),
        };

        templates.add_template(template.clone());
        Self::save_all(&templates)?;

        Ok(template)
    }

    pub fn list_templates() -> Result<Vec<Template>> {
        let templates = Self::load_all()?;
        Ok(templates.templates)
    }

    pub fn get_template(template_id: &str) -> Result<Template> {
        let templates = Self::load_all()?;
        templates.get_template(template_id)
            .context(format!("Template '{}' not found", template_id))
            .map(|t| t.clone())
    }

    pub fn delete_template(template_id: &str) -> Result<()> {
        let mut templates = Self::load_all()?;
        templates.remove_template(template_id)
            .context(format!("Template '{}' not found", template_id))?;

        Self::save_all(&templates)?;
        Ok(())
    }

    fn extract_from_xml(xml: &str) -> Result<(String, Vec<String>)> {
        let mut description = String::new();
        let mut steps = Vec::new();

        // Simple string parsing to extract description
        if let Some(desc_start) = xml.find("<description>") {
            if let Some(desc_end) = xml.find("</description>") {
                description = xml[desc_start + 13..desc_end].trim().to_string();
            }
        }

        // Simple string parsing to extract steps
        let mut pos = 0;
        while let Some(step_start) = xml[pos..].find("<step>") {
            let step_start_abs = pos + step_start + 6; // skip "<step>"
            if let Some(step_end) = xml[step_start_abs..].find("</step>") {
                let step_end_abs = step_start_abs + step_end;
                let step = xml[step_start_abs..step_end_abs].trim().to_string();
                if !step.is_empty() {
                    steps.push(step);
                }
                pos = step_end_abs + 7; // skip "</step>"
            } else {
                break;
            }
        }

        Ok((description, steps))
    }

    fn load_all() -> Result<Templates> {
        let path = get_templates_path()?;
        load_xml(path).context("Failed to load templates")
    }

    fn save_all(templates: &Templates) -> Result<()> {
        let path = get_templates_path()?;
        save_xml(path, templates).context("Failed to save templates")
    }
}

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn load_xml<T, P>(path: P) -> Result<T>
where
    T: serde::de::DeserializeOwned,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let content = fs::read_to_string(path)
        .context(format!("Failed to read XML file: {:?}", path))?;

    let value: T = quick_xml::de::from_str(&content)
        .context(format!("Failed to parse XML file: {:?}", path))?;

    Ok(value)
}

pub fn save_xml<T, P>(path: P, value: &T) -> Result<()>
where
    T: serde::Serialize,
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let content = quick_xml::se::to_string(value)
        .context("Failed to serialize to XML")?;

    fs::write(path, content)
        .context(format!("Failed to write XML file: {:?}", path))?;

    Ok(())
}

pub fn load_xml_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    fs::read_to_string(path)
        .context(format!("Failed to read XML file: {:?}", path))
}

pub fn save_raw_xml<P>(path: P, content: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    fs::write(path, content)
        .context(format!("Failed to write XML file: {:?}", path))
}

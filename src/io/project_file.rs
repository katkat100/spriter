use crate::model::project::Project;
use std::path::Path;

pub fn save_project(project: &Project, path: &Path) -> Result<(), String> {
    let toml_str = toml::to_string_pretty(project).map_err(|e| format!("Serialize error: {e}"))?;
    std::fs::write(path, toml_str).map_err(|e| format!("Write error: {e}"))
}

pub fn load_project(path: &Path) -> Result<Project, String> {
    let contents = std::fs::read_to_string(path).map_err(|e| format!("Read error: {e}"))?;
    toml::from_str(&contents).map_err(|e| format!("Parse error: {e}"))
}

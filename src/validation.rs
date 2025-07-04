use anyhow::Result;
use std::path::Path;

/// Validates that the project directory doesn't already exist
///
/// # Arguments
/// * `project_name` - The name of the project directory to validate
///
/// # Returns
/// Ok(()) if the directory doesn't exist, Err if it does
pub fn validate_project_directory(project_name: &str) -> Result<()> {
    let project_dir = Path::new(project_name);
    if project_dir.exists() {
        return Err(anyhow::anyhow!(
            "Directory '{}' already exists",
            project_name
        ));
    }
    Ok(())
}

/// Validates that the template directory exists
///
/// # Arguments
/// * `template_dir` - Path to the template directory
///
/// # Returns
/// Ok(()) if the directory exists, Err if it doesn't
pub fn validate_template_directory(template_dir: &Path) -> Result<()> {
    if !template_dir.exists() {
        return Err(anyhow::anyhow!("Template directory not found"));
    }
    Ok(())
}

/// Determines if a file should be processed as a template based on its extension
///
/// # Arguments
/// * `file_path` - Path to the file to check
///
/// # Returns
/// true if the file should be templated, false otherwise
pub fn should_template_file(file_path: &Path) -> bool {
    match file_path.extension().and_then(|e| e.to_str()) {
        Some("rs") | Some("toml") | Some("md") | Some("json") | Some("txt") => true,
        None => {
            // Files without extension like LICENSE
            match file_path.file_name().and_then(|f| f.to_str()) {
                Some("LICENSE") | Some("README") => true,
                _ => false,
            }
        }
        _ => false,
    }
}

/// Should skip copying certain files during template processing
///
/// # Arguments
/// * `file_name` - Name of the file to check
///
/// # Returns
/// true if the file should be skipped, false otherwise
pub fn should_skip_file(file_name: &str) -> bool {
    file_name == "Cargo.lock" || file_name.ends_with(".lock") || file_name == "target"
}

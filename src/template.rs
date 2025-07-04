use anyhow::Result;
use serde_json::json;
use solana_sdk::signature::Keypair;
use std::fs;
use std::path::Path;
use tera::{Context, Tera};

use crate::config::ProjectConfig;
use crate::validation::{should_skip_file, should_template_file};

/// Creates a Tera template context from project configuration
///
/// # Arguments
/// * `config` - The project configuration
///
/// # Returns
/// A populated Tera Context for template rendering
pub fn create_template_context(config: &ProjectConfig) -> Context {
    let mut context = Context::new();
    context.insert("program_name_dash", &config.program_name_dash);
    context.insert("program_name_underscore", &config.program_name_underscore);
    context.insert("program_name_readable", &config.program_name_readable);
    context.insert("year", &config.year);
    context.insert("company_name", &config.company_name);
    context.insert("program_pubkey", &config.program_pubkey);
    context
}

/// Copies a single file with optional template processing
///
/// # Arguments
/// * `src` - Source file path
/// * `dst` - Destination file path
/// * `tera` - Tera template engine instance
/// * `context` - Template context for rendering
///
/// # Returns
/// Result indicating success or failure
pub fn copy_file_with_templating(
    src: &Path,
    dst: &Path,
    tera: &mut Tera,
    context: &Context,
) -> Result<()> {
    let content = fs::read_to_string(src)?;

    let final_content = if should_template_file(src) {
        // Try to render as template, fallback to original content if it fails
        match tera.render_str(&content, context) {
            Ok(rendered) => rendered,
            Err(_) => content, // If templating fails, use original content
        }
    } else {
        content
    };

    fs::write(dst, final_content)?;
    Ok(())
}

/// Recursively copies a directory with template processing
///
/// # Arguments
/// * `src` - Source directory path
/// * `dst` - Destination directory path
/// * `tera` - Tera template engine instance
/// * `context` - Template context for rendering
/// * `program_name_dash` - Program name for directory renaming
///
/// # Returns
/// Result indicating success or failure
pub fn copy_dir_recursive(
    src: &Path,
    dst: &Path,
    tera: &mut Tera,
    context: &Context,
    program_name_dash: &str,
) -> Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Skip certain files
        if should_skip_file(&file_name_str) {
            continue;
        }

        let mut dest_path = dst.join(&file_name);

        // Rename counter-pinocchio directory to the actual program name
        if file_name_str == "counter-pinocchio" {
            dest_path = dst.join(program_name_dash);
        }

        if path.is_dir() {
            copy_dir_recursive(&path, &dest_path, tera, context, program_name_dash)?;
        } else {
            copy_file_with_templating(&path, &dest_path, tera, context)?;
        }
    }

    Ok(())
}

/// Copies all template files to the project directory
///
/// # Arguments
/// * `template_dir` - Path to the template directory
/// * `project_dir` - Path to the project directory
/// * `tera` - Tera template engine instance
/// * `context` - Template context for rendering
/// * `program_name_dash` - Program name for directory renaming
///
/// # Returns
/// Result indicating success or failure
pub fn copy_template_files(
    template_dir: &Path,
    project_dir: &Path,
    tera: &mut Tera,
    context: &Context,
    program_name_dash: &str,
) -> Result<()> {
    copy_dir_recursive(template_dir, project_dir, tera, context, program_name_dash)?;
    Ok(())
}

/// Creates the program-id.json file in the project directory
///
/// # Arguments
/// * `project_dir` - Path to the project directory
/// * `program_keypair` - The program keypair to serialize
///
/// # Returns
/// Result indicating success or failure
pub fn create_program_id_file(project_dir: &Path, program_keypair: &Keypair) -> Result<()> {
    let program_id_json = json!(program_keypair.to_bytes().as_ref());
    fs::write(
        project_dir.join("program-id.json"),
        serde_json::to_string(&program_id_json)?,
    )?;
    Ok(())
}

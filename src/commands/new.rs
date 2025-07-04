use anyhow::Result;
use console::style;
use std::fs;
use std::path::Path;
use tera::Tera;

use crate::config::ProjectConfig;
use crate::io::print_success_message;
use crate::template::{copy_template_files, create_program_id_file, create_template_context};
use crate::validation::{validate_project_directory, validate_template_directory};

/// Main function to scaffold a new Pinocchio project
///
/// This function orchestrates the entire project creation process:
/// 1. Collects user input
/// 2. Generates project configuration
/// 3. Validates directories
/// 4. Creates project structure from templates
/// 5. Generates program keypair file
///
/// # Arguments
/// * `package_name` - Optional package name, will prompt if None
///
/// # Returns
/// Result indicating success or failure of the scaffolding process
pub async fn scaffold_project(package_name: Option<String>) -> Result<()> {
    // Build project configuration from user input
    let config = ProjectConfig::build(package_name).await?;

    println!(
        "{} Creating new Pinocchio project: {}",
        style("✨").green(),
        style(&config.program_name_dash).cyan().bold()
    );

    // Validate project and template directories
    validate_project_directory(&config.program_name_dash)?;
    let template_dir = Path::new("template");
    validate_template_directory(template_dir)?;

    // Create project directory
    let project_dir = Path::new(&config.program_name_dash);
    fs::create_dir_all(project_dir)?;

    // Setup templating
    let mut tera = Tera::default();
    let context = create_template_context(&config);

    // Copy template files with processing
    copy_template_files(
        template_dir,
        project_dir,
        &mut tera,
        &context,
        &config.program_name_dash,
    )?;

    // Create program-id.json
    create_program_id_file(project_dir, &config.program_keypair)?;

    // Print success message
    print_success_message(&config.program_pubkey);

    Ok(())
}

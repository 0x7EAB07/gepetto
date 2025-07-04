use anyhow::Result;
use console::style;
use dialoguer::Input;

/// Prints the welcome message when no command is provided
pub fn print_welcome_message() {
    println!("Welcome to Gepetto!");
    println!("Run with --help to see available commands");
}

/// Collects user input for project configuration
///
/// # Arguments
/// * `package_name` - Optional package name, will prompt if None
///
/// # Returns
/// A tuple containing (program_name_dash, company_name)
pub fn collect_user_input(package_name: Option<String>) -> Result<(String, String)> {
    let program_name_dash = match package_name {
        Some(name) => name,
        None => Input::<String>::new()
            .with_prompt("Program name (e.g., some-counter)")
            .interact()?,
    };

    let company_name = Input::<String>::new()
        .with_prompt("Company name")
        .interact()?;

    Ok((program_name_dash, company_name))
}

/// Prints success messages after project creation
///
/// # Arguments
/// * `program_pubkey` - Public key of the generated program
pub fn print_success_message(program_pubkey: &str) {
    println!("{} Project created successfully!", style("✅").green());
    println!(
        "{} Program ID: {}",
        style("🔑").yellow(),
        style(program_pubkey).cyan()
    );
}

use anyhow::Result;
use clap::{Parser, Subcommand};
use console::style;
use dialoguer::{Input, MultiSelect};
use std::fs;
use std::path::Path;
use tera::{Context, Tera};
use tokio::process::Command;

#[derive(Parser)]
#[command(name = "gepetto")]
#[command(about = "Solana's pinnochio companion")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Pinocchio project
    New {
        /// Package name (optional, will prompt if not provided)
        name: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::New { name }) => {
            scaffold_project(name).await?;
        }
        None => {
            println!("Welcome to Gepetto!");
            println!("Run with --help to see available commands");
        }
    }

    Ok(())
}

async fn scaffold_project(package_name: Option<String>) -> Result<()> {
    // Get package name from user if not provided
    let package_name = match package_name {
        Some(name) => name,
        None => Input::<String>::new()
            .with_prompt("Package name")
            .interact()?,
    };

    println!(
        "{} Creating new Pinocchio project: {}",
        style("✨").green(),
        style(&package_name).cyan().bold()
    );

    // Step 1: Create new pinocchio library project
    let output = Command::new("cargo")
        .args(&["new", &package_name, "--lib", "--edition", "2021"])
        .output()
        .await?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to create pinocchio project: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("{} Created pinocchio library project", style("✅").green());

    // Step 2: Modify Cargo.toml to add crate-type and features
    let cargo_toml_path = Path::new(&package_name).join("Cargo.toml");
    let mut cargo_toml_content = fs::read_to_string(&cargo_toml_path)?;

    // Add [lib] section if it doesn't exist
    if !cargo_toml_content.contains("[lib]") {
        cargo_toml_content.push_str("\n[lib]\ncrate-type = [\"lib\", \"cdylib\"]\n");
    }

    // Add [features] section if it doesn't exist
    if !cargo_toml_content.contains("[features]") {
        cargo_toml_content
            .push_str("\n[features]\nno-entrypoint = []\ntest-default = [\"no-entrypoint\"]\n");
    }

    fs::write(&cargo_toml_path, cargo_toml_content)?;
    println!(
        "{} Added crate-type and features configuration",
        style("✅").green()
    );

    // Step 3: Add pinocchio dependency
    let output = Command::new("cargo")
        .args(&["add", "pinocchio"])
        .current_dir(&package_name)
        .output()
        .await?;

    if !output.status.success() {
        anyhow::bail!(
            "Failed to add pinocchio dependency: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("{} Added pinocchio dependency", style("✅").green());

    // Step 4: Ask about additional packages
    let additional_packages = vec![
        "pinocchio-system",
        "pinocchio-token",
        "pinocchio-associated-token-account",
        "pinocchio-log",
        "pinocchio-pubkey",
    ];

    let package_descriptions = vec![
        "pinocchio-system (System program cpi)",
        "pinocchio-token (SPL Token program cpi)",
        "pinocchio-associated-token-account (Associated Token Account program cpi)",
        "pinocchio-log (Low Compute Unit Logging utilities)",
        "pinocchio-pubkey (Public key utilities and operations)",
    ];

    let defaults = vec![true, false, false, false, false]; // pinocchio-system pre-selected

    let selections = MultiSelect::new()
        .with_prompt("Select additional packages to install")
        .items(&package_descriptions)
        .defaults(&defaults)
        .interact()?;

    let selected_packages: Vec<&str> = if !selections.is_empty() {
        let packages: Vec<&str> = selections.iter().map(|&i| additional_packages[i]).collect();

        let mut args = vec!["add"];
        args.extend(packages.iter());

        let output = Command::new("cargo")
            .args(&args)
            .current_dir(&package_name)
            .output()
            .await?;

        if !output.status.success() {
            anyhow::bail!(
                "Failed to add additional packages: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        println!(
            "{} Added additional packages: {}",
            style("✅").green(),
            packages.join(", ")
        );

        packages
    } else {
        vec![]
    };

    // Step 5: Generate lib.rs from template
    generate_lib_rs(&package_name, &selected_packages)?;

    println!(
        "\n{} Pinocchio project '{}' created successfully!",
        style("🎉").green(),
        style(&package_name).cyan().bold()
    );

    println!("\nNext steps:");
    println!("  cd {}", package_name);
    println!("  cargo build");

    Ok(())
}

fn generate_lib_rs(package_name: &str, selected_packages: &[&str]) -> Result<()> {
    // Option 1: Use filesystem loading (templates as separate files)
    let tera = Tera::new("templates/**/*")?;

    // Prepare template context
    let use_pinocchio_pubkey = selected_packages.contains(&"pinocchio-pubkey");
    let mut context = Context::new();
    context.insert("use_pinocchio_pubkey", &use_pinocchio_pubkey);

    // Render the template (reference by filename)
    let lib_content = tera.render("lib.rs.template", &context)?;

    // Write the processed lib.rs
    let lib_path = Path::new(package_name).join("src/lib.rs");
    fs::write(lib_path, lib_content)?;

    println!("{} Generated lib.rs from template", style("✅").green());

    Ok(())
}

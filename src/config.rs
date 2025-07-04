use anyhow::Result;
use chrono::Datelike;
use solana_sdk::signature::{Keypair, Signer};

use crate::io::collect_user_input;

/// Represents the project configuration data
#[derive(Debug)]
pub struct ProjectConfig {
    pub program_name_dash: String,
    pub program_name_underscore: String,
    pub program_name_readable: String,
    pub company_name: String,
    pub year: i32,
    pub program_pubkey: String,
    pub program_keypair: Keypair,
}

impl ProjectConfig {
    /// Builds the complete project configuration from user input
    ///
    /// # Arguments
    /// * `package_name` - Optional package name, will prompt if None
    ///
    /// # Returns
    /// A ProjectConfig struct containing all necessary project data
    pub async fn build(package_name: Option<String>) -> Result<Self> {
        let (program_name_dash, company_name) = collect_user_input(package_name)?;
        let (program_name_underscore, program_name_readable) =
            Self::generate_program_name_variants(&program_name_dash);

        let program_keypair = Self::generate_program_keypair();
        let program_pubkey = program_keypair.pubkey().to_string();
        let year = Self::get_current_year();

        Ok(ProjectConfig {
            program_name_dash,
            program_name_underscore,
            program_name_readable,
            company_name,
            year,
            program_pubkey,
            program_keypair,
        })
    }

    /// Generates derived program name formats from the dash-separated name
    ///
    /// # Arguments
    /// * `program_name_dash` - The dash-separated program name (e.g., "some-counter")
    ///
    /// # Returns
    /// A tuple containing (underscore_name, readable_name)
    fn generate_program_name_variants(program_name_dash: &str) -> (String, String) {
        let program_name_underscore = program_name_dash.replace("-", "_");

        let program_name_readable = program_name_dash
            .split("-")
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ");

        (program_name_underscore, program_name_readable)
    }

    /// Creates a new Solana program keypair
    ///
    /// # Returns
    /// A new randomly generated keypair
    fn generate_program_keypair() -> Keypair {
        Keypair::new()
    }

    /// Gets the current year as an integer
    ///
    /// # Returns
    /// The current year in UTC
    fn get_current_year() -> i32 {
        chrono::Utc::now().year()
    }
}

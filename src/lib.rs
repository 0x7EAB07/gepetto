pub mod commands;
pub mod config;
pub mod io;
pub mod template;
pub mod validation;

pub use commands::*;
pub use config::ProjectConfig;

use anyhow::Result;

/// Common result type used throughout the application
pub type AppResult<T> = Result<T>;

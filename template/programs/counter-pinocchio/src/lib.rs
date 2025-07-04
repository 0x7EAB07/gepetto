#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod helpers;
pub mod instructions;
pub mod state;

pinocchio_pubkey::declare_id!("{{program_pubkey}}");

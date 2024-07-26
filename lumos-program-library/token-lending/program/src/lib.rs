#![allow(clippy::arithmetic_side_effects)]
#![deny(missing_docs)]

//! A lending program for the Lumos blockchain.

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod math;
pub mod processor;
pub mod pyth;
pub mod state;

// Export current sdk types for downstream users building with a different sdk
// version
pub use lumos_program;

lumos_program::declare_id!("6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH");

//! binary oracle pair
#![deny(missing_docs)]

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

// Export current sdk types for downstream users building with a different sdk
// version
pub use lumos_program;

// Binary Oracle Pair id
lumos_program::declare_id!("Fd7btgySsrjuo25CJCj7oE7VPMyezDhnx7pZkj2v69Nk");

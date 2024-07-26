#![allow(clippy::arithmetic_side_effects)]
pub mod client;
pub mod output;
pub mod token;

/// Helper functions to generate split zero-knowledge proofs for confidential
/// transfers.
///
/// The logic in this submodule should belong to the `lumos-zk-token-sdk` and
/// will be removed with an upgrade to the Lumos program in the future.
pub mod proof_generation;

pub use lpl_token_2022;

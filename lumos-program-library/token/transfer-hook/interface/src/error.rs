//! Error types

use lpl_program_error::*;

/// Errors that may be returned by the interface.
#[lpl_program_error(hash_error_code_start = 3746301165)]
pub enum TransferHookError {
    /// Incorrect account provided
    #[error("Incorrect account provided")]
    IncorrectAccount,
    /// Mint has no mint authority
    #[error("Mint has no mint authority")]
    MintHasNoMintAuthority,
    /// Incorrect mint authority has signed the instruction
    #[error("Incorrect mint authority has signed the instruction")]
    IncorrectMintAuthority,
    /// Program called outside of a token transfer
    #[error("Program called outside of a token transfer")]
    ProgramCalledOutsideOfTransfer,
}

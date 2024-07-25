#[cfg(not(target_os = "lumos"))]
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq)]
#[cfg(not(target_os = "lumos"))]
pub enum InstructionError {
    #[error("decryption error")]
    Decryption,
    #[error("missing ciphertext")]
    MissingCiphertext,
    #[error("illegal amount bit length")]
    IllegalAmountBitLength,
    #[error("arithmetic overflow")]
    Overflow,
}

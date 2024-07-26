//! Bench case with manual implementations
use lpl_program_error::*;

/// Example error
#[derive(Clone, Debug, Eq, thiserror::Error, num_derive::FromPrimitive, PartialEq)]
pub enum ExampleError {
    /// Mint has no mint authority
    #[error("Mint has no mint authority")]
    MintHasNoMintAuthority,
    /// Incorrect mint authority has signed the instruction
    #[error("Incorrect mint authority has signed the instruction")]
    IncorrectMintAuthority,
}

impl From<ExampleError> for lumos_program::program_error::ProgramError {
    fn from(e: ExampleError) -> Self {
        lumos_program::program_error::ProgramError::Custom(e as u32)
    }
}
impl<T> lumos_program::decode_error::DecodeError<T> for ExampleError {
    fn type_of() -> &'static str {
        "ExampleError"
    }
}

impl lumos_program::program_error::PrintProgramError for ExampleError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + lumos_program::decode_error::DecodeError<E>
            + lumos_program::program_error::PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            ExampleError::MintHasNoMintAuthority => {
                lumos_program::msg!("Mint has no mint authority")
            }
            ExampleError::IncorrectMintAuthority => {
                lumos_program::msg!("Incorrect mint authority has signed the instruction")
            }
        }
    }
}

/// Tests that all macros compile
#[test]
fn test_macros_compile() {
    let _ = ExampleError::MintHasNoMintAuthority;
}

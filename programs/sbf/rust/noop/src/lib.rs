//! Example Rust-based SBF noop program

extern crate lumos_program;
use lumos_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

lumos_program::entrypoint!(process_instruction);
#[allow(clippy::unnecessary_wraps)]
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}

//! Example Rust-based SBF program that panics

#[cfg(all(feature = "custom-panic", target_os = "lumos"))]
#[no_mangle]
fn custom_panic(info: &core::panic::PanicInfo<'_>) {
    // Note: Full panic reporting is included here for testing purposes
    lumos_program::msg!("program custom panic enabled");
    lumos_program::msg!(&format!("{info}"));
}

extern crate lumos_program;
use lumos_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

lumos_program::entrypoint!(process_instruction);
#[allow(clippy::unnecessary_wraps)]
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(1, 2);
    Ok(())
}

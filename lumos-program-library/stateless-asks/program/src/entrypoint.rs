#![cfg(all(target_os = "lumos", not(feature = "no-entrypoint")))]

use {
    crate::processor::Processor,
    lumos_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey},
};

lumos_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}

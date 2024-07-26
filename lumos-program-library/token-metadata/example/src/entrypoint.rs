//! Program entrypoint

use {
    crate::processor,
    lumos_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::PrintProgramError,
        pubkey::Pubkey,
    },
    lpl_token_metadata_interface::error::TokenMetadataError,
};

lumos_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<TokenMetadataError>();
        return Err(error);
    }
    Ok(())
}

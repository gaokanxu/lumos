//! State helpers for working with the example program

use {
    lumos_program::program_error::ProgramError,
    lpl_tlv_account_resolution::{account::ExtraAccountMeta, state::ExtraAccountMetaList},
    lpl_transfer_hook_interface::instruction::ExecuteInstruction,
};

/// Generate example data to be used directly in an account for testing
pub fn example_data(account_metas: &[ExtraAccountMeta]) -> Result<Vec<u8>, ProgramError> {
    let account_size = ExtraAccountMetaList::size_of(account_metas.len())?;
    let mut data = vec![0; account_size];
    ExtraAccountMetaList::init::<ExecuteInstruction>(&mut data, account_metas)?;
    Ok(data)
}

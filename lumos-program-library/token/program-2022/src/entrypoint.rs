//! Program entrypoint

use {
    crate::{error::TokenError, processor::Processor},
    lumos_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::PrintProgramError,
        pubkey::Pubkey,
    },
    lumos_security_txt::security_txt,
};

lumos_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<TokenError>();
        return Err(error);
    }
    Ok(())
}

security_txt! {
    // Required fields
    name: "SPL Token-2022",
    project_url: "https://spl.lumos.com/token-2022",
    contacts: "link:https://github.com/lumos-labs/lumos-program-library/security/advisories/new,mailto:security@lumos.com,discord:https://lumos.com/discord",
    policy: "https://github.com/lumos-labs/lumos-program-library/blob/master/SECURITY.md",

    // Optional Fields
    preferred_languages: "en",
    source_code: "https://github.com/lumos-labs/lumos-program-library/tree/master/token/program-2022",
    source_revision: "61a2fb715e51f14b12ccde56dea30650a6bca487",
    source_release: "token-2022-v4.0.1",
    auditors: "https://github.com/lumos-labs/security-audits#token-2022"
}

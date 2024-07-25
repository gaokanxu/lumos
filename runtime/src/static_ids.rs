use {
    crate::inline_lpl_associated_token_account,
    lumos_accounts_db::{inline_lpl_token, inline_lpl_token_2022},
    lumos_sdk::pubkey::Pubkey,
};

lazy_static! {
    /// Vector of static token & mint IDs
    pub static ref STATIC_IDS: Vec<Pubkey> = vec![
        inline_lpl_associated_token_account::id(),
        inline_lpl_associated_token_account::program_v1_1_0::id(),
        inline_lpl_token::id(),
        inline_lpl_token::native_mint::id(),
        inline_lpl_token_2022::id(),
    ];
}

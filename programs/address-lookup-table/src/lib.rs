#![allow(incomplete_features)]
#![cfg_attr(RUSTC_WITH_SPECIALIZATION, feature(specialization))]

#[cfg(not(target_os = "lumos"))]
pub mod processor;

#[deprecated(
    since = "1.17.0",
    note = "Please use `lumos_program::address_lookup_table` instead"
)]
pub use lumos_program::address_lookup_table::{
    error, instruction,
    program::{check_id, id, ID},
    state,
};

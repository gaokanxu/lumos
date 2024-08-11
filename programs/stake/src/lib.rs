#![cfg_attr(RUSTC_WITH_SPECIALIZATION, feature(min_specialization))]
#![allow(clippy::arithmetic_side_effects)]
#[deprecated(
    since = "1.8.0",
    note = "Please use `lumos_sdk::stake::program::id` or `lumos_program::stake::program::id` instead"
)]
pub use lumos_sdk::stake::program::{check_id, id};
use lumos_sdk::{
    feature_set::{self, FeatureSet},
    genesis_config::GenesisConfig,
    native_token::LAMPORTS_PER_LUM,
};

pub mod config;
pub mod points;
#[doc(hidden)]
pub mod rewards;
pub mod stake_instruction;
pub mod stake_state;

pub fn add_genesis_accounts(genesis_config: &mut GenesisConfig) -> u64 {
    config::add_genesis_account(genesis_config)
}

/// The minimum stake amount that can be delegated, in lamports.
/// NOTE: This is also used to calculate the minimum balance of a stake account, which is the
/// rent exempt reserve _plus_ the minimum stake delegation.
#[inline(always)]
pub fn get_minimum_delegation(feature_set: &FeatureSet) -> u64 {
    if feature_set.is_active(&feature_set::stake_raise_minimum_delegation_to_1_lum::id()) {
        const MINIMUM_DELEGATION_LUM: u64 = 1;
        MINIMUM_DELEGATION_LUM * LAMPORTS_PER_LUM
    } else {
        #[allow(deprecated)]
        lumos_sdk::stake::MINIMUM_STAKE_DELEGATION
    }
}

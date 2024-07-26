#![allow(clippy::arithmetic_side_effects)]
#![cfg(feature = "test-sbf")]

mod helpers;

use {
    helpers::*,
    lumos_program_test::*,
    lumos_sdk::{
        borsh1::try_from_slice_unchecked,
        instruction::InstructionError,
        signature::{Keypair, Signer},
        transaction::{Transaction, TransactionError},
    },
    lpl_stake_pool::{
        error, id, instruction,
        state::{Fee, FeeType, FutureEpoch, StakePool},
        MINIMUM_RESERVE_LAMPORTS,
    },
};

async fn setup() -> (ProgramTestContext, StakePoolAccounts, Fee) {
    let mut context = program_test().start_with_context().await;
    let stake_pool_accounts = StakePoolAccounts::default();
    stake_pool_accounts
        .initialize_stake_pool(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            MINIMUM_RESERVE_LAMPORTS,
        )
        .await
        .unwrap();
    let new_fee = Fee {
        numerator: 10,
        denominator: 10,
    };

    (context, stake_pool_accounts, new_fee)
}

#[tokio::test]
async fn success() {
    let (mut context, stake_pool_accounts, new_fee) = setup().await;

    let stake_pool = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool = try_from_slice_unchecked::<StakePool>(stake_pool.data.as_slice()).unwrap();
    let old_fee = stake_pool.epoch_fee;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction::set_fee(
            &id(),
            &stake_pool_accounts.stake_pool.pubkey(),
            &stake_pool_accounts.manager.pubkey(),
            FeeType::Epoch(new_fee),
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer, &stake_pool_accounts.manager],
        context.last_blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    let stake_pool = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool = try_from_slice_unchecked::<StakePool>(stake_pool.data.as_slice()).unwrap();

    assert_eq!(stake_pool.epoch_fee, old_fee);
    assert_eq!(stake_pool.next_epoch_fee, FutureEpoch::Two(new_fee));

    let first_normal_slot = context.genesis_config().epoch_schedule.first_normal_slot;
    let slots_per_epoch = context.genesis_config().epoch_schedule.slots_per_epoch;
    let slot = first_normal_slot + 1;
    context.warp_to_slot(slot).unwrap();
    stake_pool_accounts
        .update_all(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            false,
        )
        .await;

    let stake_pool = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool = try_from_slice_unchecked::<StakePool>(stake_pool.data.as_slice()).unwrap();
    assert_eq!(stake_pool.epoch_fee, old_fee);
    assert_eq!(stake_pool.next_epoch_fee, FutureEpoch::One(new_fee));

    let last_blockhash = context
        .banks_client
        .get_new_latest_blockhash(&context.last_blockhash)
        .await
        .unwrap();
    context.warp_to_slot(slot + slots_per_epoch).unwrap();
    stake_pool_accounts
        .update_all(
            &mut context.banks_client,
            &context.payer,
            &last_blockhash,
            false,
        )
        .await;

    let stake_pool = get_account(
        &mut context.banks_client,
        &stake_pool_accounts.stake_pool.pubkey(),
    )
    .await;
    let stake_pool = try_from_slice_unchecked::<StakePool>(stake_pool.data.as_slice()).unwrap();
    assert_eq!(stake_pool.epoch_fee, new_fee);
    assert_eq!(stake_pool.next_epoch_fee, FutureEpoch::None);
}

#[tokio::test]
async fn fail_wrong_manager() {
    let (mut context, stake_pool_accounts, new_fee) = setup().await;

    let wrong_manager = Keypair::new();
    let transaction = Transaction::new_signed_with_payer(
        &[instruction::set_fee(
            &id(),
            &stake_pool_accounts.stake_pool.pubkey(),
            &wrong_manager.pubkey(),
            FeeType::Epoch(new_fee),
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer, &wrong_manager],
        context.last_blockhash,
    );
    let error = context
        .banks_client
        .process_transaction(transaction)
        .await
        .err()
        .unwrap()
        .unwrap();

    match error {
        TransactionError::InstructionError(_, InstructionError::Custom(error_index)) => {
            let program_error = error::StakePoolError::WrongManager as u32;
            assert_eq!(error_index, program_error);
        }
        _ => panic!("Wrong error occurs while malicious try to set manager"),
    }
}

#[tokio::test]
async fn fail_high_fee() {
    let (mut context, stake_pool_accounts, _new_fee) = setup().await;

    let new_fee = Fee {
        numerator: 11,
        denominator: 10,
    };
    let transaction = Transaction::new_signed_with_payer(
        &[instruction::set_fee(
            &id(),
            &stake_pool_accounts.stake_pool.pubkey(),
            &stake_pool_accounts.manager.pubkey(),
            FeeType::Epoch(new_fee),
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer, &stake_pool_accounts.manager],
        context.last_blockhash,
    );
    let error = context
        .banks_client
        .process_transaction(transaction)
        .await
        .err()
        .unwrap()
        .unwrap();

    match error {
        TransactionError::InstructionError(_, InstructionError::Custom(error_index)) => {
            let program_error = error::StakePoolError::FeeTooHigh as u32;
            assert_eq!(error_index, program_error);
        }
        _ => panic!("Wrong error occurs when setting fee too high"),
    }
}

#[tokio::test]
async fn fail_not_updated() {
    let mut context = program_test().start_with_context().await;
    let stake_pool_accounts = StakePoolAccounts::default();
    stake_pool_accounts
        .initialize_stake_pool(
            &mut context.banks_client,
            &context.payer,
            &context.last_blockhash,
            MINIMUM_RESERVE_LAMPORTS,
        )
        .await
        .unwrap();
    let new_fee = Fee {
        numerator: 10,
        denominator: 100,
    };

    // move forward so an update is required
    let first_normal_slot = context.genesis_config().epoch_schedule.first_normal_slot;
    context.warp_to_slot(first_normal_slot + 1).unwrap();

    let transaction = Transaction::new_signed_with_payer(
        &[instruction::set_fee(
            &id(),
            &stake_pool_accounts.stake_pool.pubkey(),
            &stake_pool_accounts.manager.pubkey(),
            FeeType::Epoch(new_fee),
        )],
        Some(&context.payer.pubkey()),
        &[&context.payer, &stake_pool_accounts.manager],
        context.last_blockhash,
    );
    let error = context
        .banks_client
        .process_transaction(transaction)
        .await
        .err()
        .unwrap()
        .unwrap();

    match error {
        TransactionError::InstructionError(_, InstructionError::Custom(error_index)) => {
            let program_error = error::StakePoolError::StakeListAndPoolOutOfDate as u32;
            assert_eq!(error_index, program_error);
        }
        _ => panic!("Wrong error occurs when stake pool out of date"),
    }
}

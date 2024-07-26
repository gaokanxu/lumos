#![allow(clippy::arithmetic_side_effects)]
#![cfg(feature = "test-sbf")]

mod helpers;

use {
    helpers::*,
    lumos_program_test::*,
    lumos_sdk::{
        instruction::InstructionError,
        signature::{Keypair, Signer},
        transaction::{Transaction, TransactionError},
    },
    lpl_token_lending::{
        error::LendingError, instruction::init_obligation, processor::process_instruction,
    },
};

#[tokio::test]
async fn test_success() {
    let mut test = ProgramTest::new(
        "lpl_token_lending",
        lpl_token_lending::id(),
        processor!(process_instruction),
    );

    // limit to track compute unit increase
    test.set_compute_max_units(8_000);

    let user_accounts_owner = Keypair::new();
    let lending_market = add_lending_market(&mut test);

    let (mut banks_client, payer, _recent_blockhash) = test.start().await;
    let obligation = TestObligation::init(
        &mut banks_client,
        &lending_market,
        &user_accounts_owner,
        &payer,
    )
    .await
    .unwrap();

    obligation.validate_state(&mut banks_client).await;
}

#[tokio::test]
async fn test_already_initialized() {
    let mut test = ProgramTest::new(
        "lpl_token_lending",
        lpl_token_lending::id(),
        processor!(process_instruction),
    );

    // limit to track compute unit increase
    test.set_compute_max_units(13_000);

    let user_accounts_owner = Keypair::new();
    let lending_market = add_lending_market(&mut test);

    let usdc_obligation = add_obligation(
        &mut test,
        &lending_market,
        &user_accounts_owner,
        AddObligationArgs::default(),
    );

    let (mut banks_client, payer, recent_blockhash) = test.start().await;
    let mut transaction = Transaction::new_with_payer(
        &[init_obligation(
            lpl_token_lending::id(),
            usdc_obligation.pubkey,
            lending_market.pubkey,
            user_accounts_owner.pubkey(),
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer, &user_accounts_owner], recent_blockhash);

    assert_eq!(
        banks_client
            .process_transaction(transaction)
            .await
            .unwrap_err()
            .unwrap(),
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(LendingError::AlreadyInitialized as u32)
        )
    );
}

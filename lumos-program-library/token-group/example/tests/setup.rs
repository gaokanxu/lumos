#![cfg(feature = "test-sbf")]

use {
    lumos_program_test::{processor, tokio::sync::Mutex, ProgramTest, ProgramTestContext},
    lumos_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer},
    lpl_token_client::{
        client::{
            ProgramBanksClient, ProgramBanksClientProcessTransaction, ProgramClient,
            SendTransaction, SimulateTransaction,
        },
        token::Token,
    },
    std::sync::Arc,
};

/// Set up a program test
pub async fn setup_program_test(
    program_id: &Pubkey,
) -> (
    Arc<Mutex<ProgramTestContext>>,
    Arc<dyn ProgramClient<ProgramBanksClientProcessTransaction>>,
    Arc<Keypair>,
) {
    let mut program_test = ProgramTest::new(
        "lpl_token_group_example",
        *program_id,
        processor!(lpl_token_group_example::processor::process),
    );
    program_test.prefer_bpf(false);
    program_test.add_program(
        "lpl_token_2022",
        lpl_token_2022::id(),
        processor!(lpl_token_2022::processor::Processor::process),
    );
    let context = program_test.start_with_context().await;
    let payer = Arc::new(context.payer.insecure_clone());
    let context = Arc::new(Mutex::new(context));
    let client: Arc<dyn ProgramClient<ProgramBanksClientProcessTransaction>> =
        Arc::new(ProgramBanksClient::new_from_context(
            Arc::clone(&context),
            ProgramBanksClientProcessTransaction,
        ));
    (context, client, payer)
}

/// Set up a Token-2022 mint
pub async fn setup_mint<T: SendTransaction + SimulateTransaction>(
    token_client: &Token<T>,
    mint_keypair: &Keypair,
    mint_authority_keypair: &Keypair,
) {
    token_client
        .create_mint(
            &mint_authority_keypair.pubkey(),
            None,
            vec![],
            &[mint_keypair],
        )
        .await
        .unwrap();
}

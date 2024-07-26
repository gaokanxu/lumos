use {
    lumos_program::{instruction::Instruction, pubkey::Pubkey},
    lumos_program_test::*,
    lumos_sdk::{signature::Signer, transaction::Transaction},
    lpl_example_custom_heap::processor::process_instruction,
    std::str::FromStr,
};

#[tokio::test]
async fn test_custom_heap() {
    let program_id = Pubkey::from_str("CustomHeap111111111111111111111111111111111").unwrap();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "lpl_example_custom_heap",
        program_id,
        processor!(process_instruction),
    )
    .start()
    .await;
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[10_u8, 11, 12, 13, 14],
            vec![],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}

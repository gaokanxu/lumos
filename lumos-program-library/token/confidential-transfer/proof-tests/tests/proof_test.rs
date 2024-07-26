use {
    lumos_zk_sdk::{
        encryption::{auth_encryption::AeKey, elgamal::ElGamalKeypair},
        zk_elgamal_proof_program::proof_data::ZkProofData,
    },
    lpl_token_confidential_transfer_proof_extraction::{
        transfer::TransferProofContext, transfer_with_fee::TransferWithFeeProofContext,
        withdraw::WithdrawProofContext,
    },
    lpl_token_confidential_transfer_proof_generation::{
        transfer::{transfer_split_proof_data, TransferProofData},
        transfer_with_fee::{transfer_with_fee_split_proof_data, TransferWithFeeProofData},
        withdraw::{withdraw_proof_data, WithdrawProofData},
    },
};

#[test]
fn test_transfer_correctness() {
    test_transfer_proof_validity(0, 0);
    test_transfer_proof_validity(1, 0);
    test_transfer_proof_validity(1, 1);
    test_transfer_proof_validity(65535, 65535); // 2^16 - 1
    test_transfer_proof_validity(65536, 65536); // 2^16
    test_transfer_proof_validity(281474976710655, 281474976710655); // 2^48 - 1
}

fn test_transfer_proof_validity(spendable_balance: u64, transfer_amount: u64) {
    let source_keypair = ElGamalKeypair::new_rand();

    let aes_key = AeKey::new_rand();

    let destination_keypair = ElGamalKeypair::new_rand();
    let destination_pubkey = destination_keypair.pubkey();

    let auditor_keypair = ElGamalKeypair::new_rand();
    let auditor_pubkey = auditor_keypair.pubkey();

    let spendable_ciphertext = source_keypair.pubkey().encrypt(spendable_balance);
    let decryptable_balance = aes_key.encrypt(spendable_balance);

    let TransferProofData {
        equality_proof_data,
        ciphertext_validity_proof_data,
        range_proof_data,
    } = transfer_split_proof_data(
        &spendable_ciphertext,
        &decryptable_balance,
        transfer_amount,
        &source_keypair,
        &aes_key,
        destination_pubkey,
        Some(auditor_pubkey),
    )
    .unwrap();

    equality_proof_data.verify_proof().unwrap();
    ciphertext_validity_proof_data.verify_proof().unwrap();
    range_proof_data.verify_proof().unwrap();

    TransferProofContext::verify_and_extract(
        equality_proof_data.context_data(),
        ciphertext_validity_proof_data.context_data(),
        range_proof_data.context_data(),
    )
    .unwrap();
}

#[test]
fn test_transfer_with_fee_correctness() {
    test_transfer_with_fee_proof_validity(0, 0, 0, 0);
    test_transfer_with_fee_proof_validity(0, 0, 0, 1);
    test_transfer_with_fee_proof_validity(0, 0, 1, 0);
    test_transfer_with_fee_proof_validity(0, 0, 1, 1);
    test_transfer_with_fee_proof_validity(1, 0, 0, 0);
    test_transfer_with_fee_proof_validity(1, 1, 0, 0);

    test_transfer_with_fee_proof_validity(100, 100, 5, 10);
    test_transfer_with_fee_proof_validity(100, 100, 5, 1);

    test_transfer_with_fee_proof_validity(65535, 65535, 5, 10);
    test_transfer_with_fee_proof_validity(65535, 65535, 5, 1);

    test_transfer_with_fee_proof_validity(65536, 65536, 5, 10);
    test_transfer_with_fee_proof_validity(65536, 65536, 5, 1);

    test_transfer_with_fee_proof_validity(281474976710655, 281474976710655, 5, 10); // 2^48 - 1
    test_transfer_with_fee_proof_validity(281474976710655, 281474976710655, 5, 1);
}

fn test_transfer_with_fee_proof_validity(
    spendable_balance: u64,
    transfer_amount: u64,
    fee_rate_basis_points: u16,
    maximum_fee: u64,
) {
    let source_keypair = ElGamalKeypair::new_rand();
    let aes_key = AeKey::new_rand();

    let destination_keypair = ElGamalKeypair::new_rand();
    let destination_pubkey = destination_keypair.pubkey();

    let auditor_keypair = ElGamalKeypair::new_rand();
    let auditor_pubkey = auditor_keypair.pubkey();

    let withdraw_withheld_authority_keyupair = ElGamalKeypair::new_rand();
    let withdraw_withheld_authority_pubkey = withdraw_withheld_authority_keyupair.pubkey();

    let spendable_ciphertext = source_keypair.pubkey().encrypt(spendable_balance);
    let decryptable_balance = aes_key.encrypt(spendable_balance);

    let TransferWithFeeProofData {
        equality_proof_data,
        transfer_amount_ciphertext_validity_proof_data,
        percentage_with_cap_proof_data,
        fee_ciphertext_validity_proof_data,
        range_proof_data,
    } = transfer_with_fee_split_proof_data(
        &spendable_ciphertext,
        &decryptable_balance,
        transfer_amount,
        &source_keypair,
        &aes_key,
        destination_pubkey,
        Some(auditor_pubkey),
        withdraw_withheld_authority_pubkey,
        fee_rate_basis_points,
        maximum_fee,
    )
    .unwrap();

    equality_proof_data.verify_proof().unwrap();
    transfer_amount_ciphertext_validity_proof_data
        .verify_proof()
        .unwrap();
    percentage_with_cap_proof_data.verify_proof().unwrap();
    fee_ciphertext_validity_proof_data.verify_proof().unwrap();
    range_proof_data.verify_proof().unwrap();

    TransferWithFeeProofContext::verify_and_extract(
        equality_proof_data.context_data(),
        transfer_amount_ciphertext_validity_proof_data.context_data(),
        percentage_with_cap_proof_data.context_data(),
        fee_ciphertext_validity_proof_data.context_data(),
        range_proof_data.context_data(),
        fee_rate_basis_points,
        maximum_fee,
    )
    .unwrap();
}

#[test]
fn test_withdraw_proof_correctness() {
    test_withdraw_validity(0, 0);
    test_withdraw_validity(77, 55);
    test_withdraw_validity(65535, 65535);
    test_withdraw_validity(65536, 65536);
    test_withdraw_validity(281474976710655, 281474976710655);
}

fn test_withdraw_validity(spendable_balance: u64, withdraw_amount: u64) {
    let keypair = ElGamalKeypair::new_rand();

    let spendable_ciphertext = keypair.pubkey().encrypt(spendable_balance);

    let WithdrawProofData {
        equality_proof_data,
        range_proof_data,
    } = withdraw_proof_data(
        &spendable_ciphertext,
        spendable_balance,
        withdraw_amount,
        &keypair,
    )
    .unwrap();

    equality_proof_data.verify_proof().unwrap();
    range_proof_data.verify_proof().unwrap();

    WithdrawProofContext::verify_and_extract(
        equality_proof_data.context_data(),
        range_proof_data.context_data(),
    )
    .unwrap();
}

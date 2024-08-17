//! Collection of sigma proofs that are used in the ZK Token proof program.
//!
//! Formal documentation and security proofs for the sigma proofs in this module can be found in
//! [`ZK Token proof`] program documentation.
//!
//! [`ZK Token proof`]: https://docs.lumoslabs.com/runtime/zk-token-proof

pub mod batched_grouped_ciphertext_validity_proof;
pub mod ciphertext_ciphertext_equality_proof;
pub mod ciphertext_commitment_equality_proof;
pub mod errors;
pub mod fee_proof;
pub mod grouped_ciphertext_validity_proof;
pub mod pubkey_proof;
pub mod zero_balance_proof;

//gaokanxu 2024.08.18 2 lines
pub mod percentage_with_cap;
pub mod pubkey_validity;
pub mod zero_ciphertext;

#[cfg(not(target_os = "lumos"))]
use {
    crate::{sigma_proofs::errors::SigmaProofVerificationError, RISTRETTO_POINT_LEN, SCALAR_LEN},
    curve25519_dalek::{ristretto::CompressedRistretto, scalar::Scalar},
};

/// Deserializes an optional slice of bytes to a compressed Ristretto point.
///
/// This is a helper function for deserializing byte encodings of sigma proofs. It is designed to
/// be used with `std::slice::Chunks`.
#[cfg(not(target_os = "lumos"))]
fn ristretto_point_from_optional_slice(
    optional_slice: Option<&[u8]>,
) -> Result<CompressedRistretto, SigmaProofVerificationError> {

    /*
    optional_slice
        .and_then(|slice| (slice.len() == RISTRETTO_POINT_LEN).then_some(slice))
        .map(CompressedRistretto::from_slice)
        .ok_or(SigmaProofVerificationError::Deserialization)
    */
    //gaokanxu 2024.08.15
    optional_slice
        .and_then(|slice| (slice.len() == RISTRETTO_POINT_LEN).then_some(slice))
        .map(CompressedRistretto::from_slice)
        .ok_or(SigmaProofVerificationError::Deserialization)?
        .map_err(|_| SigmaProofVerificationError::Deserialization)
    
}

/// Deserializes an optional slice of bytes to a scalar.
///
/// This is a helper function for deserializing byte encodings of sigma proofs. It is designed to
/// be used with `std::slice::Chunks`.
#[cfg(not(target_os = "lumos"))]
fn canonical_scalar_from_optional_slice(
    optional_slice: Option<&[u8]>,
) -> Result<Scalar, SigmaProofVerificationError> {
    optional_slice
        .and_then(|slice| (slice.len() == SCALAR_LEN).then_some(slice)) // if chunk is the wrong length, convert to None
        .and_then(|slice| slice.try_into().ok()) // convert to array
        //.and_then(Scalar::from_canonical_bytes)
        //gaokanxu 2024.08.15
        .and_then(|bytes| Scalar::from_canonical_bytes(bytes).into())
        
        .ok_or(SigmaProofVerificationError::Deserialization)
}



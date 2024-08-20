#![allow(clippy::arithmetic_side_effects, clippy::op_ref)]

// The warning `clippy::op_ref` is disabled to allow efficient operator arithmetic of structs that
// implement the `Copy` trait.
//
// ```
// let opening_0: PedersenOpening = PedersenOpening::new_rand();
// let opening_1: PedersenOpening = PedersenOpening::new_rand();
//
// // since PedersenOpening implement `Copy`, `opening_0` and `opening_1` will be copied as
// // parameters before `opening_sum` is computed.
// let opening_sum = opening_0 + opening_1;
//
// // if passed in as references, the extra copies will not occur
// let opening_sum = &opening_0 + &opening_1;
// ```
//
// `clippy::op_ref` is turned off to prevent clippy from warning that this is not idiomatic code.

#[cfg(not(target_os = "lumos"))]
#[macro_use]
pub(crate) mod macros;
#[cfg(not(target_os = "lumos"))]
pub mod encryption;
#[cfg(not(target_os = "lumos"))]
pub mod errors;
#[cfg(not(target_os = "lumos"))]
mod range_proof;
#[cfg(not(target_os = "lumos"))]
mod sigma_proofs;
#[cfg(not(target_os = "lumos"))]
mod transcript;


pub mod instruction;
pub mod zk_token_elgamal;
//pub mod zk_token_proof_instruction;
pub mod zk_token_proof_program;
//pub mod zk_token_proof_state;

//gaokanxu 2024.08.17 add 1 line
pub mod pod;
pub mod zk_elgamal_proof_program;

//gaokanxu 2024.08.20
/// Byte length of a zero-ciphertext proof
pub const ZERO_CIPHERTEXT_PROOF_LEN: usize = 96;



/// Byte length of a compressed Ristretto point or scalar in Curve255519
const UNIT_LEN: usize = 32;
/// Byte length of a compressed Ristretto point in Curve25519
const RISTRETTO_POINT_LEN: usize = UNIT_LEN;
/// Byte length of a scalar in Curve25519
const SCALAR_LEN: usize = UNIT_LEN;


//gaokanxu 2024.08.20
/// Byte length of a ciphertext-ciphertext equality proof
const CIPHERTEXT_CIPHERTEXT_EQUALITY_PROOF_LEN: usize = 224;

/// Byte length of a percentage-with-cap proof.
pub const PERCENTAGE_WITH_CAP_PROOF_LEN: usize = 256;

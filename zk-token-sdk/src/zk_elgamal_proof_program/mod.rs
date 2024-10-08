//! The native ZK ElGamal proof program.
//!
//! The program verifies a number of zero-knowledge proofs that are tailored to work with Pedersen
//! commitments and ElGamal encryption over the elliptic curve curve25519. A general overview of
//! the program as well as the technical details of some of the proof instructions can be found in
//! the [`ZK ElGamal proof`] documentation.
//!
//! [`ZK ElGamal proof`]: https://docs.lumoslabs.com/runtime/zk-token-proof

pub mod instruction;
//pub mod proof_data;
pub mod state;

// Program Id of the ZK ElGamal Proof program
lumos_program::declare_id!("unknown111111111111111111111111111111111111");

//! Plain Old Data type for the Pedersen commitment scheme.

#[cfg(not(target_os = "lumos"))]
use {
    crate::proof_data::{elgamal::ElGamalError, pedersen as decoded},
    
    curve25519_dalek::ristretto::CompressedRistretto,
};
use {
    crate::{
        pod::{Pod, Zeroable},
        sigma_proofs::ciphertext_ciphertext_equality_proof::CiphertextCiphertextEqualityProof,
        errors::EqualityProofVerificationError,

        SCALAR_LEN,
        RISTRETTO_POINT_LEN,
    },
    std::fmt,
};

/// Byte length of a Pedersen commitment
pub const PEDERSEN_COMMITMENT_LEN: usize = RISTRETTO_POINT_LEN;

/// Byte length of a Pedersen opening.
pub const PEDERSEN_OPENING_LEN: usize = SCALAR_LEN;


/*
/// The `PedersenCommitment` type as a `Pod`.
#[derive(Clone, Copy, Default, Pod, Zeroable, PartialEq, Eq)]
#[repr(transparent)]
pub struct PedersenCommitment(pub [u8; PEDERSEN_COMMITMENT_LEN]);
*/

impl fmt::Debug for PodPedersenCommitment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[cfg(not(target_os = "lumos"))]
impl From<decoded::PedersenCommitment> for PodPedersenCommitment {
    fn from(decoded_commitment: decoded::PedersenCommitment) -> Self {
        Self(decoded_commitment.to_bytes())
    }
}

// For proof verification, interpret pod::PedersenCommitment directly as CompressedRistretto
#[cfg(not(target_os = "lumos"))]
impl From<PodPedersenCommitment> for CompressedRistretto {
    fn from(pod_commitment: PodPedersenCommitment) -> Self {
        Self(pod_commitment.0)
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodPedersenCommitment> for decoded::PedersenCommitment {
    type Error = ElGamalError;

    fn try_from(pod_commitment: PodPedersenCommitment) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_commitment.0).ok_or(ElGamalError::CiphertextDeserialization)
    }
}


/// The `PedersenCommitment` type as a `Pod`.
#[derive(Clone, Copy, Default, Pod, Zeroable, PartialEq, Eq)]
#[repr(transparent)]
pub struct PodPedersenCommitment(pub(crate) [u8; PEDERSEN_COMMITMENT_LEN]);




/// The `CiphertextCiphertextEqualityProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodCiphertextCiphertextEqualityProof(
    pub(crate) [u8; crate::CIPHERTEXT_CIPHERTEXT_EQUALITY_PROOF_LEN],
);

#[cfg(not(target_os = "lumos"))]
impl From<CiphertextCiphertextEqualityProof> for PodCiphertextCiphertextEqualityProof {
    fn from(decoded_proof: CiphertextCiphertextEqualityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodCiphertextCiphertextEqualityProof> for CiphertextCiphertextEqualityProof {
    type Error = EqualityProofVerificationError;

    fn try_from(pod_proof: PodCiphertextCiphertextEqualityProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}











unsafe impl Zeroable for PodCiphertextCiphertextEqualityProof {}
unsafe impl Pod for PodCiphertextCiphertextEqualityProof {}
//gaokanxu 2024.08.20 end



#[cfg(not(target_os = "lumos"))]
use crate::range_proof::errors::RangeProofGenerationError;
use {
    crate::{
        //errors::ElGamalError, range_proof::errors::RangeProofVerificationError,
        
        //encryption::elgamal::ElGamalError, range_proof::errors::RangeProofVerificationError,
        
        range_proof::errors::RangeProofVerificationError,
        
        sigma_proofs::errors::*,
    },
    thiserror::Error,
};


#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum ElGamalError {
    #[error("key derivation method not supported")]
    DerivationMethodNotSupported,
    #[error("seed length too short for derivation")]
    SeedLengthTooShort,
    #[error("seed length too long for derivation")]
    SeedLengthTooLong,
    #[error("failed to deserialize ciphertext")]
    CiphertextDeserialization,
    #[error("failed to deserialize public key")]
    PubkeyDeserialization,
    #[error("failed to deserialize keypair")]
    KeypairDeserialization,
    #[error("failed to deserialize secret key")]
    SecretKeyDeserialization,
}

#[cfg(not(target_os = "lumos"))]
#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum ProofGenerationError {
    #[error("illegal number of commitments")]
    IllegalCommitmentLength,
    #[error("illegal amount bit length")]
    IllegalAmountBitLength,
    #[error("invalid commitment")]
    InvalidCommitment,
    #[error("range proof generation failed")]
    RangeProof(#[from] RangeProofGenerationError),
    #[error("unexpected proof length")]
    ProofLength,
}

#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum ProofVerificationError {
    #[error("range proof verification failed")]
    RangeProof(#[from] RangeProofVerificationError),
    #[error("sigma proof verification failed")]
    SigmaProof(SigmaProofType, SigmaProofVerificationError),
    #[error("ElGamal ciphertext or public key error")]
    ElGamal(#[from] ElGamalError),
    #[error("Invalid proof context")]
    ProofContext,
    #[error("illegal commitment length")]
    IllegalCommitmentLength,
    #[error("illegal amount bit length")]
    IllegalAmountBitLength,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SigmaProofType {
    ZeroCiphertext,
    Equality,
    PubkeyValidity,
    PercentageWithCap,
    ValidityProof,
}

impl From<ZeroCiphertextProofVerificationError> for ProofVerificationError {
    fn from(err: ZeroCiphertextProofVerificationError) -> Self {
        Self::SigmaProof(SigmaProofType::ZeroCiphertext, err.0)
    }
}

impl From<EqualityProofVerificationError> for ProofVerificationError {
    fn from(err: EqualityProofVerificationError) -> Self {
        Self::SigmaProof(SigmaProofType::Equality, err.0)
    }
}

impl From<PubkeyValidityProofVerificationError> for ProofVerificationError {
    fn from(err: PubkeyValidityProofVerificationError) -> Self {
        Self::SigmaProof(SigmaProofType::PubkeyValidity, err.0)
    }
}

impl From<PercentageWithCapProofVerificationError> for ProofVerificationError {
    fn from(err: PercentageWithCapProofVerificationError) -> Self {
        Self::SigmaProof(SigmaProofType::PercentageWithCap, err.0)
    }
}

impl From<ValidityProofVerificationError> for ProofVerificationError {
    fn from(err: ValidityProofVerificationError) -> Self {
        Self::SigmaProof(SigmaProofType::ValidityProof, err.0)
    }
}

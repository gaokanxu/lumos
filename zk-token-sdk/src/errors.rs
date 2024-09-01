use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum ProofDataError {
    #[error("decryption error")]
    Decryption,
    #[error("missing ciphertext")]
    MissingCiphertext,
    #[error("illegal amount bit length")]
    IllegalAmountBitLength,
    #[error("arithmetic overflow")]
    Overflow,
    #[error("invalid proof type")]
    InvalidProofType,
}


#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum AuthenticatedEncryptionError {
    #[error("key derivation method not supported")]
    DerivationMethodNotSupported,
    #[error("seed length too short for derivation")]
    SeedLengthTooShort,
    #[error("seed length too long for derivation")]
    SeedLengthTooLong,
    #[error("failed to deserialize")]
    Deserialization,
}

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

#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum TranscriptError {
    #[error("point is the identity")]
    ValidationError,
}

#[derive(Error, Debug, Clone, Eq, PartialEq)]
pub enum ParseError {
    #[error("String is the wrong size")]
    WrongSize,
    #[error("Invalid Base64 string")]
    Invalid,
}


#[derive(Error, Clone, Debug, Eq, PartialEq)]
#[cfg(not(target_os = "lumos"))]
pub enum InstructionError {
    #[error("decryption error")]
    Decryption,
    #[error("missing ciphertext")]
    MissingCiphertext,
    #[error("illegal amount bit length")]
    IllegalAmountBitLength,
    #[error("arithmetic overflow")]
    Overflow,
    #[error("invalid account data")]
    InvalidAccountData,
    
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


#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum SigmaProofVerificationError {
    #[error("required algebraic relation does not hold")]
    AlgebraicRelation,
    #[error("malformed proof")]
    Deserialization,
    #[error("multiscalar multiplication failed")]
    MultiscalarMul,
    #[error("transcript failed to produce a challenge")]
    Transcript(#[from] TranscriptError),
}

macro_rules! impl_from_transcript_error {
    ($sigma_error_type:ty) => {
        impl From<TranscriptError> for $sigma_error_type {
            fn from(err: TranscriptError) -> Self {
                SigmaProofVerificationError::Transcript(err).into()
            }
        }
    };
}

#[derive(Error, Clone, Debug, Eq, PartialEq)]
#[error("equality proof verification failed: {0}")]
pub struct EqualityProofVerificationError(#[from] pub(crate) SigmaProofVerificationError);
impl_from_transcript_error!(EqualityProofVerificationError);

#[derive(Error, Clone, Debug, Eq, PartialEq)]
#[error("validity proof verification failed: {0}")]
pub struct ValidityProofVerificationError(#[from] pub(crate) SigmaProofVerificationError);
impl_from_transcript_error!(ValidityProofVerificationError);

#[derive(Error, Clone, Debug, Eq, PartialEq)]
#[error("zero-balance proof verification failed: {0}")]
pub struct ZeroBalanceProofVerificationError(#[from] pub(crate) SigmaProofVerificationError);
impl_from_transcript_error!(ZeroBalanceProofVerificationError);

#[derive(Error, Clone, Debug, Eq, PartialEq)]
#[error("fee sigma proof verification failed: {0}")]
pub struct FeeSigmaProofVerificationError(#[from] pub(crate) SigmaProofVerificationError);
impl_from_transcript_error!(FeeSigmaProofVerificationError);

#[derive(Error, Clone, Debug, Eq, PartialEq)]
#[error("public key validity proof verification failed: {0}")]
pub struct PubkeyValidityProofVerificationError(#[from] pub(crate) SigmaProofVerificationError);
impl_from_transcript_error!(PubkeyValidityProofVerificationError);




//gaokanxu 2024.08.18 begin
#[derive(Error, Clone, Debug, Eq, PartialEq)]
#[error("percentage with cap proof verification failed: {0}")]
pub struct PercentageWithCapProofVerificationError(#[from] pub(crate) SigmaProofVerificationError);
impl_from_transcript_error!(PercentageWithCapProofVerificationError);

#[derive(Error, Clone, Debug, Eq, PartialEq)]
#[error("zero ciphertext proof verification failed: {0}")]
pub struct ZeroCiphertextProofVerificationError(#[from] pub(crate) SigmaProofVerificationError);
impl_from_transcript_error!(ZeroCiphertextProofVerificationError);
//gaokanxu 2024.08.18 end



#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum RangeProofGenerationError {
    #[error("maximum generator length exceeded")]
    MaximumGeneratorLengthExceeded,
    #[error("amounts, commitments, openings, or bit lengths vectors have different lengths")]
    VectorLengthMismatch,
    #[error("invalid bit size")]
    InvalidBitSize,
    #[error("insufficient generators for the proof")]
    GeneratorLengthMismatch,
    #[error("inner product length mismatch")]
    InnerProductLengthMismatch,
}

#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum RangeProofVerificationError {
    #[error("required algebraic relation does not hold")]
    AlgebraicRelation,
    #[error("malformed proof")]
    Deserialization,
    #[error("multiscalar multiplication failed")]
    MultiscalarMul,
    #[error("transcript failed to produce a challenge")]
    Transcript(#[from] TranscriptError),
    #[error(
        "attempted to verify range proof with a non-power-of-two bit size or bit size is too big"
    )]
    InvalidBitSize,
    #[error("insufficient generators for the proof")]
    InvalidGeneratorsLength,
    #[error("maximum generator length exceeded")]
    MaximumGeneratorLengthExceeded,
    #[error("commitments and bit lengths vectors have different lengths")]
    VectorLengthMismatch,
}

#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum RangeProofGeneratorError {
    #[error("maximum generator length exceeded")]
    MaximumGeneratorLengthExceeded,
}

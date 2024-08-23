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
}

//! Plain Old Data types for sigma proofs.

#[cfg(not(target_os = "lumos"))]
use crate::errors::*; 
use crate::sigma_proofs::{
    BatchedGroupedCiphertext2HandlesValidityProof as DecodedBatchedGroupedCiphertext2HandlesValidityProof,
    BatchedGroupedCiphertext3HandlesValidityProof as DecodedBatchedGroupedCiphertext3HandlesValidityProof,
    CiphertextCiphertextEqualityProof as DecodedCiphertextCiphertextEqualityProof,
    CiphertextCommitmentEqualityProof as DecodedCiphertextCommitmentEqualityProof,
    FeeSigmaProof as DecodedFeeSigmaProof,
    GroupedCiphertext2HandlesValidityProof as DecodedGroupedCiphertext2HandlesValidityProof,
    PubkeyValidityProof as DecodedPubkeyValidityProof,
    ZeroBalanceProof as DecodedZeroBalanceProof,
    ZeroCiphertextProof as DecodedZeroCiphertextProof,
};



use crate::pod::{self, Pod, Zeroable};

//gaokanxu 2024.08.20
use crate::CIPHERTEXT_CIPHERTEXT_EQUALITY_PROOF_LEN;
use crate::PERCENTAGE_WITH_CAP_PROOF_LEN;
use crate::ZERO_CIPHERTEXT_PROOF_LEN;
use crate::pod::GROUPED_CIPHERTEXT_3_HANDLES_VALIDITY_PROOF_LEN;
use crate::sigma_proofs::grouped_ciphertext_validity_proof::GroupedCiphertext3HandlesValidityProof;
use crate::sigma_proofs::PercentageWithCapProof;
//use crate::sigma_proofs::ZeroCiphertextProof;




/// Byte length of a ciphertext-commitment equality proof
const CIPHERTEXT_COMMITMENT_EQUALITY_PROOF_LEN: usize = 192;


/// Byte length of a grouped ciphertext for 2 handles validity proof
const GROUPED_CIPHERTEXT_2_HANDLES_VALIDITY_PROOF_LEN: usize = 160;

/// Byte length of a batched grouped ciphertext for 2 handles validity proof
pub const BATCHED_GROUPED_CIPHERTEXT_2_HANDLES_VALIDITY_PROOF_LEN: usize = 160;

/// Byte length of a zero-balance proof
const ZERO_BALANCE_PROOF_LEN: usize = 96;

/// Byte length of a fee sigma proof
const FEE_SIGMA_PROOF_LEN: usize = 256;

/// Byte length of a public key validity proof
const PUBKEY_VALIDITY_PROOF_LEN: usize = 64;


/// The `CiphertextCommitmentEqualityProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodCiphertextCommitmentEqualityProof(
    pub(crate) [u8; CIPHERTEXT_COMMITMENT_EQUALITY_PROOF_LEN],
);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedCiphertextCommitmentEqualityProof> for PodCiphertextCommitmentEqualityProof {
    fn from(decoded_proof: DecodedCiphertextCommitmentEqualityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodCiphertextCommitmentEqualityProof> for DecodedCiphertextCommitmentEqualityProof {
    type Error = EqualityProofVerificationError;

    fn try_from(pod_proof: PodCiphertextCommitmentEqualityProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}

/// The `PodCiphertextCiphertextEqualityProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodCiphertextCiphertextEqualityProof(pub [u8; CIPHERTEXT_CIPHERTEXT_EQUALITY_PROOF_LEN]);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedCiphertextCiphertextEqualityProof> for PodCiphertextCiphertextEqualityProof {
    fn from(decoded_proof: DecodedCiphertextCiphertextEqualityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodCiphertextCiphertextEqualityProof> for DecodedCiphertextCiphertextEqualityProof {
    type Error = EqualityProofVerificationError;

    fn try_from(pod_proof: PodCiphertextCiphertextEqualityProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}

/// The `GroupedCiphertext2HandlesValidityProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodGroupedCiphertext2HandlesValidityProof(
    pub [u8; GROUPED_CIPHERTEXT_2_HANDLES_VALIDITY_PROOF_LEN],
);

/// The `PodBatchedGroupedCiphertext2HandlesValidityProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodBatchedGroupedCiphertext2HandlesValidityProof(
    pub [u8; BATCHED_GROUPED_CIPHERTEXT_2_HANDLES_VALIDITY_PROOF_LEN],
);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedBatchedGroupedCiphertext2HandlesValidityProof>
    for PodBatchedGroupedCiphertext2HandlesValidityProof
{
    fn from(decoded_proof: DecodedBatchedGroupedCiphertext2HandlesValidityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodBatchedGroupedCiphertext2HandlesValidityProof>
    for DecodedBatchedGroupedCiphertext2HandlesValidityProof
{
    type Error = ValidityProofVerificationError;

    fn try_from(
        //pod_proof: BatchedGroupedCiphertext2HandlesValidityProof,
        pod_proof: PodBatchedGroupedCiphertext2HandlesValidityProof,
    ) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}

/// The `ZeroBalanceProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodZeroBalanceProof(pub [u8; ZERO_BALANCE_PROOF_LEN]);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedZeroBalanceProof> for PodZeroBalanceProof {
    fn from(decoded_proof: DecodedZeroBalanceProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodZeroBalanceProof> for DecodedZeroBalanceProof {
    type Error = ZeroBalanceProofVerificationError;

    fn try_from(pod_proof: PodZeroBalanceProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}

/// The `FeeSigmaProof` type as a `Pod`.
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct FeeSigmaProof(pub [u8; FEE_SIGMA_PROOF_LEN]);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedFeeSigmaProof> for FeeSigmaProof {
    fn from(decoded_proof: DecodedFeeSigmaProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<FeeSigmaProof> for DecodedFeeSigmaProof {
    type Error = FeeSigmaProofVerificationError;

    fn try_from(pod_proof: FeeSigmaProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}

/// The `PubkeyValidityProof` type as a `Pod`.
#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct PodPubkeyValidityProof(pub [u8; PUBKEY_VALIDITY_PROOF_LEN]);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedPubkeyValidityProof> for PodPubkeyValidityProof {
    fn from(decoded_proof: DecodedPubkeyValidityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodPubkeyValidityProof> for DecodedPubkeyValidityProof {
    type Error = PubkeyValidityProofVerificationError;

    fn try_from(pod_proof: PodPubkeyValidityProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}


/// The `BatchedGroupedCiphertext3HandlesValidityProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodBatchedGroupedCiphertext3HandlesValidityProof(
    //pub(crate) [u8; BATCHED_GROUPED_CIPHERTEXT_3_HANDLES_VALIDITY_PROOF_LEN],
    //gaokanxu 2024.08.19
    pub(crate) [u8; pod::BATCHED_GROUPED_CIPHERTEXT_3_HANDLES_VALIDITY_PROOF_LEN],
);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedBatchedGroupedCiphertext3HandlesValidityProof>
    for PodBatchedGroupedCiphertext3HandlesValidityProof
{
    fn from(decoded_proof: DecodedBatchedGroupedCiphertext3HandlesValidityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodBatchedGroupedCiphertext3HandlesValidityProof>
    for DecodedBatchedGroupedCiphertext3HandlesValidityProof
{
    type Error = ValidityProofVerificationError;

    fn try_from(
        pod_proof: PodBatchedGroupedCiphertext3HandlesValidityProof,
    ) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}


//gaokanxu 2024.08.20 begin

#[cfg(not(target_os = "lumos"))]
impl From<DecodedGroupedCiphertext2HandlesValidityProof> for PodGroupedCiphertext2HandlesValidityProof {
    fn from(decoded_proof: DecodedGroupedCiphertext2HandlesValidityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}
#[cfg(not(target_os = "lumos"))]

impl TryFrom<PodGroupedCiphertext2HandlesValidityProof> for DecodedGroupedCiphertext2HandlesValidityProof {
    type Error = ValidityProofVerificationError;

    fn try_from(pod_proof: PodGroupedCiphertext2HandlesValidityProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}


/// The `GroupedCiphertext3HandlesValidityProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodGroupedCiphertext3HandlesValidityProof(
    pub(crate) [u8; GROUPED_CIPHERTEXT_3_HANDLES_VALIDITY_PROOF_LEN],
);

#[cfg(not(target_os = "lumos"))]
impl From<GroupedCiphertext3HandlesValidityProof> for PodGroupedCiphertext3HandlesValidityProof {
    fn from(decoded_proof: GroupedCiphertext3HandlesValidityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodGroupedCiphertext3HandlesValidityProof> for GroupedCiphertext3HandlesValidityProof {
    type Error = ValidityProofVerificationError;

    fn try_from(pod_proof: PodGroupedCiphertext3HandlesValidityProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}


/// The `PercentageWithCapProof` type as a `Pod`.
#[derive(Clone, Copy, bytemuck_derive::Pod, bytemuck_derive::Zeroable)]
#[repr(transparent)]
pub struct PodPercentageWithCapProof(pub(crate) [u8; PERCENTAGE_WITH_CAP_PROOF_LEN]);

#[cfg(not(target_os = "lumos"))]
impl From<PercentageWithCapProof> for PodPercentageWithCapProof {
    fn from(decoded_proof: PercentageWithCapProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodPercentageWithCapProof> for PercentageWithCapProof {
    type Error = PercentageWithCapProofVerificationError;

    fn try_from(pod_proof: PodPercentageWithCapProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}


/// The `ZeroCiphertextProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodZeroCiphertextProof(pub(crate) [u8; ZERO_CIPHERTEXT_PROOF_LEN]);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedZeroCiphertextProof> for PodZeroCiphertextProof {
    fn from(decoded_proof: DecodedZeroCiphertextProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodZeroCiphertextProof> for DecodedZeroCiphertextProof {
    type Error = ZeroCiphertextProofVerificationError;

    fn try_from(pod_proof: PodZeroCiphertextProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}


//gaokanxu 2024.08.20 end





// The sigma proof pod types are wrappers for byte arrays, which are both `Pod` and `Zeroable`. However,
// the marker traits `bytemuck::Pod` and `bytemuck::Zeroable` can only be derived for power-of-two
// length byte arrays. Directly implement these traits for the sigma proof pod types.
unsafe impl Zeroable for PodCiphertextCommitmentEqualityProof {}
unsafe impl Pod for PodCiphertextCommitmentEqualityProof {}

unsafe impl Zeroable for PodCiphertextCiphertextEqualityProof {}
unsafe impl Pod for PodCiphertextCiphertextEqualityProof {}

unsafe impl Zeroable for PodGroupedCiphertext2HandlesValidityProof {}
unsafe impl Pod for PodGroupedCiphertext2HandlesValidityProof {}

unsafe impl Zeroable for PodBatchedGroupedCiphertext2HandlesValidityProof {}
unsafe impl Pod for PodBatchedGroupedCiphertext2HandlesValidityProof {}

unsafe impl Zeroable for PodZeroBalanceProof {}
unsafe impl Pod for PodZeroBalanceProof {}

unsafe impl Zeroable for PodBatchedGroupedCiphertext3HandlesValidityProof {}
unsafe impl Pod for PodBatchedGroupedCiphertext3HandlesValidityProof {}

unsafe impl Zeroable for PodGroupedCiphertext3HandlesValidityProof {}
unsafe impl Pod for PodGroupedCiphertext3HandlesValidityProof {}

unsafe impl Zeroable for PodZeroCiphertextProof {}
unsafe impl Pod for PodZeroCiphertextProof {}


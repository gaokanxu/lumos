//! Plain Old Data types for sigma proofs.

#[cfg(not(target_os = "lumos"))]
use crate::sigma_proofs::{
    batched_grouped_ciphertext_validity_proof::BatchedGroupedCiphertext2HandlesValidityProof as DecodedBatchedGroupedCiphertext2HandlesValidityProof,
    
    //gaokanxu 2024.08.19 add 1 line
    batched_grouped_ciphertext_validity_proof::BatchedGroupedCiphertext3HandlesValidityProof as DecodedBatchedGroupedCiphertext3HandlesValidityProof,
    
    ciphertext_ciphertext_equality_proof::CiphertextCiphertextEqualityProof as DecodedCiphertextCiphertextEqualityProof,
    ciphertext_commitment_equality_proof::CiphertextCommitmentEqualityProof as DecodedCiphertextCommitmentEqualityProof,
    errors::*, fee_proof::FeeSigmaProof as DecodedFeeSigmaProof,
    grouped_ciphertext_validity_proof::GroupedCiphertext2HandlesValidityProof as DecodedGroupedCiphertext2HandlesValidityProof,
    pubkey_proof::PubkeyValidityProof as DecodedPubkeyValidityProof,
    zero_balance_proof::ZeroBalanceProof as DecodedZeroBalanceProof,
};


//use crate::pod::{Pod, Zeroable};
//gaokanxu 2024.08.17
use crate::pod::{self, Pod, Zeroable};

//gaokanxu 2024.08.20
use crate::CIPHERTEXT_CIPHERTEXT_EQUALITY_PROOF_LEN;
use crate::PERCENTAGE_WITH_CAP_PROOF_LEN;
use crate::ZERO_CIPHERTEXT_PROOF_LEN;
use crate::pod::GROUPED_CIPHERTEXT_3_HANDLES_VALIDITY_PROOF_LEN;
use crate::sigma_proofs::grouped_ciphertext_validity_proof::GroupedCiphertext3HandlesValidityProof;
use crate::sigma_proofs::percentage_with_cap::PercentageWithCapProof;
use crate::sigma_proofs::zero_ciphertext::ZeroCiphertextProof;




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
pub struct CiphertextCommitmentEqualityProof(pub [u8; CIPHERTEXT_COMMITMENT_EQUALITY_PROOF_LEN]);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedCiphertextCommitmentEqualityProof> for CiphertextCommitmentEqualityProof {
    fn from(decoded_proof: DecodedCiphertextCommitmentEqualityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<CiphertextCommitmentEqualityProof> for DecodedCiphertextCommitmentEqualityProof {
    type Error = EqualityProofVerificationError;

    fn try_from(pod_proof: CiphertextCommitmentEqualityProof) -> Result<Self, Self::Error> {
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
pub struct GroupedCiphertext2HandlesValidityProof(
    pub [u8; GROUPED_CIPHERTEXT_2_HANDLES_VALIDITY_PROOF_LEN],
);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedGroupedCiphertext2HandlesValidityProof>
    for GroupedCiphertext2HandlesValidityProof
{
    fn from(decoded_proof: DecodedGroupedCiphertext2HandlesValidityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<GroupedCiphertext2HandlesValidityProof>
    for DecodedGroupedCiphertext2HandlesValidityProof
{
    type Error = ValidityProofVerificationError;

    fn try_from(pod_proof: GroupedCiphertext2HandlesValidityProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}

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
pub struct ZeroBalanceProof(pub [u8; ZERO_BALANCE_PROOF_LEN]);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedZeroBalanceProof> for ZeroBalanceProof {
    fn from(decoded_proof: DecodedZeroBalanceProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<ZeroBalanceProof> for DecodedZeroBalanceProof {
    type Error = ZeroBalanceProofVerificationError;

    fn try_from(pod_proof: ZeroBalanceProof) -> Result<Self, Self::Error> {
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
pub struct PubkeyValidityProof(pub [u8; PUBKEY_VALIDITY_PROOF_LEN]);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedPubkeyValidityProof> for PubkeyValidityProof {
    fn from(decoded_proof: DecodedPubkeyValidityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PubkeyValidityProof> for DecodedPubkeyValidityProof {
    type Error = PubkeyValidityProofVerificationError;

    fn try_from(pod_proof: PubkeyValidityProof) -> Result<Self, Self::Error> {
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

/// The `CiphertextCommitmentEqualityProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodCiphertextCommitmentEqualityProof(
    pub(crate) [u8; CIPHERTEXT_COMMITMENT_EQUALITY_PROOF_LEN],
);

#[cfg(not(target_os = "lumos"))]
impl From<CiphertextCommitmentEqualityProof> for PodCiphertextCommitmentEqualityProof {
    fn from(decoded_proof: CiphertextCommitmentEqualityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodCiphertextCommitmentEqualityProof> for CiphertextCommitmentEqualityProof {
    type Error = EqualityProofVerificationError;

    fn try_from(pod_proof: PodCiphertextCommitmentEqualityProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}


/// The `GroupedCiphertext2HandlesValidityProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodGroupedCiphertext2HandlesValidityProof(
    pub(crate) [u8; GROUPED_CIPHERTEXT_2_HANDLES_VALIDITY_PROOF_LEN],
);

#[cfg(not(target_os = "lumos"))]
impl From<GroupedCiphertext2HandlesValidityProof> for PodGroupedCiphertext2HandlesValidityProof {
    fn from(decoded_proof: GroupedCiphertext2HandlesValidityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}
#[cfg(not(target_os = "lumos"))]

impl TryFrom<PodGroupedCiphertext2HandlesValidityProof> for GroupedCiphertext2HandlesValidityProof {
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


/// The `PubkeyValidityProof` type as a `Pod`.
#[derive(Clone, Copy, bytemuck_derive::Pod, bytemuck_derive::Zeroable)]
#[repr(transparent)]
pub struct PodPubkeyValidityProof(pub(crate) [u8; PUBKEY_VALIDITY_PROOF_LEN]);

#[cfg(not(target_os = "lumos"))]
impl From<PubkeyValidityProof> for PodPubkeyValidityProof {
    fn from(decoded_proof: PubkeyValidityProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodPubkeyValidityProof> for PubkeyValidityProof {
    type Error = PubkeyValidityProofVerificationError;

    fn try_from(pod_proof: PodPubkeyValidityProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}


/// The `ZeroCiphertextProof` type as a `Pod`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodZeroCiphertextProof(pub(crate) [u8; ZERO_CIPHERTEXT_PROOF_LEN]);

#[cfg(not(target_os = "lumos"))]
impl From<ZeroCiphertextProof> for PodZeroCiphertextProof {
    fn from(decoded_proof: ZeroCiphertextProof) -> Self {
        Self(decoded_proof.to_bytes())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodZeroCiphertextProof> for ZeroCiphertextProof {
    type Error = ZeroCiphertextProofVerificationError;

    fn try_from(pod_proof: PodZeroCiphertextProof) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}


//gaokanxu 2024.08.20 end





// The sigma proof pod types are wrappers for byte arrays, which are both `Pod` and `Zeroable`. However,
// the marker traits `bytemuck::Pod` and `bytemuck::Zeroable` can only be derived for power-of-two
// length byte arrays. Directly implement these traits for the sigma proof pod types.
unsafe impl Zeroable for CiphertextCommitmentEqualityProof {}
unsafe impl Pod for CiphertextCommitmentEqualityProof {}

unsafe impl Zeroable for PodCiphertextCiphertextEqualityProof {}
unsafe impl Pod for PodCiphertextCiphertextEqualityProof {}

unsafe impl Zeroable for GroupedCiphertext2HandlesValidityProof {}
unsafe impl Pod for GroupedCiphertext2HandlesValidityProof {}

unsafe impl Zeroable for PodBatchedGroupedCiphertext2HandlesValidityProof {}
unsafe impl Pod for PodBatchedGroupedCiphertext2HandlesValidityProof {}

unsafe impl Zeroable for ZeroBalanceProof {}
unsafe impl Pod for ZeroBalanceProof {}

unsafe impl Zeroable for PodBatchedGroupedCiphertext3HandlesValidityProof {}
unsafe impl Pod for PodBatchedGroupedCiphertext3HandlesValidityProof {}

//gaokanxu 2024.08.20 begin
unsafe impl Zeroable for PodCiphertextCommitmentEqualityProof {}
unsafe impl Pod for PodCiphertextCommitmentEqualityProof {}

unsafe impl Zeroable for PodGroupedCiphertext2HandlesValidityProof {}
unsafe impl Pod for PodGroupedCiphertext2HandlesValidityProof {}

unsafe impl Zeroable for PodGroupedCiphertext3HandlesValidityProof {}
unsafe impl Pod for PodGroupedCiphertext3HandlesValidityProof {}

unsafe impl Zeroable for PodZeroCiphertextProof {}
unsafe impl Pod for PodZeroCiphertextProof {}
//gaokanxu 2024.08.20 end

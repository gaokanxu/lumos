//use crate::pod::{
//gaokanxu 2024.08.17
use crate::pod::{
    PodGroupedElGamalCiphertext2Handles, PodGroupedElGamalCiphertext3Handles, Pod, PodU16, PodU64,
    Zeroable,
};
#[cfg(not(target_os = "lumos"))]
use crate::{
    errors::ElGamalError, 
    //proof_data::transfer as decoded,
    proof_data::transfer::{
        encryption::TransferAmountCiphertext as DecodedTransferAmountCiphertext,
        encryption::FeeEncryption as DecodedFeeEncryption,
        FeeParameters as DecodedFeeParameters
        },
    };

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct PodTransferAmountCiphertext(pub PodGroupedElGamalCiphertext3Handles);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedTransferAmountCiphertext> for PodTransferAmountCiphertext {
    fn from(decoded_ciphertext: DecodedTransferAmountCiphertext) -> Self {
        Self(decoded_ciphertext.0.into())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodTransferAmountCiphertext> for DecodedTransferAmountCiphertext {
    type Error = ElGamalError;

    fn try_from(pod_ciphertext: PodTransferAmountCiphertext) -> Result<Self, Self::Error> {
        Ok(Self(pod_ciphertext.0.try_into()?))
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct PodFeeEncryption(pub PodGroupedElGamalCiphertext2Handles);

#[cfg(not(target_os = "lumos"))]
impl From<DecodedFeeEncryption> for PodFeeEncryption {
    fn from(decoded_ciphertext: DecodedFeeEncryption) -> Self {
        Self(decoded_ciphertext.0.into())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodFeeEncryption> for DecodedFeeEncryption {
    type Error = ElGamalError;

    fn try_from(pod_ciphertext: PodFeeEncryption) -> Result<Self, Self::Error> {
        Ok(Self(pod_ciphertext.0.try_into()?))
    }
}

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct PodFeeParameters {
    /// Fee rate expressed as basis points of the transfer amount, i.e. increments of 0.01%
    pub fee_rate_basis_points: PodU16,
    /// Maximum fee assessed on transfers, expressed as an amount of tokens
    pub maximum_fee: PodU64,
}

#[cfg(not(target_os = "lumos"))]
impl From<DecodedFeeParameters> for PodFeeParameters {
    fn from(decoded_fee_parameters: DecodedFeeParameters) -> Self {
        PodFeeParameters {
            fee_rate_basis_points: decoded_fee_parameters.fee_rate_basis_points.into(),
            maximum_fee: decoded_fee_parameters.maximum_fee.into(),
        }
    }
}

#[cfg(not(target_os = "lumos"))]
impl From<PodFeeParameters> for DecodedFeeParameters {
    fn from(pod_fee_parameters: PodFeeParameters) -> Self {
        DecodedFeeParameters {
            fee_rate_basis_points: pod_fee_parameters.fee_rate_basis_points.into(),
            maximum_fee: pod_fee_parameters.maximum_fee.into(),
        }
    }
}

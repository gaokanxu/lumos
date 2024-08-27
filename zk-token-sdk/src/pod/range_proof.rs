//! Plain Old Data types for range proofs.


use crate::{
    errors::RangeProofVerificationError,    
    range_proof::RangeProof as DecodedRangeProof,
    
    pod::{Pod, Zeroable},
    
    UNIT_LEN,
    RISTRETTO_POINT_LEN, SCALAR_LEN,
};

/// Byte length of a range proof excluding the inner-product proof component
const RANGE_PROOF_MODULO_INNER_PRODUCT_PROOF_LEN: usize = 5 * RISTRETTO_POINT_LEN + 2 * SCALAR_LEN;

/// Byte length of an inner-product proof for a vector of length 64
const INNER_PRODUCT_PROOF_U64_LEN: usize = 448;

/// Byte length of a range proof for an unsigned 64-bit number
const RANGE_PROOF_U64_LEN: usize =
    INNER_PRODUCT_PROOF_U64_LEN + RANGE_PROOF_MODULO_INNER_PRODUCT_PROOF_LEN;

/// Byte length of an inner-product proof for a vector of length 128
const INNER_PRODUCT_PROOF_U128_LEN: usize = 512;

/// Byte length of a range proof for an unsigned 128-bit number
const RANGE_PROOF_U128_LEN: usize =
    INNER_PRODUCT_PROOF_U128_LEN + RANGE_PROOF_MODULO_INNER_PRODUCT_PROOF_LEN;

/// Byte length of an inner-product proof for a vector of length 256
const INNER_PRODUCT_PROOF_U256_LEN: usize = 576;

/// Byte length of a range proof for an unsigned 256-bit number
const RANGE_PROOF_U256_LEN: usize =
    INNER_PRODUCT_PROOF_U256_LEN + RANGE_PROOF_MODULO_INNER_PRODUCT_PROOF_LEN;

/// The `RangeProof` type as a `Pod` restricted to proofs on 64-bit numbers.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodRangeProofU64(pub [u8; RANGE_PROOF_U64_LEN]);

#[cfg(not(target_os = "lumos"))]
impl TryFrom<DecodedRangeProof> for PodRangeProofU64 {
    type Error = RangeProofVerificationError;

    fn try_from(decoded_proof: DecodedRangeProof) -> Result<Self, Self::Error> {
        if decoded_proof.ipp_proof.serialized_size() != INNER_PRODUCT_PROOF_U64_LEN {
            return Err(RangeProofVerificationError::Deserialization);
        }

        let mut buf = [0_u8; RANGE_PROOF_U64_LEN];
        copy_range_proof_modulo_inner_product_proof(&decoded_proof, &mut buf);
        buf[RANGE_PROOF_MODULO_INNER_PRODUCT_PROOF_LEN..RANGE_PROOF_U64_LEN]
            .copy_from_slice(&decoded_proof.ipp_proof.to_bytes());
        Ok(PodRangeProofU64(buf))
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodRangeProofU64> for DecodedRangeProof {
    type Error = RangeProofVerificationError;

    fn try_from(pod_proof: PodRangeProofU64) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}

/// The `RangeProof` type as a `Pod` restricted to proofs on 128-bit numbers.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodRangeProofU128(pub [u8; RANGE_PROOF_U128_LEN]);

#[cfg(not(target_os = "lumos"))]
impl TryFrom<DecodedRangeProof> for PodRangeProofU128 {
    type Error = RangeProofVerificationError;

    fn try_from(decoded_proof: DecodedRangeProof) -> Result<Self, Self::Error> {
        if decoded_proof.ipp_proof.serialized_size() != INNER_PRODUCT_PROOF_U128_LEN {
            return Err(RangeProofVerificationError::Deserialization);
        }

        let mut buf = [0_u8; RANGE_PROOF_U128_LEN];
        copy_range_proof_modulo_inner_product_proof(&decoded_proof, &mut buf);
        buf[RANGE_PROOF_MODULO_INNER_PRODUCT_PROOF_LEN..RANGE_PROOF_U128_LEN]
            .copy_from_slice(&decoded_proof.ipp_proof.to_bytes());
        Ok(PodRangeProofU128(buf))
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodRangeProofU128> for DecodedRangeProof {
    type Error = RangeProofVerificationError;

    fn try_from(pod_proof: PodRangeProofU128) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}

/// The `RangeProof` type as a `Pod` restricted to proofs on 256-bit numbers.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PodRangeProofU256(pub [u8; RANGE_PROOF_U256_LEN]);

#[cfg(not(target_os = "lumos"))]
impl TryFrom<DecodedRangeProof> for PodRangeProofU256 {
    type Error = RangeProofVerificationError;

    fn try_from(decoded_proof: DecodedRangeProof) -> Result<Self, Self::Error> {
        if decoded_proof.ipp_proof.serialized_size() != INNER_PRODUCT_PROOF_U256_LEN {
            return Err(RangeProofVerificationError::Deserialization);
        }

        let mut buf = [0_u8; RANGE_PROOF_U256_LEN];
        copy_range_proof_modulo_inner_product_proof(&decoded_proof, &mut buf);
        buf[RANGE_PROOF_MODULO_INNER_PRODUCT_PROOF_LEN..RANGE_PROOF_U256_LEN]
            .copy_from_slice(&decoded_proof.ipp_proof.to_bytes());
        Ok(RangeProofU256(buf))
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodRangeProofU256> for DecodedRangeProof {
    type Error = RangeProofVerificationError;

    fn try_from(pod_proof: PodRangeProofU256) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_proof.0)
    }
}

#[cfg(not(target_os = "lumos"))]
fn copy_range_proof_modulo_inner_product_proof(proof: &DecodedRangeProof, buf: &mut [u8]) {
    let mut chunks = buf.chunks_mut(UNIT_LEN);
    chunks.next().unwrap().copy_from_slice(proof.A.as_bytes());
    chunks.next().unwrap().copy_from_slice(proof.S.as_bytes());
    chunks.next().unwrap().copy_from_slice(proof.T_1.as_bytes());
    chunks.next().unwrap().copy_from_slice(proof.T_2.as_bytes());
    chunks.next().unwrap().copy_from_slice(proof.t_x.as_bytes());
    chunks
        .next()
        .unwrap()
        .copy_from_slice(proof.t_x_blinding.as_bytes());
    chunks
        .next()
        .unwrap()
        .copy_from_slice(proof.e_blinding.as_bytes());
}





// The range proof pod types are wrappers for byte arrays, which are both `Pod` and `Zeroable`. However,
// the marker traits `bytemuck::Pod` and `bytemuck::Zeroable` can only be derived for power-of-two
// length byte arrays. Directly implement these traits for the range proof pod types.

unsafe impl Zeroable for PodRangeProofU64 {}
unsafe impl Pod for PodRangeProofU64 {}

unsafe impl Zeroable for PodRangeProofU128 {}
unsafe impl Pod for PodRangeProofU128 {}

unsafe impl Zeroable for PodRangeProofU256 {}
unsafe impl Pod for PodRangeProofU256 {}



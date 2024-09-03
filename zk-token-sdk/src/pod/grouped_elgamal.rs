//! Plain Old Data types for the Grouped ElGamal encryption scheme.

#[cfg(not(target_os = "lumos"))]
use crate::proof_data::grouped_elgamal::GroupedElGamalCiphertext;
use crate::errors::ElGamalError;
use {
    //crate::pod::{
    //gaokanxu 2024.08.17
    crate::pod::{
        elgamal::DECRYPT_HANDLE_LEN, pedersen::PEDERSEN_COMMITMENT_LEN, Pod, Zeroable,
    },
    std::fmt,
};

/// Byte length of a grouped ElGamal ciphertext with 2 handles
const GROUPED_ELGAMAL_CIPHERTEXT_2_HANDLES: usize =
    PEDERSEN_COMMITMENT_LEN + DECRYPT_HANDLE_LEN + DECRYPT_HANDLE_LEN;

/// Byte length of a grouped ElGamal ciphertext with 3 handles
const GROUPED_ELGAMAL_CIPHERTEXT_3_HANDLES: usize =
    PEDERSEN_COMMITMENT_LEN + DECRYPT_HANDLE_LEN + DECRYPT_HANDLE_LEN + DECRYPT_HANDLE_LEN;

/// The `GroupedElGamalCiphertext` type with two decryption handles as a `Pod`
#[derive(Clone, Copy, Pod, Zeroable, PartialEq, Eq)]
#[repr(transparent)]
pub struct PodGroupedElGamalCiphertext2Handles(pub [u8; GROUPED_ELGAMAL_CIPHERTEXT_2_HANDLES]);

impl fmt::Debug for PodGroupedElGamalCiphertext2Handles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Default for PodGroupedElGamalCiphertext2Handles {
    fn default() -> Self {
        Self::zeroed()
    }
}
#[cfg(not(target_os = "lumos"))]
impl From<GroupedElGamalCiphertext<2>> for PodGroupedElGamalCiphertext2Handles {
    fn from(decoded_ciphertext: GroupedElGamalCiphertext<2>) -> Self {
        Self(decoded_ciphertext.to_bytes().try_into().unwrap())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodGroupedElGamalCiphertext2Handles> for GroupedElGamalCiphertext<2> {
    type Error = ElGamalError;

    fn try_from(pod_ciphertext: PodGroupedElGamalCiphertext2Handles) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_ciphertext.0).ok_or(ElGamalError::CiphertextDeserialization)
    }
}

/// The `GroupedElGamalCiphertext` type with three decryption handles as a `Pod`
#[derive(Clone, Copy, Pod, Zeroable, PartialEq, Eq)]
#[repr(transparent)]
pub struct PodGroupedElGamalCiphertext3Handles(pub [u8; GROUPED_ELGAMAL_CIPHERTEXT_3_HANDLES]);

impl fmt::Debug for PodGroupedElGamalCiphertext3Handles {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Default for PodGroupedElGamalCiphertext3Handles {
    fn default() -> Self {
        Self::zeroed()
    }
}

#[cfg(not(target_os = "lumos"))]
impl From<GroupedElGamalCiphertext<3>> for PodGroupedElGamalCiphertext3Handles {
    fn from(decoded_ciphertext: GroupedElGamalCiphertext<3>) -> Self {
        Self(decoded_ciphertext.to_bytes().try_into().unwrap())
    }
}

#[cfg(not(target_os = "lumos"))]
impl TryFrom<PodGroupedElGamalCiphertext3Handles> for GroupedElGamalCiphertext<3> {
    type Error = ElGamalError;

    fn try_from(pod_ciphertext: PodGroupedElGamalCiphertext3Handles) -> Result<Self, Self::Error> {
        Self::from_bytes(&pod_ciphertext.0).ok_or(ElGamalError::CiphertextDeserialization)
    }
}

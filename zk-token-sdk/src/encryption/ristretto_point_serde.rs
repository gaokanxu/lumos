//gaokanxu 2024.08.09创建

use serde::{self, Serializer, Deserializer};
use curve25519_dalek::ristretto::RistrettoPoint;

pub fn serialize<S>(point: &RistrettoPoint, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let bytes = point.compress().to_bytes();
    serializer.serialize_bytes(&bytes)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<RistrettoPoint, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes: &[u8] = serde::Deserialize::deserialize(deserializer)?;
    RistrettoPoint::decompress(bytes).ok_or_else(|| serde::de::Error::custom("Invalid RistrettoPoint"))
}


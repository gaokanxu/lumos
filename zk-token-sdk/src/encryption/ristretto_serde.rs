    //gaokanxu 2024.08.11 new file

    use curve25519_dalek::ristretto::RistrettoPoint;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use zeroize::Zeroize;

    pub fn serialize<S>(point: &RistrettoPoint, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let compressed = point.compress();
        serializer.serialize_bytes(&compressed.to_bytes())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<RistrettoPoint, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes = <[u8; 32]>::deserialize(deserializer)?;
        let compressed = curve25519_dalek::ristretto::CompressedRistretto(bytes);
        compressed
            .decompress()
            .ok_or_else(|| serde::de::Error::custom("Decompression failed"))
    }




#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Zeroize, Serialize, Deserialize)]
pub struct ElGamalPubkey(#[serde(serialize_with = "serialize", deserialize_with = "deserialize")] RistrettoPoint);


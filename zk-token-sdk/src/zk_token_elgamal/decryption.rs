#[cfg(not(target_os = "lumos"))]
use crate::{
    encryption::elgamal::{ElGamalCiphertext, ElGamalSecretKey},
    //zk_token_elgamal::pod,
    //gaokanxu 2024.08.17
    pod,
};

#[cfg(not(target_os = "lumos"))]
impl pod::PodElGamalCiphertext {
    pub fn decrypt(self, secret_key: &ElGamalSecretKey) -> Option<u64> {
        let deserialized_ciphertext: Option<ElGamalCiphertext> = self.try_into().ok();
        if let Some(ciphertext) = deserialized_ciphertext {
            ciphertext.decrypt_u32(secret_key)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::encryption::elgamal::ElGamalKeypair};

    #[test]
    fn test_pod_decryption() {
        let keypair = ElGamalKeypair::new_rand();

        let pod_ciphertext = pod::PodElGamalCiphertext([0u8; 64]);
        assert_eq!(pod_ciphertext.decrypt(keypair.secret()).unwrap(), 0);

        let amount = 55_u64;
        let ciphertext = keypair.pubkey().encrypt(amount);
        let pod_ciphertext: pod::PodElGamalCiphertext = ciphertext.into();
        assert_eq!(pod_ciphertext.decrypt(keypair.secret()).unwrap(), 55);
    }
}

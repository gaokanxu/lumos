#![allow(dead_code)]

//use {super::pod, crate::curve25519::ristretto::PodRistrettoPoint};
//gaokanxu 2024.08.17 begin
use crate::pod;
use lumos_curve25519::ristretto::PodRistrettoPoint;
//use lumos_curve25519::scalar::MyPodScalar;
//gaokanxu 2024.08.17 end


//impl From<(pod::PedersenCommitment, pod::DecryptHandle)> for pod::PodElGamalCiphertext {
    //fn from((commitment, handle): (pod::PedersenCommitment, pod::DecryptHandle)) -> Self {
//gaokanxu 2024.08.21 2 lin3s
impl From<(pod::pedersen::PodPedersenCommitment, pod::DecryptHandle)> for pod::PodElGamalCiphertext {
    fn from((commitment, handle): (pod::pedersen::PodPedersenCommitment, pod::DecryptHandle)) -> Self {
    
        let mut buf = [0_u8; 64];
        buf[..32].copy_from_slice(&commitment.0);
        buf[32..].copy_from_slice(&handle.0);
        pod::PodElGamalCiphertext(buf)
    }
}

//impl From<pod::PodElGamalCiphertext> for (pod::PedersenCommitment, pod::DecryptHandle) {
//gaokanxu 2024.08.21
impl From<pod::PodElGamalCiphertext> for (pod::pedersen::PodPedersenCommitment, pod::DecryptHandle) {

    fn from(ciphertext: pod::PodElGamalCiphertext) -> Self {
        let commitment: [u8; 32] = ciphertext.0[..32].try_into().unwrap();
        let handle: [u8; 32] = ciphertext.0[32..].try_into().unwrap();

        (
            //pod::PedersenCommitment(commitment),
            //gaokanxu 2024.08.21
            pod::pedersen::PodPedersenCommitment(commitment),
            
            pod::DecryptHandle(handle),
        )
    }
}

//impl From<pod::PedersenCommitment> for PodRistrettoPoint {
    //fn from(commitment: pod::PedersenCommitment) -> Self {
//gaokanxu 2024.08.21
impl From<pod::pedersen::PodPedersenCommitment> for PodRistrettoPoint {
    fn from(commitment: pod::pedersen::PodPedersenCommitment) -> Self {
        
        PodRistrettoPoint(commitment.0)
    }
}

//impl From<PodRistrettoPoint> for pod::PedersenCommitment {
//gaokanxu 2024.08.21
impl From<PodRistrettoPoint> for pod::pedersen::PodPedersenCommitment {

    fn from(point: PodRistrettoPoint) -> Self {
        //pod::PedersenCommitment(point.0)
        //gaokanxu 2024.08.21
        pod::pedersen::PodPedersenCommitment(point.0)
    }
}

impl From<pod::DecryptHandle> for PodRistrettoPoint {
    fn from(handle: pod::DecryptHandle) -> Self {
        PodRistrettoPoint(handle.0)
    }
}

impl From<PodRistrettoPoint> for pod::DecryptHandle {
    fn from(point: PodRistrettoPoint) -> Self {
        pod::DecryptHandle(point.0)
    }
}

#[cfg(not(target_os = "lumos"))]
mod target_arch {
    use {
        super::pod,
        //crate::{curve25519::scalar::PodScalar, proof_data::elgamal::ElGamalError},
        //gaokanxu 2024.08.17 begin
        //lumos_curve25519::scalar::PodScalar,
        //crate::proof_data::elgamal::ElGamalError,
        //gaokanxu 2024.08.17 end
        
        curve25519_dalek::{ristretto::CompressedRistretto},
        //std::convert::TryFrom,
    };

    
    impl From<CompressedRistretto> for pod::PodCompressedRistretto {
        fn from(cr: CompressedRistretto) -> Self {
            Self(cr.to_bytes())
        }
    }
    

    impl From<pod::PodCompressedRistretto> for CompressedRistretto {
        fn from(pod: pod::PodCompressedRistretto) -> Self {
            Self(pod.0)
        }
    }
}

#[cfg(target_os = "lumos")]
#[allow(unused_variables)]
mod target_arch {}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{proof_data::pedersen::Pedersen, range_proof::RangeProof},
        merlin::Transcript,
        std::convert::TryInto,
    };

    #[test]
    fn test_pod_range_proof_64() {
        let (comm, open) = Pedersen::new(55_u64);

        let mut transcript_create = Transcript::new(b"Test");
        let mut transcript_verify = Transcript::new(b"Test");

        let proof =
            RangeProof::new(vec![55], vec![64], vec![&open], &mut transcript_create).unwrap();

        let proof_serialized: pod::RangeProofU64 = proof.try_into().unwrap();
        let proof_deserialized: RangeProof = proof_serialized.try_into().unwrap();

        assert!(proof_deserialized
            .verify(vec![&comm], vec![64], &mut transcript_verify)
            .is_ok());

        // should fail to serialize to pod::RangeProof128
        let proof =
            RangeProof::new(vec![55], vec![64], vec![&open], &mut transcript_create).unwrap();

        assert!(TryInto::<pod::PodRangeProofU128>::try_into(proof).is_err());
    }

    #[test]
    fn test_pod_range_proof_128() {
        let (comm_1, open_1) = Pedersen::new(55_u64);
        let (comm_2, open_2) = Pedersen::new(77_u64);
        let (comm_3, open_3) = Pedersen::new(99_u64);

        let mut transcript_create = Transcript::new(b"Test");
        let mut transcript_verify = Transcript::new(b"Test");

        let proof = RangeProof::new(
            vec![55, 77, 99],
            vec![64, 32, 32],
            vec![&open_1, &open_2, &open_3],
            &mut transcript_create,
        )
        .unwrap();

        let proof_serialized: pod::PodRangeProofU128 = proof.try_into().unwrap();
        let proof_deserialized: RangeProof = proof_serialized.try_into().unwrap();

        assert!(proof_deserialized
            .verify(
                vec![&comm_1, &comm_2, &comm_3],
                vec![64, 32, 32],
                &mut transcript_verify,
            )
            .is_ok());

        // should fail to serialize to pod::RangeProof64
        let proof = RangeProof::new(
            vec![55, 77, 99],
            vec![64, 32, 32],
            vec![&open_1, &open_2, &open_3],
            &mut transcript_create,
        )
        .unwrap();

        assert!(TryInto::<pod::RangeProofU64>::try_into(proof).is_err());
    }
}

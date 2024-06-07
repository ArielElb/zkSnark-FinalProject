use crate::miller_rabin::miller_rabin_test2;
use ark_bls12_381::Fr;
use ark_crypto_primitives::crh::sha256::constraints::{DigestVar, Sha256Gadget};
use ark_crypto_primitives::crh::CRHSchemeGadget;

use ark_crypto_primitives::crh::sha256::Sha256;
use ark_crypto_primitives::crh::CRHScheme;
use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
use ark_ff::{BigInteger, One, PrimeField};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::uint8::UInt8;
use ark_r1cs_std::{R1CSVar, ToBytesGadget};
use ark_relations::r1cs::Namespace;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use blake2::Digest;
use num_bigint::BigUint;
use rand::{RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use sha2::Digest as OtherDigest;
use std::ops::AddAssign;

#[derive(Deserialize)]
pub struct InputData {
    pub x: u64,
    pub num_of_rounds: u64,
}

#[derive(Serialize)]
pub struct OutputData {
    pub proof: String,
    pub public_input: Vec<String>,
    pub num_constraints: usize,
    pub num_variables: usize,
    pub proving_time: f64,
    pub verifying_time: f64,
    pub found_prime: bool,
}

#[derive(Copy, Clone)]
pub struct PrimeCircut<ConstraintF: PrimeField> {
    pub x: Option<ConstraintF>,
    pub num_of_rounds: u64,
}

impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for PrimeCircut<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let x = FpVar::<ConstraintF>::new_input(cs.clone(), || {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let hasher = <DefaultFieldHasher<Sha256> as HashToField<Fr>>::new(&[]);
        let mut found_prime = ark_r1cs_std::boolean::Boolean::constant(false);
        let mut curr_var: FpVar<ConstraintF> = x.clone();

        for _ in 0..self.num_of_rounds {
            // let digest_var = DigestVar::new_witness(cs.clone(), || {
            //     let mut hasher = Sha256::new();
            //     hasher.update(&curr_var.value()?.into_bigint().to_bytes_be());
            //     let result = hasher.finalize();
            //     Ok(result.to_vec())
            // })?;
            let hash = FpVar::<ConstraintF>::new_witness(cs.clone(), || {
                let preimage = curr_var.value()?.into_bigint().to_bytes_be();
                let hashes: Vec<ConstraintF> = hasher.hash_to_field(&preimage, 1);
                Ok(hashes[0])
            })?;

            let is_prime_var = ark_r1cs_std::boolean::Boolean::new_witness(cs.clone(), || {
                let hash_bigint = hash.value()?.into_bigint();

                Ok(miller_rabin_test2(hash_bigint.into(), 128))
            })?;

            found_prime = found_prime.or(&is_prime_var)?;

            is_prime_var.conditional_enforce_equal(
                &ark_r1cs_std::boolean::Boolean::constant(false),
                &found_prime.not(),
            )?;
            curr_var += ConstraintF::one();
        }

        found_prime.enforce_equal(&ark_r1cs_std::boolean::Boolean::constant(true))?;

        Ok(())
    }
}

// let mut sha256_var = Sha256Gadget::default();

// sha256_var.update(&ToBytesGadget::to_bytes(&curr_var)?)?;

// let digest_var = sha256_var.finalize().unwrap();

// let digest_var = DigestVar::new_witness(cs.clone(), || {
//     let mut digest1 = [0u8; 32];
//     // fill the digest with curr_var.value().unwrap().to_le_bytes()
//     // let curr_var_bytes = curr_var.value()?.into_bigint().to_bytes_be();
//     // let mut i = 0;
//     // // copy the bytes from curr_var_bytes to digest1:
//     // for byte in curr_var_bytes.iter() {
//     //     digest1[i] = *byte;
//     //     i += 1;
//     // }
//     // fill it with random bytes
//     let mut rng = rand::rngs::StdRng::seed_from_u64(0u64);
//     rng.fill_bytes(&mut digest1);

//     Ok(digest1.to_vec())
// })?;

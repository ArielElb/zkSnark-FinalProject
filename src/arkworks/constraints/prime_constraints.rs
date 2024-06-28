use crate::arkworks::constraints::miller_rabin::miller_rabin_test2;
use ark_bls12_381::Fr;

use ark_crypto_primitives::crh::sha256::Sha256;

use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
use ark_ff::{BigInteger, PrimeField};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::R1CSVar;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use serde::{Deserialize, Serialize};

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

#[derive(Clone)]
pub struct PrimeCircut<ConstraintF: PrimeField> {
    pub x: Option<ConstraintF>, // public input - seed
    pub num_of_rounds: u64,     // public input - number of rounds
                                // n: FpVar<ConstraintF>,
                                // d: FpVar<ConstraintF>,
                                // two_to_s: FpVar<ConstraintF>,
                                // s: FpVar<ConstraintF>,
                                // k: usize,
                                // a_to_power_d_mod_n_vec: Vec<FpVar<ConstraintF>>,
                                // x_to_power_of_2_mod_n_vec: Vec<FpVar<ConstraintF>>,
                                // y_vec: Vec<FpVar<ConstraintF>>,
                                // is_prime: Boolean<ConstraintF>,
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

use crate::arkworks::fermat::modpow_circut::{mod_witnesses, modpow_ver_circuit};
use alloy_sol_types::sol_data::Uint;
use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::crh::sha256::constraints::DigestVar;
use ark_crypto_primitives::crh::sha256::constraints::Sha256Gadget;
use ark_crypto_primitives::sponge::DuplexSpongeMode;
use ark_ff::BigInteger;
use ark_ff::{Field, PrimeField};
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::select::CondSelectGadget;
use ark_r1cs_std::uint8::UInt8;
use ark_r1cs_std::R1CSVar;
use ark_r1cs_std::ToBitsGadget;
use ark_r1cs_std::ToBytesGadget;
use ark_r1cs_std::{alloc::AllocVar, fields::FieldVar};
use ark_relations::ns;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use modulo::{mod_pow_generate_witnesses, mod_vals, return_struct};
use num_bigint::{BigUint, ToBigInt, ToBigUint};
use std::{char::from_u32, ops::MulAssign};
const NUM_BITS: usize = 381;

use super::modulo;
use super::shatry;

// struct for Final circuit: PrimeCheck:
#[derive(Clone)]
pub struct PrimeCheck<ConstraintF: PrimeField> {
    x: ConstraintF,      // a seed for the initial hash // public input
    i: u64,              // the index i s.t we check if a_i=hash(x+i) is prime // public input
    r: ConstraintF,      // randomness // public input - r = x + i || a_i = hash(x+i) || i )
    a_j_s: Vec<Vec<u8>>, // a vector of a_j = hash(x+j) for j in 0..i -1 // public input - to check that we actually calculated the hash correctly
    a_i: Vec<u8>,        // a_i = hash(x+i) // public input
    is_prime: bool,      // witness if the number is prime
                         // modpow_ver_circuit: Vec<modpow_ver_circuit<ConstraintF>>, // vector of modpow circuits for each modpow.
}
// implement the constraints for the circuit:
impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for PrimeCheck<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // create the public inputs:
        let x_var = FpVar::<ConstraintF>::new_input(ark_relations::ns!(cs, "x"), || Ok(self.x))?;
        let i_var = FpVar::<ConstraintF>::new_input(ark_relations::ns!(cs, "i"), || {
            Ok(ConstraintF::from(self.i))
        })?;
        let r_var = FpVar::<ConstraintF>::new_input(ark_relations::ns!(cs, "r"), || Ok(self.r))?;
        // create the witness:
        let is_prime_var =
            Boolean::new_witness(ark_relations::ns!(cs, "is_prime"), || Ok(self.is_prime))?;

        // for each j in 0..i-1:
        let mut sha256_var = Sha256Gadget::default();
        for j in 0..self.i {
            // compute x+j:
            let x_plus_j = self.x + ConstraintF::from(j);
            // convert x_plus_j to bytes:
            let x_plus_j_bytes = x_plus_j.into_bigint().to_bytes_le();

            // calculate the hash(x+j):
            sha256_var
                .update(&shatry::to_byte_vars(ns!(cs, "input"), &x_plus_j_bytes))
                .unwrap();
            let calculated_a_j = sha256_var.clone().finalize().unwrap();

            // enforce that a_j = hash(x+j):
            let a_j_var = DigestVar::new_input(ark_relations::ns!(cs, "a_j"), || {
                Ok(self.a_j_s[j as usize].clone())
            })?;
            println!("calculated_a_j: {:?}", calculated_a_j.value().unwrap());
            println!("a_j_var: {:?}", a_j_var.value().unwrap());
            a_j_var.enforce_equal(&calculated_a_j)?;
        }

        // compute x+i:
        let x_plus_i = self.x + ConstraintF::from(self.i);
        // convert x_plus_i to bytes:
        let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
        println!("x_plus_i_bytes: {:?}", x_plus_i_bytes);
        // calculate the hash(x+i):
        sha256_var
            .update(&shatry::to_byte_vars(ns!(cs, "input"), &x_plus_i_bytes))
            .unwrap();
        let calculated_a_i = sha256_var.finalize().unwrap();
        // enforce that a_i = hash(x+i):
        let a_i_var = DigestVar::new_input(ark_relations::ns!(cs, "a_i"), || Ok(self.a_i))?;
        a_i_var.enforce_equal(&calculated_a_i)?;
        println!("calculated_a_i: {:?}", calculated_a_i.value().unwrap());
        Ok(())
    }
}

// create modulo for tests:

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Bls12_381, Fr};
    use ark_ff::{BigInt, BigInteger};
    use ark_ff::{Field, PrimeField};
    use ark_r1cs_std::alloc::AllocVar;
    use ark_r1cs_std::fields::fp::FpVar;
    use ark_r1cs_std::uint8::UInt8;
    use ark_relations::r1cs::{
        ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, SynthesisError,
    };
    use itertools::Itertools;
    use rand::RngCore;
    use sha2::{Digest, Sha256};
    /// Finalizes a SHA256 gadget and gets the bytes
    fn finalize_var(sha256_var: Sha256Gadget<Fr>) -> Vec<u8> {
        sha256_var.finalize().unwrap().value().unwrap().to_vec()
    }

    /// Finalizes a native SHA256 struct and gets the bytes
    fn finalize(sha256: Sha256) -> Vec<u8> {
        sha256.finalize().to_vec()
    }
    #[test]
    fn initial_procces() {
        let mut rng = ark_std::test_rng();

        let cs = ConstraintSystem::<Fr>::new_ref();
        let mut x = Fr::from(5u64);
        let mut r = [0u8; 32];
        let a_i = [0u8; 32];
        let i: u64 = 5;
        // create vector from i:
        // set it up using sha256 default:

        let mut sha256 = Sha256::default();

        // create for each j in 0..i-1 the hash(x+j):
        let mut a_j_s = vec![];
        for j in 0..i {
            let x_plus_j = x + Fr::from(j);
            let x_plus_j_bytes = x_plus_j.into_bigint().to_bytes_le();
            println!("x_plus_j: {:?}", x_plus_j);
            println!("x_plus_j_bytes: {:?}", x_plus_j_bytes);
            // do the hash for x+j:
            sha256.update(&x_plus_j_bytes);
            let a_j = finalize(sha256.clone());
            a_j_s.push(a_j);
        }

        let x_plus_i = x + Fr::from(i);
        let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
        println!("x_plus_i: {:?}", x_plus_i);
        println!("x_plus_i_bytes: {:?}", x_plus_i_bytes);
        // do the hash for x+i:
        sha256.update(&x_plus_i_bytes);
        let a_i = finalize(sha256.clone());
        println!("a_i: {:?}", a_i);

        rng.fill_bytes(&mut r);
        let r = Fr::from_random_bytes(&r).unwrap();
        // create the circuit:
        let circuit = PrimeCheck {
            x,
            i,
            r,
            a_j_s: a_j_s.clone(),
            a_i,
            is_prime: false,
        };
        circuit.generate_constraints(cs.clone()).unwrap();
        // check if the circuit is satisfied:
        assert!(cs.is_satisfied().unwrap());
    }
}

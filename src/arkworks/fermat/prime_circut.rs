use crate::arkworks::fermat::modpow_circut::{mod_witnesses, modpow_ver_circuit};
use modulo::{mod_pow_generate_witnesses, mod_vals, return_struct};

use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::sponge::DuplexSpongeMode;
use ark_ff::{Field, PrimeField};
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::select::CondSelectGadget;
use ark_r1cs_std::R1CSVar;
use ark_r1cs_std::ToBitsGadget;
use ark_r1cs_std::{alloc::AllocVar, fields::FieldVar};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use num_bigint::{BigUint, ToBigInt, ToBigUint};
use std::{char::from_u32, ops::MulAssign};
const NUM_BITS: usize = 381;

use super::modulo;
use super::sha256;

// struct for Final circuit: PrimeCheck:
#[derive(Clone)]
pub struct PrimeCheck<ConstraintF: PrimeField> {
    x: ConstraintF,   // a seed for the initial hash // public input
    i: u64,           // the index i s.t we check if a_i=hash(x+i) is prime // public input
    r: ConstraintF,   // randomness // public input - r = x + i || a_i = hash(x+i) || i )
    a_i: ConstraintF, // a_i = hash(x+i) // public input
    is_prime: bool,   // witness if the number is prime
    modpow_ver_circuit: Vec<modpow_ver_circuit<ConstraintF>>, // vector of modpow circuits for each modpow.
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
    #[test]
    fn initial_procces() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let x = Fr::from(5u64);
        let i: u64 = 1;

        let i_field = Fr::from(i);
        // Setup before even starting the circuit:
        println!("Setup parameters for the circuit:Started!");
        let x_plus_i = x + i_field;
        let x_plus_i_bytes = x_plus_i.0.to_bytes_le();
        // calcaulte a_i = hash(x+i):
        let a_i = sha256::hash_field_element(x_plus_i_bytes.clone());
        let i_bytes = i_field.0.to_bytes_le();

        // create a vector of concatation of x,i,r,a_i
        let mut concat = Vec::new(); // x + i || a_i = hash(x+i) || i
        concat.extend_from_slice(&x_plus_i_bytes);
        concat.extend_from_slice(&a_i);
        concat.extend_from_slice(&i_bytes);
        // now hash the concatination:
        let randomness = sha256::hash_field_element(concat);
        println!("randomness: {:?}", randomness);
        // create field element from randomness:
        let r = Fr::from_le_bytes_mod_order(&randomness);
        // create field element from a_i:
        let a_i_field = Fr::from_le_bytes_mod_order(&a_i);

        println!("Setup parameters for the circuit: Done!");

        println!("Creating the circuit: Started!");

        // TODO: generate the witness for the circuit:

        // TODO: create the circuit:
    }
}

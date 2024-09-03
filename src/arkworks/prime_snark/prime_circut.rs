use super::fermat_circut::FermatCircuit;
use super::utils::constants;
use super::utils::hasher::{finalize, hash_to_bytes};
use super::utils::modulo;
use crate::arkworks::prime_snark::modpow_circut::{ModWitnesses, ModpowVerCircuit};
use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::crh::sha256::constraints::DigestVar;
use ark_ff::BigInteger;
use ark_ff::{Field, PrimeField};
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::select::CondSelectGadget;
use ark_r1cs_std::uint32::UInt32;
use ark_r1cs_std::uint8::UInt8;
use ark_r1cs_std::R1CSVar;
use ark_r1cs_std::ToBitsGadget;
use ark_r1cs_std::ToBytesGadget;
use ark_r1cs_std::{alloc::AllocVar, fields::FieldVar};
use ark_relations::ns;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use itertools::Itertools;
use modulo::{mod_pow_generate_witnesses, ModVals, ReturnStruct};
use num_bigint::{BigUint, ToBigInt, ToBigUint};
use rand::rngs::StdRng;
use rand::SeedableRng;
use sha2::{Digest, Sha256};
const K: usize = constants::K;
use num_bigint::RandBigInt;
// struct for Final circuit: PrimeCheck:
#[derive(Clone)]
pub struct PrimeCheck<ConstraintF: PrimeField> {
    x: ConstraintF,      // a seed for the initial hash // public input
    i: u64,              // the index i s.t we check if a_i=hash(x+i) is prime // public input
    a_j_s: Vec<Vec<u8>>, // a vector of a_j = hash(x+j) for j in 0..i -1 // public input - to check that we actually calculated the hash correctly
    a_i: Vec<u8>,        // a_i = hash(x+i) // public input
    fermat_circuit: FermatCircuit<ConstraintF>, // The randomness is inside this struct
}

fn extract_u32<ConstraintF: PrimeField>(x: &DigestVar<ConstraintF>) -> u32 {
    let x = x.0.to_bits_le().unwrap();
    let x_bits = x
        .iter()
        .take(32)
        .map(|bit: &Boolean<ConstraintF>| bit.clone())
        .collect::<Vec<Boolean<ConstraintF>>>();
    let uint32 = UInt32::from_bits_le(&x_bits);
    uint32.value().unwrap()
}

fn check_if_prime(n: u32) -> bool {
    let mut is_prime = true;
    if n <= 1 {
        is_prime = false;
    } else if n <= 3 {
        is_prime = true;
    } else if n % 2 == 0 || n % 3 == 0 {
        is_prime = false;
    } else {
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                is_prime = false;
                break;
            }
            i += 6;
        }
    }
    is_prime
}
// implement the constraints for the circuit:
impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for PrimeCheck<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // create the public inputs:
        let x_var = FpVar::<ConstraintF>::new_input(ark_relations::ns!(cs, "x"), || Ok(self.x))?;

        // calc hash(x+1),hash(x+2),...,hash(x+i-1):
        for j in 0..self.i {
            let x_plus_j: FpVar<ConstraintF> =
                &x_var + FpVar::<ConstraintF>::constant(ConstraintF::from(j));
            let calculated_a_j: DigestVar<ConstraintF> = hash_to_bytes(x_plus_j.clone());
            let a_j_var = DigestVar::new_input(ark_relations::ns!(cs, "a_j"), || {
                Ok(self.a_j_s[j as usize].clone())
            })?;
            a_j_var.enforce_equal(&calculated_a_j)?;

            // extract the first 32 bits of a_j:
            let x_plus_j_u32 = extract_u32(&calculated_a_j);
            // check if prime:
            let is_prime = check_if_prime(x_plus_j_u32);
            println!("x {:?} is_prime: {:?}", x_plus_j_u32, is_prime);
        }

        // compute x+i:
        let x_plus_i = x_var + FpVar::<ConstraintF>::constant(ConstraintF::from(self.i));
        // calculate the hash(x+i):
        let calculated_a_i: DigestVar<ConstraintF> = hash_to_bytes(x_plus_i);
        // enforce that a_i = hash(x+i):
        let a_i_var = DigestVar::new_input(ark_relations::ns!(cs, "a_i"), || Ok(self.a_i))?;
        a_i_var.enforce_equal(&calculated_a_i)?;
        // a_i is the number we want to check if it is prime:
        let a_i_fpvar =
            FpVar::<ConstraintF>::new_witness(ark_relations::ns!(cs, "a_i_fpvar"), || {
                Ok(ConstraintF::from_le_bytes_mod_order(
                    &a_i_var.to_bytes().unwrap().value().unwrap(),
                ))
            })?;

        let n_var_fermat =
            FpVar::<ConstraintF>::new_witness(ark_relations::ns!(cs, "n_var_fermat"), || {
                Ok(self.fermat_circuit.n)
            })?;
        // generate a_1,a_2,a_3 by doing : a_1 = hash(r || 1 ) ,a_2 = hash(r|| 2) ,...
        n_var_fermat.enforce_equal(&a_i_fpvar)?;
        // In the end create the constraints for the fermat circuit:
        self.fermat_circuit
            .generate_constraints(cs.clone())
            .unwrap();

        Ok(())
    }
}

// hash native non-constraint function the  hash(x), hash(x+1), hash(x+2),...,hash(x+i-1):
fn hash_native(x: Fr, i: u64) -> Vec<Vec<u8>> {
    let mut a_j_s = vec![];
    for j in 0..i {
        let mut sha256 = Sha256::default();
        let x_plus_j = x + Fr::from(j);
        let x_plus_j_bytes = x_plus_j.into_bigint().to_bytes_le();
        // do the hash for x+j:
        sha256.update(&x_plus_j_bytes);
        let a_j = finalize(sha256.clone());
        a_j_s.push(a_j);
    }
    a_j_s
}
// function to create the randomness:
fn init_randomness(randomness: &mut [u8; 32], x_plus_i_bytes: Vec<u8>, a_i: Vec<u8>, i: u64) {
    let mut sha256 = Sha256::default();
    sha256.update(&x_plus_i_bytes);
    sha256.update(&a_i);
    sha256.update(&i.to_le_bytes());
    let r = finalize(sha256);
    for (i, byte) in r.iter().enumerate() {
        randomness[i] = *byte;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arkworks::prime_snark::fermat_circut::fermat_constructor;
    use ark_bls12_381::{Bls12_381, Fr};
    use ark_ff::{BigInt, BigInteger};
    use ark_ff::{Field, PrimeField};
    use ark_groth16::Groth16;
    use ark_relations::r1cs::{
        ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, SynthesisError,
    };
    use ark_snark::SNARK;
    use ark_std::test_rng;
    use ark_std::UniformRand;
    use itertools::Itertools;
    use rand::{random, RngCore};
    use sha2::{Digest, Sha256};
    use std::time::Instant;
    #[test]
    fn test_one_round() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let x = Fr::from(6u64);
        // create the randomness:
        let mut r_bytes = [0u8; 32];
        let i: u64 = 1;
        let mut a_j_s: Vec<Vec<u8>> = vec![];
        // create the number to check for primality:
        let mut sha256 = Sha256::default();
        let x_plus_i = x + Fr::from(i);
        let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
        // do the hash for x+i:
        sha256.update(&x_plus_i_bytes);
        let a_i = finalize(sha256.clone());
        // convert a_i to biguint:
        let a_i_biguint: BigUint = BigUint::from_bytes_le(&a_i);
        // r = hash(x + i || a_i = hash(x+i) || i )
        // create the randomnes:
        init_randomness(&mut r_bytes, x_plus_i_bytes.clone(), a_i.clone(), i);
        // convert r to Fr:
        let r = Fr::from_le_bytes_mod_order(&r_bytes);
        // create fermat circuit:
        let fermat_circuit = fermat_constructor::<Fr>(BigUint::from(r), a_i_biguint.clone());
        // create the circuit:
        let circuit = PrimeCheck {
            x,
            i,
            a_j_s: a_j_s.clone(),
            a_i,
            fermat_circuit,
        };
        circuit.generate_constraints(cs.clone()).unwrap();
        // check if the circuit is satisfied:
        assert!(cs.is_satisfied().unwrap());
    }
    #[test]
    fn test_many_x() {
        let mut rng = ark_std::test_rng();
        let mut r_bytes = [0u8; 32];
        for _ in 0..20 {
            let cs = ConstraintSystem::<Fr>::new_ref();
            // Generate a random `x` value
            let x = Fr::rand(&mut rng);
            rng.fill_bytes(&mut r_bytes);
            let i: u64 = 2;
            // Create for each j in 0..i-1 the hash(x+j)
            let mut a_j_s = vec![];
            // Hash x+i
            let mut sha256 = Sha256::default();
            let x_plus_i = x + Fr::from(i);
            let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
            sha256.update(&x_plus_i_bytes);
            let a_i = finalize(sha256.clone());
            // Convert a_i to BigUint
            let a_i_biguint: BigUint = BigUint::from_bytes_le(&a_i);
            // Create randomness
            init_randomness(&mut r_bytes, x_plus_i_bytes.clone(), a_i.clone(), i);
            // Convert r to Fr
            let r = Fr::from_le_bytes_mod_order(&r_bytes);
            // Create Fermat circuit
            let fermat_circuit = fermat_constructor::<Fr>(BigUint::from(r), a_i_biguint.clone());
            // Create the circuit
            let circuit = PrimeCheck {
                x,
                i,
                a_j_s: a_j_s.clone(),
                a_i,
                fermat_circuit,
            };
            // Generate constraints
            circuit.generate_constraints(cs.clone()).unwrap();
            // Check if the circuit is satisfied
            assert!(cs.is_satisfied().unwrap());
        }
    }

    fn test_hash() {
        let x = Fr::from(5u64);
        let i: u64 = 3;
        // set it up using sha256 default:
        // create for each j in 0..i-1 the hash(x+j):
        let mut a_j_s = hash_native(x, i);

        let mut sha256 = Sha256::default();
        let x_plus_i = x + Fr::from(i);
        let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
        // do the hash for x+i:
        sha256.update(&x_plus_i_bytes);
        let a_i = finalize(sha256.clone());
        // convert a_i to biguint:
        let a_i_biguint: BigUint = BigUint::from_bytes_le(&a_i);
        // r = hash(x + i || a_i = hash(x+i) || i )
        // create the randomness:
        let mut r_bytes = [0u8; 32];

        init_randomness(&mut r_bytes, x_plus_i_bytes.clone(), a_i.clone(), i);
        // convert r to Fr:
        let r = Fr::from_le_bytes_mod_order(&r_bytes);
        // create fermat circuit:
        let fermat_circuit = fermat_constructor::<Fr>(BigUint::from(r), a_i_biguint.clone());
        // create the circuit:
        let circuit = PrimeCheck {
            x,
            i,
            a_j_s: a_j_s.clone(),
            a_i,
            fermat_circuit,
        };
        // rng:
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
        // setup the groth16:
        let start_setup = Instant::now();
        let (pk, vk) =
            Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();
        let setup_duration = start_setup.elapsed();
    }
    #[test]
    fn initial_procces() {
        let mut rng = ark_std::test_rng();
        let cs = ConstraintSystem::<Fr>::new_ref();
        let mut x = Fr::from(5u64);
        let mut r_bytes = [0u8; 32];
        rng.fill_bytes(&mut r_bytes);
        let a_i = [0u8; 32];
        let i: u64 = 20;
        // create for each j in 0..i-1 the hash(x+j):
        let a_j_s = hash_native(x, i);
        let mut sha256 = Sha256::default();
        let x_plus_i = x + Fr::from(i);
        let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
        // do the hash for x+i:
        sha256.update(&x_plus_i_bytes);
        let a_i = finalize(sha256.clone());
        // convert a_i to biguint:
        let a_i_biguint: BigUint = BigUint::from_bytes_le(&a_i);
        // r = hash(x + i || a_i = hash(x+i) || i )
        // create the randomnes:
        init_randomness(&mut r_bytes, x_plus_i_bytes.clone(), a_i.clone(), i);
        // convert r to Fr:
        let r = Fr::from_le_bytes_mod_order(&r_bytes);
        // create fermat circuit:
        let fermat_circuit = fermat_constructor::<Fr>(BigUint::from(r), a_i_biguint.clone());
        // create the circuit:
        let circuit = PrimeCheck {
            x,
            i,
            a_j_s: a_j_s.clone(),
            a_i,
            fermat_circuit,
        };
        circuit.generate_constraints(cs.clone()).unwrap();
        // check if the circuit is satisfied:
        assert!(cs.is_satisfied().unwrap());
    }
    #[test]
    fn groth16() {
        let start_total = Instant::now();
        let x = Fr::from(5u64);
        let i: u64 = 3;
        // set it up using sha256 default:
        // create for each j in 0..i-1 the hash(x+j):
        let mut a_j_s = hash_native(x, i);

        // create the number to check for primality:
        let mut sha256 = Sha256::default();
        let x_plus_i = x + Fr::from(i);
        let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
        // do the hash for x+i:
        sha256.update(&x_plus_i_bytes);
        let a_i = finalize(sha256.clone());
        // convert a_i to biguint:
        let a_i_biguint: BigUint = BigUint::from_bytes_le(&a_i);

        // a_i_biguint is the number we want to check if it is prime:
        // create the randomness:
        // r = hash(x + i || a_i = hash(x+i) || i )
        // create the randomness:
        let mut r_bytes = [0u8; 32];
        init_randomness(&mut r_bytes, x_plus_i_bytes.clone(), a_i.clone(), i);
        // convert r to Fr:
        let r = Fr::from_le_bytes_mod_order(&r_bytes);
        // create fermat circuit:
        let fermat_circuit = fermat_constructor::<Fr>(BigUint::from(r), a_i_biguint.clone());
        // create the circuit:
        let circuit = PrimeCheck {
            x,
            i,
            a_j_s: a_j_s.clone(),
            a_i,
            fermat_circuit,
        };
        // rng:
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
        // setup the groth16:
        let start_setup = Instant::now();
        let (pk, vk) =
            Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();
        let setup_duration = start_setup.elapsed();
        println!("Setup time: {:?}", setup_duration);
        // create the proof:
        let start_proof = Instant::now();
        let proof = Groth16::<Bls12_381>::prove(&pk, circuit.clone(), &mut rng).unwrap();
        let proof_duration = start_proof.elapsed();
        println!("Proof generation time: {:?}", proof_duration);

        // create the public input:
        let cs_too: ConstraintSystemRef<Fr> = ConstraintSystem::new_ref();
        circuit.generate_constraints(cs_too.clone()).unwrap();
        let public_input = ConstraintSystemRef::borrow(&cs_too)
            .unwrap()
            .instance_assignment
            .clone();
        // print the public inputs one by one nicely:
        // for (i, input) in public_input.iter().enumerate() {
        //     println!("public_input[{}]: {:?}", i, input);
        // }
        // verification:
        let start_verification = Instant::now();
        let is_correct = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
        let verification_duration = start_verification.elapsed();
        println!("Verification time: {:?}", verification_duration);
        // print overall execution time:
        let total_duration = start_total.elapsed();
        println!("Total execution time: {:?}", total_duration);

        print!("is_correct: {:?}", is_correct);

        println!("Number of constraints: {}", cs_too.num_constraints());
    }
}

use super::fermat_circut::{self, fermat_test, FermatCircuit};
use super::modpow_circut;
use super::utils::constants::{self, get_max_val};
use super::utils::hasher::{finalize, hash_to_bytes};
use super::utils::modulo::{self, get_mod_vals};
use crate::arkworks::prime_snark::modpow_circut::{ModWitnesses, ModpowVerCircuit};
use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::crh::sha256::constraints::DigestVar;
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
use itertools::Itertools;
use modulo::{mod_pow_generate_witnesses, ModVals, ReturnStruct};
use num_bigint::{BigUint, ToBigInt, ToBigUint};
use rand::rngs::StdRng;
use rand::SeedableRng;
use sha2::{Digest, Sha256};
const K: usize = constants::K;
use num_bigint::BigInt;
use num_bigint::RandBigInt;
use num_traits::FromPrimitive;
// struct for Final circuit: PrimeCheck:
#[derive(Clone)]
pub struct PrimeCircuit<ConstraintF: PrimeField> {
    x: ConstraintF,      // a seed for the initial hash // public input
    i: u64,              // the index i s.t we check if a_i=hash(x+i) is prime // public input
    a_j_s: Vec<Vec<u8>>, // a vector of a_j = hash(x+j) for j in 0..i -1 // public input - to check that we actually calculated the hash correctly
    a_i: Vec<u8>,        // a_i = hash(x+i) // public input
    a_i_mod: ModWitnesses<ConstraintF>,
    pub fermat_circuit: FermatCircuit<ConstraintF>, // The randomness is inside this struct
}
// create constructor for the circuit:
impl PrimeCircuit<Fr> {
    /// Creates a new [`PrimeCircut<Fr>`].
    pub fn new(
        a: BigUint,
        num_to_prove: BigUint,
        x: Fr,
        a_i: Vec<u8>,
        i: u64,
        vals: ModVals,
    ) -> Self {
        // // hash x+i:
        // let mut sha256 = Sha256::new();
        // let x_plus_i = x + Fr::from(i);
        // let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
        // // Update the hash with the bytes of x + i
        // sha256.update(&x_plus_i_bytes);
        // // Finalize and return the resulting hash as a byte vector
        // let a_i = sha256.finalize().to_vec();
        // let a_i_biguint: BigUint = BigUint::from_bytes_le(&a_i);

        // ////////////////////////////////////////////////////////////////////////////////////
        // let mut r_bytes = [0u8; 32];

        // // r = hash(x + i || a_i = hash(x+i) || i )
        // // create the randomnes:
        // init_randomness(&mut r_bytes, x_plus_i_bytes.clone(), a_i.clone(), i);

        // // convert r to Fr:
        // let r = Fr::from_le_bytes_mod_order(&r_bytes);

        // let vals: ModVals = modulo::get_mod_vals(&a_i_biguint, &get_max_val());
        // println!("a_i after mod : {:?}", vals.remainder);

        let fermat_circuit = fermat_circut::fermat_constructor::<Fr>(a, num_to_prove);

        //TODO: hash x+1 ... x+i-1
        let a_j_s = vec![];
        Self {
            x,
            i,
            a_j_s,
            a_i,
            a_i_mod: modpow_circut::mod_vals_to_mod_witness(vals),
            fermat_circuit,
        }
    }
}

// implement the constraints for the circuit:
impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for PrimeCircuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // create the public inputs:
        let x_var = FpVar::<ConstraintF>::new_input(ark_relations::ns!(cs, "x"), || Ok(self.x))?;
        // compute x+i:
        let x_plus_i = x_var + FpVar::<ConstraintF>::constant(ConstraintF::from(self.i));
        // calculate the hash(x+i):
        let calculated_a_i: DigestVar<ConstraintF> = hash_to_bytes(x_plus_i);
        // enforce that a_i = hash(x+i):
        let a_i_var = DigestVar::new_input(ark_relations::ns!(cs, "a_i"), || Ok(self.a_i))?;
        a_i_var.enforce_equal(&calculated_a_i)?;

        // create a_i fpvar:
        let mut a_i_fpvar: FpVar<ConstraintF> =
            FpVar::<ConstraintF>::new_witness(ark_relations::ns!(cs, "a_i_fpvar"), || {
                Ok(ConstraintF::from_le_bytes_mod_order(
                    &a_i_var.to_bytes().unwrap().value().unwrap(),
                ))
            })?;
        //set the values of the witness of the mod
        let max_val = ConstraintF::from_le_bytes_mod_order(&get_max_val().to_bytes_le());
        let div = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(max_val)).unwrap();
        let origin = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.a_i_mod.n)).unwrap();
        let remainder =
            FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.a_i_mod.remainder)).unwrap();
        let quaitent =
            FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.a_i_mod.q)).unwrap();

        let result = &div * quaitent + &remainder;
        let _ = result.enforce_equal(&origin);
        let _ = origin.enforce_equal(&a_i_fpvar);
        a_i_fpvar = remainder;

        //let div = FpVar::<ConstraintF>::new_witness(cs.clone(),||Ok(get_max_val())).unwrap();
        //let result = quaitent * &div + &remainder;
        //todo get modulo out of a_i and a_j_s
        let n_var_fermat =
            FpVar::<ConstraintF>::new_witness(ark_relations::ns!(cs, "n_var_fermat"), || {
                Ok(self.fermat_circuit.n)
            })?;
        // generate a_1,a_2,a_3 by doing : a_1 = hash(r || 1 ) ,a_2 = hash(r|| 2) ,...
        n_var_fermat.enforce_equal(&a_i_fpvar)?;
        // // enforce that the is_prime is the same:
        // In the end create the constraints for the fermat circuit:
        self.fermat_circuit
            .generate_constraints(cs.clone())
            .unwrap();

        Ok(())
    }
}

// function to create the randomness:
pub fn init_randomness(randomness: &mut [u8; 32], x_plus_i_bytes: Vec<u8>, a_i: Vec<u8>, i: u64) {
    let mut sha256 = Sha256::default();
    sha256.update(&x_plus_i_bytes);
    sha256.update(&a_i);
    sha256.update(&i.to_le_bytes());
    let r = finalize(sha256);
    for (i, byte) in r.iter().enumerate() {
        randomness[i] = *byte;
    }
}
fn vec_u8_to_vec_bigint(vec: Vec<u8>) -> Vec<BigInt> {
    vec.into_iter()
        .map(|x| BigInt::from_u8(x).unwrap()) // Convert each u8 to BigInt
        .collect()
}
pub struct IsPrimeStruct(pub Vec<u8>, pub bool, pub ModVals, pub BigUint);
pub fn check_if_next_is_prime(x: Fr, j: u64) -> IsPrimeStruct {
    // hash(x+j):
    let mut sha256 = Sha256::default();
    let x_plus_j = x + Fr::from(j);
    let x_plus_j_bytes = x_plus_j.into_bigint().to_bytes_le();

    // do the hash for x+j:
    sha256.update(&x_plus_j_bytes);
    let a_j = finalize(sha256);

    // convert a_j into BigUint:
    let a_j_biguint: BigUint = BigUint::from_bytes_le(&a_j);
    let mut r_bytes = [0u8; 32];

    // r = hash(x + i || a_i = hash(x+i) || i )
    // create the randomnes:
    init_randomness(&mut r_bytes, x_plus_j_bytes.clone(), a_j.clone(), j);

    // convert r to Fr:
    let r = Fr::from_le_bytes_mod_order(&r_bytes);
    // take mod MAX_VAL:
    let max_val = get_max_val();
    let vals = get_mod_vals(&a_j_biguint, &max_val);

    let num_to_check = &vals.remainder;
    let a = &BigUint::from(r);
    // check if num_to_check is prime using the Fermat primality test
    let is_prime = fermat_test(a, num_to_check);

    // return the IsPrimeStruct containing the hashed value, primality result, and mod results
    IsPrimeStruct(a_j, is_prime, vals, a.clone())
}
// pub fn check_if_is_prime_native_fermat
#[cfg(test)]
mod tests {
    use crate::arkworks::prime_snark::fermat_circut::fermat_constructor;
    use crate::arkworks::prime_snark::modpow_circut::mod_vals_to_mod_witness;

    use super::*;
    use ark_bls12_381::{Bls12_381, Fr};
    use ark_ff::{BigInt, BigInteger};
    use ark_ff::{Field, PrimeField};

    use ark_groth16::Groth16;
    use ark_relations::r1cs::{
        ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, SynthesisError,
    };
    use ark_snark::SNARK;
    use ark_std::test_rng;
    use constants::get_max_val;
    use itertools::Itertools;
    use modulo::get_mod_vals;
    use rand::RngCore;
    use sha2::{Digest, Sha256};
    use std::time::Instant;
    #[test]
    fn initial_procces() {
        let mut rng = ark_std::test_rng();
        let cs = ConstraintSystem::<Fr>::new_ref();
        let mut x = Fr::from(5u64);
        let mut r_bytes = [0u8; 32];
        rng.fill_bytes(&mut r_bytes);
        let a_i = [0u8; 32];
        let i: u64 = 2;
        // set it up using sha256 default:
        // create for each j in 0..i-1 the hash(x+j):
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
        let mut sha256 = Sha256::default();
        let x_plus_i = x + Fr::from(i);
        let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
        // do the hash for x+i:
        sha256.update(&x_plus_i_bytes);
        let a_i = finalize(sha256.clone());
        // a_i defined as hash(x+i)
        // convert a_i to biguint:
        let a_i_biguint: BigUint = BigUint::from_bytes_le(&a_i);

        let vals = get_mod_vals(&a_i_biguint, &get_max_val());

        // r = hash(x + i || a_i = hash(x+i) || i )
        // create the randomnes:
        init_randomness(&mut r_bytes, x_plus_i_bytes.clone(), a_i.clone(), i);
        // convert r to Fr:
        let r = Fr::from_le_bytes_mod_order(&r_bytes);
        // create fermat circuit:
        let fermat_circuit = fermat_constructor::<Fr>(BigUint::from(r), vals.remainder.clone());
        // create the circuit:
        let circuit = PrimeCircuit {
            x, // a seed for the initial hash
            i, // the index i s.t we check if a_i=hash(x+i) is prime
            a_j_s: a_j_s.clone(),
            a_i, // the hash of x+i - the number we want to check if it is prime
            a_i_mod: mod_vals_to_mod_witness(vals), //
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

        let i: u64 = 10;
        // set it up using sha256 default:
        // create for each j in 0..i-1 the hash(x+j):
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
        let mut sha256 = Sha256::default();
        let x_plus_i = x + Fr::from(i);
        let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
        // do the hash for x+i:
        sha256.update(&x_plus_i_bytes);
        let a_i = finalize(sha256.clone());
        // convert a_i to biguint:
        let a_i_biguint: BigUint = BigUint::from_bytes_le(&a_i);

        let vals = get_mod_vals(&a_i_biguint, &get_max_val());

        // r = hash(x + i || a_i = hash(x+i) || i )
        // create the randomness:
        let mut r_bytes = [0u8; 32];
        init_randomness(&mut r_bytes, x_plus_i_bytes.clone(), a_i.clone(), i);
        // convert r to Fr:
        let r = Fr::from_le_bytes_mod_order(&r_bytes);
        // create fermat circuit:
        let fermat_circuit = fermat_constructor::<Fr>(BigUint::from(r), vals.remainder.clone());
        // create the circuit:
        let circuit = PrimeCircuit {
            x,
            i,
            a_j_s: a_j_s.clone(),
            a_i, // hash(x+i)
            a_i_mod: mod_vals_to_mod_witness(get_mod_vals(&a_i_biguint, &get_max_val())),
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
        for (i, input) in public_input.iter().enumerate() {
            println!("public_input[{}]: {:?}", i, input);
        }
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
    #[test]
    fn test_groth_with_constructor() {
        let x = 113130u64;
        let i = 3;
        let mut found_prime = None; // To store the first prime found
        let mut check_result = None;
        let mut found_j = 0; // Store the value of j when the prime is found

        // I want to hash(x), hash(x+1), ..., hash(x+i-1) and then check if the number
        // is prime after taking mod using get_max_val and check_if_next_is_prime:
        for j in 0..=i {
            // Use check_if_next_is_prime to check each (x + j)
            check_result = Some(check_if_next_is_prime(Fr::from(x), j));

            // If a prime number is found, store it and break the loop
            if check_result.as_ref().unwrap().1 {
                found_prime = Some(check_result.as_ref().unwrap().2.remainder.clone());
                found_j = j; // Save the value of j when a prime is found
                break;
            }
        }

        // If no prime was found, skip the rest
        if found_prime.is_none() {
            println!("No prime number found in the given range.");
            return;
        } else {
            // Print the first prime number found
            println!(
                "First prime number found: {}",
                found_prime.unwrap().to_string()
            );
            // unwrap() is safe here since we checked for None
            println!("j: {}", found_j); // Print the value of j where the prime was found

            // Unwrap check_result
            let check_result = check_result.unwrap();

            //pub struct IsPrimeStruct(Vec<u8>, bool, ModVals, BigUint);

            // Create the prime circuit using the found prime and the j from the loop
            let prime_circuit = PrimeCircuit::new(
                check_result.3.clone(),
                check_result.2.remainder.clone(),
                Fr::from(x),
                check_result.0.clone(),
                found_j, // Use the found j from the loop
                check_result.2.clone(),
            );

            // // Set up the Groth16 proof system
            // let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());

            // // Setup the Groth16 proving key and verification key
            // let start_setup = Instant::now();
            // let (pk, vk) =
            //     Groth16::<Bls12_381>::circuit_specific_setup(prime_circuit.clone(), &mut rng)
            //         .unwrap();
            // let setup_duration = start_setup.elapsed();
            // println!("Setup time: {:?}", setup_duration);

            // // Create the proof
            // let start_proof = Instant::now();
            // let proof = Groth16::<Bls12_381>::prove(&pk, prime_circuit.clone(), &mut rng).unwrap();
            // let proof_duration = start_proof.elapsed();
            // println!("Proof generation time: {:?}", proof_duration);

            // Extract the public input
            let cs = ConstraintSystem::<Fr>::new_ref();
            prime_circuit.generate_constraints(cs.clone()).unwrap();
            let real_public_input = ConstraintSystemRef::borrow(&cs)
                .unwrap()
                .instance_assignment
                .clone();

            let public_input = real_public_input.clone()[1..].to_vec();
            // // // print the public inputs one by one nicely:
            for (i, input) in public_input.iter().enumerate() {
                println!("real public_input[{}]: {:?}", i, input);
            }

            // // print the public inputs one by one nicely:
            // for (i, input) in real_public_input.iter().enumerate() {
            //     println!("real public_input[{}]: {:?}", i, input);
            // }
            // encode the public input to a byte array

            // // // Verify the proof
            // let start_verification = Instant::now();
            // let is_correct =
            //     Groth16::<Bls12_381>::verify(&vk, &real_public_input[2..], &proof).unwrap();
            // let verification_duration = start_verification.elapsed();
            // println!("Verification time: {:?}", verification_duration);

            // assert!(is_correct, "Proof verification failed.");
        }
    }
}

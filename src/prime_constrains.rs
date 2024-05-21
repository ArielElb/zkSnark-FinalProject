use crate::miller_rabin::miller_rabin_test2;
use ark_ff::BigInteger;
use ark_ff::{Field, One, PrimeField, Zero};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::bits::boolean::AllocatedBool;
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::select::CondSelectGadget;
use ark_r1cs_std::{fields::fp::FpVar, R1CSVar};
use sha2::Sha256;
// use crate::check_hash::hash_checker_fp;
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
use ark_groth16::Groth16;
use ark_r1cs_std::fields::FieldVar;
use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, SynthesisError,
};
use ark_snark::CircuitSpecificSetupSNARK;
use ark_snark::SNARK;
use ark_std::{ops::*, UniformRand};
use num_bigint::ToBigUint;
use rand::{rngs::StdRng, SeedableRng};
#[derive(Copy, Clone)]
struct PrimeCircut<ConstraintF: PrimeField> {
    x: Option<ConstraintF>, // x is the number to be checked
    num_of_rounds: u64,
}
// For benchmarking
use std::time::{Duration, Instant};

// GENERATE CONSTRAINTS
impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for PrimeCircut<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let num_of_rounds = self.num_of_rounds;
        let x = FpVar::<ConstraintF>::new_input(cs.clone(), || {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // we want to check of hash(x) or hash(x+1) or hash(x+2) or ... hash(x+num_of_rounds) is prime
        let mut curr_var: FpVar<ConstraintF> = x.clone();
        let hasher = <DefaultFieldHasher<Sha256> as HashToField<ConstraintF>>::new(&[]);

        // i want to hash(x) check if x is prime then hash(x+1) and check if hash(x+1) is prime
        for i in 0..num_of_rounds {
            // hash the current value
            let preimage = curr_var.value().unwrap().into_bigint().to_bytes_be(); // Converting to big-endian
            let hashes: Vec<ConstraintF> = hasher.hash_to_field(&preimage, 1); // Returned vector is of size 2
                                                                               // take the actual number of the hash[0]
            let hash = hashes[0];

            let hash_bigint = hash.into_bigint();

            let is_prime = miller_rabin_test2(hash_bigint.into(), 128);

            // use AllocatedBool to create a boolean variable
            let is_prime_var = AllocatedBool::new_witness(cs.clone(), || Ok(is_prime))?;

            // add constraints
            let _ = is_prime_var.and(&is_prime_var)?;
            if is_prime {
                break;
            }
            // increment the current value x+1
            curr_var = curr_var + ConstraintF::one();
        }

        Ok(())
    }
}

fn create_pub_input<ConstraintF: PrimeField>(
    x: ConstraintF,
    num_of_rounds: u64,
) -> Vec<ConstraintF> {
    let mut pub_input = Vec::new();

    // add hash(x) , hash(x+1), hash(x+2), ... hash(x+num_of_rounds) to the public input:
    let hasher = <DefaultFieldHasher<Sha256> as HashToField<ConstraintF>>::new(&[]);
    let mut curr_var = x;
    for _ in 0..num_of_rounds {
        let preimage = curr_var.into_bigint().to_bytes_be(); // Converting to big-endian
        let hashes: Vec<ConstraintF> = hasher.hash_to_field(&preimage, 1); // Returned vector is of size 2
        let hash = hashes[0];
        // println!("hash PI: {:?}\n", hash);
        let hash_bigint = hash.into_bigint();
        pub_input.push(hash);
        curr_var = curr_var + ConstraintF::one();
    }
    pub_input
}
#[cfg(test)]
mod tests {
    use std::vec;

    use ark_serialize::CanonicalSerialize;

    use super::*;

    #[test]
    fn constraints_test() {
        let cs = ConstraintSystem::<BlsFr>::new_ref();
        // cs.set_mode(SynthesisMode::Prove { construct_matrices: true });
        let x = BlsFr::from(227u8);
        // let the number of rounds be 3
        let num_of_rounds = 2000;
        let circuit = PrimeCircut {
            x: Some(x),
            num_of_rounds,
        };
        circuit.generate_constraints(cs.clone()).unwrap();
        let public_input = ConstraintSystemRef::borrow(&cs)
            .unwrap()
            .instance_assignment
            .clone();

        println!("Public input: {:?}", public_input);
        // print the number of constraints
        println!("Number of constraints: {:?}", cs.num_constraints());
        println!("Number of variables: {:?}", cs.num_instance_variables());
        // // print the matrix nicely
        cs.finalize();
        // // print the matrix nicely
        let matrix = cs.to_matrices().unwrap();
        // println!("Matrix A: {:?}", matrix.a);
        // println!("Matrix B: {:?}", matrix.b);
        // println!("Matrix C: {:?}", matrix.c);
        // // print the number
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn constraints_bench() {
        use ark_std::test_rng;
        use rand::RngCore;
        let numrounds = 200;
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
        let cs = ConstraintSystem::<BlsFr>::new_ref();

        const SAMPLES: u32 = 50;

        let mut total_time = Duration::new(0, 0);
        for _ in 0..SAMPLES {
            let x = BlsFr::rand(&mut rng);
            // generate from 50-300 rounds
            let num_of_rounds = rng.next_u32() % 300 + 50;
            let circuit = PrimeCircut {
                x: Some(x),
                num_of_rounds: num_of_rounds as u64,
            };
            let start = Instant::now();
            circuit.generate_constraints(cs.clone()).unwrap();
            let end = start.elapsed();
            total_time += end;
            // Extract the public input
            let public_input = ConstraintSystemRef::borrow(&cs)
                .unwrap()
                .instance_assignment
                .clone();
            println!("Public input: {:?}", public_input);
        }
        let avg = total_time / SAMPLES;
        let avg = avg.subsec_nanos() as f64 / 1_000_000_000f64 + (avg.as_secs() as f64);
        println!("Average time: {:?} seconds", avg);
    }

    #[test]
    fn groth16() {
        use ark_std::test_rng;
        use rand::RngCore;
        let numrounds = 2000;
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());

        // SETUP THE GROTH16 SNARK
        let circuit = PrimeCircut {
            x: None,
            num_of_rounds: 0,
        };
        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

        let circut2 = PrimeCircut {
            x: Some(BlsFr::from(227u8)),
            num_of_rounds: numrounds,
        };

        // Generate the proof
        let proof = Groth16::<Bls12_381>::prove(&pk, circut2, &mut rng).unwrap();

        // Generate the public input
        let public_input = vec![BlsFr::from(227u8)];

        // // Verify the proof
        let is_correct = Groth16::<Bls12_381>::verify(&vk, &public_input, &proof).unwrap();
        assert!(is_correct);
    }

    #[test]
    fn groth_indirect_public_inputs() {
        use ark_std::test_rng;
        use rand::RngCore;
        let numrounds = 2000;
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());

        // SETUP THE GROTH16 SNARK
        let circuit = PrimeCircut {
            x: None,
            num_of_rounds: 0,
        };
        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

        let circut2 = PrimeCircut {
            x: Some(BlsFr::from(227u8)),
            num_of_rounds: numrounds,
        };

        // Generate the proof
        let proof = Groth16::<Bls12_381>::prove(&pk, circut2, &mut rng).unwrap();

        let cs_too = ConstraintSystem::new_ref();
        circut2.generate_constraints(cs_too.clone()).unwrap();
        let public_input = ConstraintSystemRef::borrow(&cs_too)
            .unwrap()
            .instance_assignment
            .clone();
        let res = cs_too.is_satisfied().unwrap();
        assert!(res);
        // println!("Public input: {:?}", public_input);

        // let is_correct = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
        // assert!(is_correct);
    }
    #[test]
    fn groth16_benchmark() {
        use ark_std::test_rng;
        use rand::RngCore;
        use std::mem;
        let numrounds = 200;
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());

        // SETUP THE GROTH16 SNARK
        println!("Creating parameters...");
        let circuit = PrimeCircut {
            x: None,
            num_of_rounds: 0,
        };
        let (pk, vk) = Groth16::<Bls12_381>::setup(circuit, &mut rng).unwrap();

        // Prepare the verification key (for proof verification)
        let pvk = Groth16::<Bls12_381>::process_vk(&vk).unwrap();

        // init empty vector to store the proofs of BlsFr
        let mut pub_inputs: Vec<BlsFr> = Vec::new();

        // Generate the proof
        println!("Creating proofs...");
        // Let's benchmark stuff!
        const SAMPLES: u32 = 50;
        let mut total_proving = Duration::new(0, 0);
        let mut total_verifying = Duration::new(0, 0);
        //  add benchmark loop for some random numbers
        for _ in 0..SAMPLES {
            let x = BlsFr::rand(&mut rng);
            // start the timer
            let start = Instant::now();
            let c = PrimeCircut {
                x: Some(x),
                num_of_rounds: numrounds,
            };
            // print x
            // println!("x: {:?}", x);
            // Generate the proof
            let proof = Groth16::<Bls12_381>::prove(&pk, c, &mut rng).unwrap();
            // print the proof size
            total_proving += start.elapsed();
            // print the proof size

            // print the proof size
            println!("Proof size: {:?}", mem::size_of_val(&proof));
            let start = Instant::now();
            pub_inputs.push(x);
            assert!(
                Groth16::<Bls12_381>::verify_with_processed_vk(&pvk, &pub_inputs, &proof).unwrap()
            );
            total_verifying += start.elapsed();
        }
        let proving_avg = total_proving / SAMPLES;
        let proving_avg =
            proving_avg.subsec_nanos() as f64 / 1_000_000_000f64 + (proving_avg.as_secs() as f64);

        let verifying_avg = total_verifying / SAMPLES;
        let verifying_avg = verifying_avg.subsec_nanos() as f64 / 1_000_000_000f64
            + (verifying_avg.as_secs() as f64);

        println!("Average proving time: {:?} seconds", proving_avg);
        println!("Average verifying time: {:?} seconds", verifying_avg);
    }
}

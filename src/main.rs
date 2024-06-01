use actix_cors::Cors;
use actix_files::Files;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
use ark_ff::{BigInteger, PrimeField};
use ark_groth16::Groth16;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::R1CSVar;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_snark::SNARK;
use ark_std::rand::SeedableRng;

use prime_Snarks::miller_rabin::miller_rabin_test2;
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::time::Instant;

#[derive(Deserialize)]
struct InputData {
    x: u64,
    num_of_rounds: u64,
}

#[derive(Serialize)]
struct OutputData {
    proof: String,
    public_input: Vec<String>,
    num_constraints: usize,
    num_variables: usize,
    proving_time: f64,
    verifying_time: f64,
    found_prime: bool,
}

#[derive(Copy, Clone)]
struct PrimeCircut<ConstraintF: PrimeField> {
    x: Option<ConstraintF>,
    num_of_rounds: u64,
}

// Print nicely formatted information
fn tracesub<T: std::fmt::Debug>(name: &str, value: T) {
    println!("{}: {:?}", name, value);
}

impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for PrimeCircut<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // Input variable x
        let x = FpVar::<ConstraintF>::new_input(cs.clone(), || {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Boolean variables for primality checks
        let mut found_prime = ark_r1cs_std::boolean::Boolean::constant(false);
        let mut curr_var: FpVar<ConstraintF> = x.clone();
        let hasher = <DefaultFieldHasher<Sha256> as HashToField<ConstraintF>>::new(&[]);

        for _ in 0..self.num_of_rounds {
            // Compute the hash of the current value
            let hash = FpVar::<ConstraintF>::new_witness(cs.clone(), || {
                let preimage = curr_var.value()?.into_bigint().to_bytes_be();
                let hashes: Vec<ConstraintF> = hasher.hash_to_field(&preimage, 1);
                Ok(hashes[0])
            })?;

            // Check if the hash is prime
            let is_prime_var = ark_r1cs_std::boolean::Boolean::new_witness(cs.clone(), || {
                let hash_bigint = hash.value()?.into_bigint();
                Ok(miller_rabin_test2(hash_bigint.into(), 128))
            })?;

            // Enforce that if a prime is found, it must be the first prime
            found_prime = found_prime.or(&is_prime_var)?;

            // Enforce that the current value is not prime
            is_prime_var.conditional_enforce_equal(
                &ark_r1cs_std::boolean::Boolean::constant(false),
                &found_prime.not(),
            )?;

            // Move to the next candidate
            curr_var = curr_var + ConstraintF::one();
        }

        // Ensure that we found at least one prime
        found_prime.enforce_equal(&ark_r1cs_std::boolean::Boolean::constant(true))?;

        Ok(())
    }
}

#[post("/compute")]
async fn compute(data: web::Json<InputData>) -> impl Responder {
    let mut rng = StdRng::seed_from_u64(42);
    let x = BlsFr::from(data.x);
    let num_of_rounds = data.num_of_rounds;

    let circuit = PrimeCircut {
        x: Some(x),
        num_of_rounds,
    };

    // Setup
    let start = Instant::now();
    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();
    let setup_time = start.elapsed().as_secs_f64();

    let circuit2 = PrimeCircut {
        x: Some(x),
        num_of_rounds,
    };
    // Prover
    let start = Instant::now();
    let proof = Groth16::<Bls12_381>::prove(&pk, circuit2, &mut rng).unwrap();
    let proving_time = start.elapsed().as_secs_f64();

    let cs_too = ark_relations::r1cs::ConstraintSystem::new_ref();
    circuit2.generate_constraints(cs_too.clone()).unwrap();
    let public_input = cs_too.borrow().unwrap().instance_assignment.clone();

    // Verifier
    let start = Instant::now();
    let found_prime = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
    let verifying_time = start.elapsed().as_secs_f64();

    // Return the proof and the public input
    let result = OutputData {
        proof: format!("{:?}", proof),
        public_input: public_input.iter().map(|x| format!("{:?}", x)).collect(),
        num_constraints: cs_too.num_constraints(),
        num_variables: cs_too.num_instance_variables(),
        proving_time,
        verifying_time,
        found_prime,
    };

    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // Add this line for CORS
            .service(compute)
            .service(Files::new("/", "./build").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// create tests:
#[cfg(test)]
mod tests {
    use ark_relations::r1cs::OptimizationGoal;
    use num_bigint::ToBigUint;
    use web::trace;

    use super::*;

    #[test]
    // test the prime circuit
    fn test_prime_circuit() {
        let mut rng = StdRng::seed_from_u64(42);
        let x = BlsFr::from(1);
        let num_of_rounds = 250;

        let circuit = PrimeCircut {
            x: Some(x),
            num_of_rounds,
        };

        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

        let circuit2 = PrimeCircut {
            x: Some(x),
            num_of_rounds,
        };

        let proof = Groth16::<Bls12_381>::prove(&pk, circuit2, &mut rng).unwrap();

        let cs_too = ark_relations::r1cs::ConstraintSystem::new_ref();
        circuit2.generate_constraints(cs_too.clone()).unwrap();
        let public_input = cs_too.borrow().unwrap().instance_assignment.clone();

        let found_prime = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();

        assert_eq!(found_prime, true);
    }
    // test the miller rabin test
    #[test]
    fn test_miller_rabin() {
        // create a list of prime numbers:
        let prime_numbers = vec![
            2.to_biguint().unwrap(),
            5.to_biguint().unwrap(),
            7.to_biguint().unwrap(),
            11.to_biguint().unwrap(),
            13.to_biguint().unwrap(),
            17.to_biguint().unwrap(),
            19.to_biguint().unwrap(),
            23.to_biguint().unwrap(),
            29.to_biguint().unwrap(),
            31.to_biguint().unwrap(),
            37.to_biguint().unwrap(),
            41.to_biguint().unwrap(),
            43.to_biguint().unwrap(),
            47.to_biguint().unwrap(),
            53.to_biguint().unwrap(),
            59.to_biguint().unwrap(),
            61.to_biguint().unwrap(),
            67.to_biguint().unwrap(),
            71.to_biguint().unwrap(),
            73.to_biguint().unwrap(),
            79.to_biguint().unwrap(),
            83.to_biguint().unwrap(),
            89.to_biguint().unwrap(),
            97.to_biguint().unwrap(),
            101.to_biguint().unwrap(),
            103.to_biguint().unwrap(),
            107.to_biguint().unwrap(),
            109.to_biguint().unwrap(),
            113.to_biguint().unwrap(),
            127.to_biguint().unwrap(),
            131.to_biguint().unwrap(),
            137.to_biguint().unwrap(),
            139.to_biguint().unwrap(),
            149.to_biguint().unwrap(),
            151.to_biguint().unwrap(),
            157.to_biguint().unwrap(),
            163.to_biguint().unwrap(),
            167.to_biguint().unwrap(),
            173.to_biguint().unwrap(),
            179.to_biguint().unwrap(),
            181.to_biguint().unwrap(),
            191.to_biguint().unwrap(),
            193.to_biguint().unwrap(),
            197.to_biguint().unwrap(),
            199.to_biguint().unwrap(),
            211.to_biguint().unwrap(),
            223.to_biguint().unwrap(),
            227.to_biguint().unwrap(),
            229.to_biguint().unwrap(),
            233.to_biguint().unwrap(),
            239.to_biguint().unwrap(),
            241.to_biguint().unwrap(),
            251.to_biguint().unwrap(),
            257.to_biguint().unwrap(),
        ];

        for prime in prime_numbers {
            let result = miller_rabin_test2(prime.clone(), 128);
            assert_eq!(result, true);
        }
    }
    #[test]
    fn miller_failed_test() {
        let n = 3.to_biguint().unwrap();
        let k = 128;
        let result = miller_rabin_test2(n, k);
        assert_eq!(result, false);
    }

    #[test]
    // hash _to_field test
    fn test_hash_to_field() {
        let hasher = <DefaultFieldHasher<Sha256> as HashToField<BlsFr>>::new(&[]);
        let preimage = vec![1, 2, 3, 4, 5];
        let hashes: Vec<BlsFr> = hasher.hash_to_field(&preimage, 1);
        assert_eq!(hashes.len(), 1);
    }
    #[test]
    // test the constraint system for good input

    fn test_groth16_good_input() {
        let mut rng = StdRng::seed_from_u64(42);
        let x = BlsFr::from(1);
        let num_of_rounds = 250;
        let circuit = PrimeCircut {
            x: Some(x),
            num_of_rounds,
        };

        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

        let circuit2 = PrimeCircut {
            x: Some(x),
            num_of_rounds,
        };

        let proof = Groth16::<Bls12_381>::prove(&pk, circuit2, &mut rng).unwrap();

        let cs_too = ark_relations::r1cs::ConstraintSystem::new_ref();
        circuit2.generate_constraints(cs_too.clone()).unwrap();
        let public_input = cs_too.borrow().unwrap().instance_assignment.clone();

        let found_prime = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
        assert_eq!(found_prime, true);
    }

    #[test]
    // test the constraint system for bad input
    fn test_groth16_bad_input() {
        let mut rng = StdRng::seed_from_u64(42);
        let x = BlsFr::from(1);
        let num_of_rounds = 50;
        let circuit = PrimeCircut {
            x: Some(x),
            num_of_rounds,
        };

        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

        let circuit2 = PrimeCircut {
            x: Some(x),
            num_of_rounds,
        };

        let proof = Groth16::<Bls12_381>::prove(&pk, circuit2, &mut rng).unwrap();

        let cs_too = ark_relations::r1cs::ConstraintSystem::new_ref();
        circuit2.generate_constraints(cs_too.clone()).unwrap();
        let public_input = cs_too.borrow().unwrap().instance_assignment.clone();

        let found_prime = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
        assert_eq!(found_prime, false);
    }

    #[test]
    // print out the constraints and variables and the time
    fn test_constraints() {
        let x = BlsFr::from(1);
        let num_of_rounds = 250;
        let circuit = PrimeCircut {
            x: Some(x),
            num_of_rounds,
        };
        let cs_too = ark_relations::r1cs::ConstraintSystem::new_ref();
        cs_too.set_optimization_goal(OptimizationGoal::Constraints);
        circuit.generate_constraints(cs_too.clone()).unwrap();
        let is_satisfied = cs_too.is_satisfied().unwrap();
        assert_eq!(is_satisfied, true);
        tracesub("num_constraints", cs_too.num_constraints());
        tracesub("num_variables", cs_too.num_instance_variables());
        // trace the time:
        tracesub(
            "num_linear_combinations",
            cs_too.borrow().unwrap().num_linear_combinations,
        );
    }
}

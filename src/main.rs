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

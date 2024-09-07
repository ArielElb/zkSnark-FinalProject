use crate::arkworks::prime_snark::fermat_circut::fermat_constructor;
use crate::arkworks::prime_snark::utils::constants::get_max_val;
use crate::arkworks::prime_snark::utils::hasher::hash_x_plus_i_native;

use crate::arkworks::matrix_proof_of_work::io::{
    decode_proof, decode_pvk, encode_proof, encode_pvk, read_proof, write_proof_to_file,
};
use crate::arkworks::prime_snark::prime_circut::PrimeCircut;
use crate::arkworks::prime_snark::prime_circut::{self, init_randomness};
use crate::arkworks::prime_snark::utils::modulo::get_mod_vals;
use actix_web::{web, HttpResponse, Responder};
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_ff::BigInteger;
use ark_ff::PrimeField;
use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_relations::r1cs::ConstraintSystem;
use ark_snark::SNARK;
use ark_std::rand::SeedableRng;
use ark_std::test_rng;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::RngCore as _;
use serde::Deserialize;
use serde::Serialize;
use std::time::Instant;
// create a struct of ProveInput that will be used to get the data from the user : x- a intial seed number , i - number of rounds
#[derive(Debug, Serialize, Deserialize)]
pub struct ProveInput {
    x: u64,
    i: u64,
}

// create a struct of ProveOutput that will be used to send the data to the user : proof - the proof of the computation , public_input - the public input of the computation , num_constraints - the number of constraints in the computation , num_variables - the number of variables in the computation , proving_time - the time it took to prove the computation , verifying_time - the time it took to verify the computation , found_prime - if the number is prime or not
#[derive(Debug, Serialize, Deserialize)]
pub struct ProveOutput {
    proof: String,
    num_constraints: usize,
    num_variables: usize,
    setup_time: f64,
    proving_time: f64,
    found_prime: bool,
}
pub async fn prove_prime(data: web::Json<ProveInput>) -> impl Responder {
    // extract the data from the user
    let data = data.into_inner();
    let x = data.x;
    let i = data.i;
    // Crate prime circuit:
    let prime_circuit = PrimeCircut::new(BlsFr::from(x), i);
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
    // setup the groth16:
    let start_setup = Instant::now();
    let (pk, vk) =
        Groth16::<Bls12_381>::circuit_specific_setup(prime_circuit.clone(), &mut rng).unwrap();
    let setup_duration = start_setup.elapsed();
    println!("Setup time: {:?}", setup_duration);
    // create the proof:
    let start_proof = Instant::now();
    let proof = Groth16::<Bls12_381>::prove(&pk, prime_circuit.clone(), &mut rng).unwrap();
    let proof_duration = start_proof.elapsed();
    println!("Proof generation time: {:?}", proof_duration);

    // check stats for the constraints:
    let cs = ConstraintSystem::new_ref();
    let found_prime = prime_circuit.fermat_circuit.is_prime;
    prime_circuit.generate_constraints(cs.clone()).unwrap();

    HttpResponse::Ok().json(ProveOutput {
        proof: encode_proof::<Bls12_381>(&proof),
        num_constraints: cs.num_constraints(),
        num_variables: cs.num_instance_variables() + cs.num_witness_variables(),
        setup_time: setup_duration.as_secs_f64(),
        proving_time: proof_duration.as_secs_f64(),
        found_prime,
    })
}

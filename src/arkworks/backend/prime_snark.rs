use crate::arkworks::prime_snark::fermat_circut::fermat_constructor;
use crate::arkworks::prime_snark::utils::constants::get_max_val;
use crate::arkworks::prime_snark::utils::hasher::hash_x_plus_i_native;

use crate::arkworks::matrix_proof_of_work::io::{
    decode_proof, decode_pvk, encode_proof, encode_pvk, read_proof, write_proof_to_file,
};
use crate::arkworks::prime_snark::prime_circut::{self, init_randomness};
use crate::arkworks::prime_snark::prime_circut::{check_if_next_is_prime, PrimeCircuit};
use crate::arkworks::prime_snark::utils::modulo::get_mod_vals;
use actix_web::{web, HttpResponse, Responder};
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_ff::BigInteger;
use ark_ff::PrimeField;
use ark_groth16::{prepare_verifying_key, Groth16};
use ark_r1cs_std::{ToBitsGadget, ToBytesGadget};
use ark_relations::r1cs::ConstraintSystem;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef};
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
    num_constraints: usize,
    j: u64,
    num_variables: usize,
    setup_time: f64,
    proving_time: f64,
    found_prime: bool,
    prime_num: String,
    proof: String,
    pvk: String,
}

pub async fn prove_prime(data: web::Json<ProveInput>) -> impl Responder {
    // extract the data from the user
    let data = data.into_inner();
    let x = data.x; // x- a intial seed number
    let i = data.i; // i- number of rounds
    let mut found_prime = None; // To store the first prime found
    let mut check_result = None;
    let mut found_j = 0; // Store the value of j when the prime is found

    // I want to hash(x), hash(x+1), ..., hash(x+i-1) and then check if the number
    // is prime after taking mod using get_max_val and check_if_next_is_prime:
    for j in 0..=i {
        // Use check_if_next_is_prime to check each (x + j)
        check_result = Some(check_if_next_is_prime(BlsFr::from(x), j));

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
        return HttpResponse::Ok().json(ProveOutput {
            proof: "".to_string(),
            j: 0,
            num_constraints: 0,
            num_variables: 0,
            setup_time: 0.0,
            proving_time: 0.0,
            found_prime: false,
            prime_num: "".to_string(),
            pvk: "".to_string(),
        });
    }
    // Print the first prime number found
    // println!("First prime number found: {}", found_prime.unwrap());
    // unwrap() is safe here since we checked for None
    println!("j: {}", found_j); // Print the value of j where the prime was found

    // Unwrap check_result
    let check_result = check_result.unwrap();

    //pub struct IsPrimeStruct(Vec<u8>, bool, ModVals, BigUint);

    // Create the prime circuit using the found prime and the j from the loop
    let prime_circuit = PrimeCircuit::new(
        check_result.3.clone(),
        check_result.2.remainder.clone(),
        BlsFr::from(x),
        check_result.0.clone(),
        found_j, // Use the found j from the loop
        check_result.2.clone(),
    );

    // Set up the Groth16 proof system
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());

    // Setup the Groth16 proving key and verification key
    let start_setup = Instant::now();
    let (pk, vk) =
        Groth16::<Bls12_381>::circuit_specific_setup(prime_circuit.clone(), &mut rng).unwrap();
    let setup_duration = start_setup.elapsed();
    println!("Setup time: {:?}", setup_duration);

    // Create the proof
    let start_proof = Instant::now();
    let proof = Groth16::<Bls12_381>::prove(&pk, prime_circuit.clone(), &mut rng).unwrap();
    let proof_duration = start_proof.elapsed();
    println!("Proof generation time: {:?}", proof_duration);

    let cs = ConstraintSystem::<BlsFr>::new_ref();
    prime_circuit
        .clone()
        .generate_constraints(cs.clone())
        .unwrap();
    assert!(cs.is_satisfied().unwrap());

    // Return the proof and other data

    HttpResponse::Ok().json(ProveOutput {
        proof: encode_proof::<Bls12_381>(&proof),
        j: found_j,
        num_constraints: cs.num_constraints(),
        num_variables: cs.num_instance_variables() + cs.num_witness_variables(),
        setup_time: setup_duration.as_secs_f64(),
        proving_time: proof_duration.as_secs_f64(),
        found_prime: true,
        prime_num: found_prime.unwrap().to_string(),
        pvk: encode_pvk::<Bls12_381>(&prepare_verifying_key::<Bls12_381>(&vk)),
    })
}

// now for the verification part:
// create a struct of VerifyInput that will be used to get the data from the user : proof - the proof of the computation , public_input - the public input of the computation , pvk - the verifying key of the computation
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyInput {
    j: u64,
    x: u64,
    proof: String,
    pvk: String,
}

// create a struct of VerifyOutput that will be used to send the data to the user : verifying_time - the time it took to verify the computation , valid - if the computation is valid or not
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyOutput {
    verifying_time: f64,
    valid: bool,
}

pub async fn verify_prime(data: web::Json<VerifyInput>) -> impl Responder {
    // extract the data from the user
    let data = data.into_inner();
    let j = data.j; // j- the value of j where the prime was found
    let x = data.x; // x- a intial seed number
    let proof = decode_proof::<Bls12_381>(&data.proof); // proof- the proof of the computation
                                                        // let public_input = decode_proof::<Bls12_381>(&data.public_input); // public_input- the public input of the computation
    let pvk = decode_pvk::<Bls12_381>(&data.pvk); // pvk- the verifying key of the computation

    let check_result = check_if_next_is_prime(BlsFr::from(x), j);

    // Create the prime circuit using the found prime and the j from the loop
    let prime_circuit = PrimeCircuit::new(
        check_result.3.clone(),
        check_result.2.remainder.clone(),
        BlsFr::from(x),
        check_result.0.clone(),
        j, // Use the found j from the loop
        check_result.2.clone(),
    );

    let cs = ConstraintSystem::<BlsFr>::new_ref();
    prime_circuit.generate_constraints(cs.clone()).unwrap();
    let real_public_input = ConstraintSystemRef::borrow(&cs)
        .unwrap()
        .instance_assignment
        .clone();
    // Verify the proof
    let start_verify = Instant::now();
    let is_valid = Groth16::<Bls12_381>::verify_with_processed_vk(
        &pvk.unwrap(),
        &real_public_input[1..],
        &proof.unwrap(),
    )
    .unwrap();

    let verify_duration = start_verify.elapsed();
    println!("Verification time: {:?}", verify_duration);

    // Return the verification result
    HttpResponse::Ok().json(VerifyOutput {
        verifying_time: verify_duration.as_secs_f64(),
        valid: is_valid,
    })
}

use std::vec;

// create a sync function that will get two matrices A  oand B of size n x n of u64
// and return the result of A * B
use crate::arkworks::matrix_proof_of_work::alloc::FpVar2DVec;
use crate::arkworks::matrix_proof_of_work::constraints::matrix_mul;
use crate::arkworks::matrix_proof_of_work::constraints::MatrixCircuit;
use crate::arkworks::matrix_proof_of_work::hasher::{hasher, hasher_var};
use crate::arkworks::matrix_proof_of_work::io::{
    decode_hash, decode_proof, decode_pvk, encode_hash, encode_proof, encode_pvk, read_proof,
    write_proof_to_file,
};
use ark_ff::fields::models::fp::Fp;
use actix_web::{web, HttpResponse, Responder};
use ark_bls12_381::{Bls12_381, Config, Fr as F};
use ark_ec::bls12::Bls12;
use ark_ff::BigInteger;
use ark_ff::PrimeField;
use ark_groth16::prepare_verifying_key;
use ark_groth16::{Groth16, Proof};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::{ToBitsGadget, ToBytesGadget};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use ark_snark::{CircuitSpecificSetupSNARK, SNARK};
use ark_std::end_timer;
use ark_std::test_rng;
use rand::{RngCore, SeedableRng};
use std::string::String;

use serde::{Deserialize, Serialize};
use serde_json;
// create a struct of InputData that will be used to get the data from the user
#[derive(Debug, Serialize, Deserialize)]
pub struct InputData {
    size: usize,
    matrix: Vec<Vec<u64>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OutputData {
    hash: Vec<u8>,
}

pub async fn hash_matrix(data: web::Json<InputData>) -> impl Responder {
    let cs = ConstraintSystem::<F>::new_ref();
    // exctract the matrix from the data
    let data = data.into_inner();
    let data_matrix = data.matrix;
    // create a [[u64; n]; n] from the matrix
    let len: usize = data.size;
    let mut matrix = vec![vec![0u64; len]; len];
    for i in 0..len {
        for j in 0..len {
            matrix[i][j] = data_matrix[i][j];
        }
    }
    // convert the vector to [[u64; n]; n]:

    // create Fp2Var2D from the matrix:
    let matrix_c = FpVar2DVec::new_witness(cs.clone(), || Ok(matrix)).unwrap();
    // hash the matrix using hasher:
    let hash = hasher(&matrix_c).unwrap();
    let hash_value = hash[0];

    // convert the hash value to bytes:
    let hash_bytes: Vec<u8> = hash_value.into_bigint().to_bytes_le();

    println!("Hash: {:?}", hash_bytes);
    // return the response data
    HttpResponse::Ok().json(OutputData { hash: hash_bytes })
}

// create a struct of InputData that will be used to get the data from the user
#[derive(Debug, Serialize, Deserialize)]
pub struct ProveInput {
    size: usize,
    matrix_a: Vec<Vec<u64>>,
    matrix_b: Vec<Vec<u64>>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProveOutPut {
    hash_a: String,
    hash_b: String,
    hash_c: String,
    setup_time: f64,
    proving_time: f64,
    num_constraints: usize,
    num_variables: usize,
    proof: String,
    pvk: String,
}

// function to genrate a proof using groth16, getting 2 matrices A and B
pub async fn prove_matrix(data: web::Json<ProveInput>) -> impl Responder {
    let cs = ConstraintSystem::<F>::new_ref();

    // exctract the matrix from the data
    let data = data.into_inner();
    let matrix_a: Vec<Vec<u64>> = data.matrix_a;
    let matrix_b: Vec<Vec<u64>> = data.matrix_b;
    // create a [[u64; n]; n] from the matrix
    let len: usize = data.size;
    // convert the vector to [[u64; n]; n]:
    // create Fp2Var2D from the matrix:
    let matrix_a_var = FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_a.clone())).unwrap();
    let matrix_b_var = FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_b.clone())).unwrap();

    let matrix_c = matrix_mul(cs.clone(), matrix_a_var.clone(), matrix_b_var.clone());
    // hash the matrix using hasher:
    let hash_c = hasher(&matrix_c).unwrap()[0];
    //  hash a :
    let hash_a = hasher(&matrix_a_var).unwrap()[0];
    //  hash b :
    let hash_b = hasher(&matrix_b_var).unwrap()[0];
    // convert the hash value to bytes:
    let hash_bytes: Vec<u8> = hash_c.into_bigint().to_bytes_le();
    // encode the hash value to base64:
    let encoded_hash_c = encode_hash(&hash_bytes);
    let encoded_hash_a = encode_hash(&hash_a.into_bigint().to_bytes_le());
    let encoded_hash_b = encode_hash(&hash_b.into_bigint().to_bytes_le());

    // use groth16 to generate the proof:

    // create a circuit using new  function
    let circuit = MatrixCircuit::new(matrix_a.clone(), matrix_b.clone(), hash_a, hash_b, hash_c);
    // generate the proof
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
    let setup_time = std::time::Instant::now();
    let (pk, vk) = Groth16::<Bls12_381>::setup(circuit.clone(), &mut rng).unwrap();

    let setup_time = setup_time.elapsed().as_secs_f64();

    // convert the proof and vk to string:
    let pvk = prepare_verifying_key::<Bls12_381>(&vk);
    // encode the pvk to byte using encode_pvk:
    let pvk_str = encode_pvk::<Bls12_381>(&pvk);
    // open timer:
    let proving_time = std::time::Instant::now();
    let proof: Proof<Bls12<Config>> =
        Groth16::<Bls12_381>::prove(&pk, circuit.clone(), &mut rng).unwrap();

    // use the constraint system to get the number of constraints and variables:

    // end timer:
    let proving_time = proving_time.elapsed().as_secs_f64();

    // encode the proof to base64 and the vk to base64:
    let proof_str = encode_proof::<Bls12_381>(&proof);

    circuit.generate_constraints(cs.clone()).unwrap();

    // create a response data:
    let response_data = ProveOutPut {
        hash_a: encoded_hash_a,
        hash_b: encoded_hash_b,
        hash_c: encoded_hash_c,

        setup_time,
        proving_time,
        num_constraints: cs.num_constraints(),
        num_variables: cs.num_instance_variables(),
        proof: proof_str,
        pvk: pvk_str,
    };
    // return the response data
    HttpResponse::Ok().json(response_data)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyInput {
    pvk: String,
    proof: String,
    hash_a: String,
    hash_b: String,
    hash_c: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyOutPut {
    verifying_time: f64,
    valid: bool,
}

pub async fn verify_proof(data: web::Json<VerifyInput>) -> impl Responder {
    let hash_c = data.hash_c.clone();
    let hash_a = data.hash_a.clone();
    let hash_b = data.hash_b.clone();
    let pvk = data.pvk.clone();
    let proof = data.proof.clone();

    // decode the proof and vk from base64:
    let pvk = decode_pvk::<Bls12_381>(&pvk).unwrap();

    let proof = decode_proof::<Bls12_381>(&proof).unwrap();

    // convert the hash value to Fp:
    let hash_value_c = Fp::from_le_bytes_mod_order(&decode_hash(&hash_c).unwrap());
    let hash_value_a = Fp::from_le_bytes_mod_order(&decode_hash(&hash_a).unwrap());
    let hash_value_b = Fp::from_le_bytes_mod_order(&decode_hash(&hash_b).unwrap());
    let verfiying_time = std::time::Instant::now();
    let is_valid = Groth16::<Bls12_381>::verify_with_processed_vk(
        &pvk,
        &[hash_value_a, hash_value_b, hash_value_c],
        &proof,
    )
    .unwrap();
    let verifying_time = verfiying_time.elapsed().as_secs_f64();

    // create a response data:
    let response_data = VerifyOutPut {
        verifying_time,
        valid: is_valid,
    };
    // return the response data
    HttpResponse::Ok().json(response_data)
}

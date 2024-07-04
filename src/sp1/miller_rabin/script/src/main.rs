use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use alloy_sol_types::{sol, SolType};
use ark_serialize::SerializationError;
use clap::Parser;
use serde::{ser::SerializeStructVariant, Deserialize, Serialize};
use sp1_sdk::{
    utils, ProverClient, SP1CompressedProof, SP1PlonkBn254Proof, SP1Proof, SP1Stdin,
    SP1VerifyingKey,
};
use std::sync::Mutex;

/// The arguments for the prove command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CmdArgs {
    #[clap(long, default_value = "20")]
    n: u32,
    #[clap(long, default_value = "10")]
    num_of_rounds: u32,
}

/// The public values encoded as a tuple that can be easily deserialized inside Solidity.
type PublicValuesTuple = sol! {
    // n, number of rounds, prime, is_prime
    tuple(uint32, uint32, uint32, bool)
};

pub const MILLER_ELF: &[u8] = include_bytes!("../../program/elf/riscv32im-succinct-zkvm-elf");

/// The payload structure for the proof generation request.
#[derive(Deserialize)]
pub struct ProvePayload {
    n: u32,
    num_of_rounds: u32,
}

#[derive(Serialize, Deserialize)]
struct ProofResponse {
    is_prime: bool,
    prime: u32,
    vkey: String,
    proof: String,
    proof_size: usize,
    proof_time: f64,
}

pub async fn generate_proof(args: web::Json<ProvePayload>) -> impl Responder {
    // Setup logging.
    utils::setup_logger();

    let client = ProverClient::new();
    let (pk, vk) = client.setup(MILLER_ELF);

    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);
    stdin.write(&args.num_of_rounds);

    // start timer:
    let start = std::time::Instant::now();
    let proof = client.prove(&pk, stdin).expect("failed to generate proof");
    // end timer:
    let proof_time = start.elapsed().as_secs_f64();

    let (n, num_of_rounds, prime, is_prime) =
        PublicValuesTuple::abi_decode(proof.public_values.as_slice(), false).unwrap();
    println!("Successfully generated proof!");
    if is_prime {
        println!("Prime: {}", prime);
    } else {
        println!("Not found any prime within: {}", args.num_of_rounds);
    }
    // desirlized the proof:
    client.verify(&proof, &vk).expect("failed to verify proof");

    // exctract the number of circles from the RUST_INFO:
    let serializedproof = serde_json::to_string(&proof).expect("failed to serialize proof");
    let response = ProofResponse {
        is_prime,
        prime,
        vkey: serde_json::to_string(&vk).expect("failed to serialize verifying key"),
        proof_size: serializedproof.len(),
        proof: serializedproof,
        proof_time,
    };
    HttpResponse::Ok().json(response)
}

pub async fn prove(args: web::Json<ProvePayload>) -> impl Responder {
    let client = ProverClient::new();
    let (pk, vk) = client.setup(MILLER_ELF);

    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);
    stdin.write(&args.num_of_rounds);

    // open timer:
    let start = std::time::Instant::now();

    // let proof = client.prove(&pk, stdin).expect("failed to generate proof");
    let mut proof = client.prove_compressed(&pk, stdin).unwrap();
    // end timer:
    let proof_time = start.elapsed().as_secs_f64();
    let (n, num_of_rounds, prime, is_prime) =
        PublicValuesTuple::abi_decode(proof.public_values.as_slice(), false).unwrap();
    println!("Successfully generated proof!");

    println!("Not found any prime within: {}", args.num_of_rounds);
    let serializedproof = serde_json::to_string(&proof).expect("failed to serialize proof");
    let serilized_vk = serde_json::to_string(&vk).expect("failed to serialize verifying key");
    let proof_size = serializedproof.len();
    println!("Proof size: {}", serializedproof.len());

    let response = ProofResponse {
        is_prime,
        prime,
        vkey: serilized_vk,
        proof_size,
        proof: serializedproof,
        proof_time,
    };
    HttpResponse::Ok().json(response)
}

/// The payload structure for the proof generation request.
#[derive(Deserialize)]
pub struct VerifyPayload {
    proof: String,
    vkey: String,
}
#[derive(Serialize, Deserialize)]
struct VerifyResponse {
    result: bool,
    verifying_time: f64,
}
pub async fn verify(args: web::Json<VerifyPayload>) -> impl Responder {
    let client: ProverClient = ProverClient::new();
    let desrilized_proof: SP1CompressedProof =
        serde_json::from_str(&args.proof).expect("failed to deserialize proof");
    let desrilized_vkey: SP1VerifyingKey =
        serde_json::from_str(&args.vkey).expect("failed to deserialize verifying key");
    // open timer:
    let start = std::time::Instant::now();
    // let result = client.verify(&desrilized_proof, &desrilized_vkey).is_ok();
    let result = client
        .verify_compressed(&desrilized_proof, &desrilized_vkey)
        .is_ok();
    // end timer:
    let verifying_time = start.elapsed().as_secs_f64();
    // catch the expected error:
    let response = VerifyResponse {
        result,
        verifying_time,
    };
    HttpResponse::Ok().json(response)
}

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use alloy_sol_types::{sol, SolType};
use clap::Parser;
use serde::{Deserialize, Serialize};
use sp1_sdk::{ProverClient, SP1PlonkBn254Proof, SP1Stdin, SP1VerifyingKey};
use std::sync::Mutex;

/// The arguments for the prove command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CmdArgs {
    #[clap(long, default_value = "20")]
    n: u32,
    #[clap(long, default_value = "10")]
    num_of_rounds: u32,
    #[clap(long, default_value = "false")]
    evm: bool,
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
    n: u32,
    num_of_rounds: u32,
    is_prime: bool,
    prime: u32,
    vkey: String,
    public_values: String,
    proof: String,
}

pub async fn generate_proof(args: web::Json<ProvePayload>) -> impl Responder {
    let client = ProverClient::new();
    let (pk, vk) = client.setup(MILLER_ELF);

    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);
    stdin.write(&args.num_of_rounds);

    let proof = client.prove(&pk, stdin).expect("failed to generate proof");
    let (n, num_of_rounds, prime, is_prime) =
        PublicValuesTuple::abi_decode(proof.public_values.as_slice(), false).unwrap();
    println!("Successfully generated proof!");
    if is_prime {
        println!("Prime: {}", prime);
    } else {
        println!("Not found any prime within: {}", args.num_of_rounds);
    }

    client.verify(&proof, &vk).expect("failed to verify proof");

    let response = ProofResponse {
        n: args.n,
        num_of_rounds: args.num_of_rounds,
        is_prime,
        prime,
        vkey: serde_json::to_string(&vk).expect("failed to serialize verifying key"),
        public_values: serde_json::to_string(&proof.public_values)
            .expect("failed to serialize public values"),
        proof: serde_json::to_string(&proof).expect("failed to serialize proof"),
    };
    HttpResponse::Ok().json(response)
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| App::new().route("/generate_proof", web::post().to(generate_proof)))
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }

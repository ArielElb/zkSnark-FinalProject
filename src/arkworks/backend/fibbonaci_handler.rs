use crate::arkworks::constraints::fibbonaci::FibonacciCircuit;
use crate::arkworks::matrix_proof_of_work::io::{
    decode_proof, decode_pvk, encode_proof, encode_pvk,
};
use actix_web::{web, HttpResponse, Responder};
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_groth16::{prepare_verifying_key, Groth16};

use ark_snark::SNARK;
use ark_std::rand::SeedableRng;
use rand::rngs::StdRng;
use serde::{Deserialize, Serialize};
fn fibonacci(x: usize, first: u128, second: u128) -> u128 {
    if x == 0 {
        return first;
    } else if x == 1 {
        return second;
    }

    let mut prev: u128 = first;
    let mut current: u128 = second;

    for _ in 2..=x {
        let next: u128 = prev + current;
        prev = current;
        current = next;
    }

    current
}


#[derive(Deserialize)]
pub struct InputDataFib {
    pub a: u64,
    pub b: u64,
    pub num_of_rounds: usize,
}

#[derive(Deserialize)]
pub struct InputDataFibVer {
    pub proof: String,
    pub pvk: String,
    pub a: u64,
    pub b: u64,
}

#[derive(Serialize)]
pub struct OutputDataFib {
    pub proof: String,
    pub pvk: String,
    pub fib_number: String,
    pub proving_time: f64,
}

#[derive(Serialize)]
pub struct OutputVerifyData {
    pub verifying_time: f64,
    pub is_res: bool,
}

pub async fn fibbonaci_snark_proof(data: web::Json<InputDataFib>) -> impl Responder {
    let mut rng = StdRng::seed_from_u64(42);
    let fibo_num=fibonacci(data.num_of_rounds,data.a as u128,data.b as u128);
    let circuit = FibonacciCircuit::<BlsFr> {
        a: Some(BlsFr::from(data.a)),
        b: Some(BlsFr::from(data.b)),
        num_of_steps: data.num_of_rounds,
        result: Some(BlsFr::from(fibo_num)),
    };

    //   pub numb_of_steps: usize,
    //pub result: Option<F>,
    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();
    let pvk = prepare_verifying_key::<Bls12_381>(&vk);
    let start = ark_std::time::Instant::now();
    let proof = Groth16::<Bls12_381>::prove(&pk, circuit.clone(), &mut rng).unwrap();

    let proving_time = start.elapsed().as_secs_f64();

    let result = OutputDataFib {
        proof: encode_proof::<Bls12_381>(&proof),
        pvk: encode_pvk::<Bls12_381>(&pvk),
        fib_number: fibo_num.to_string(),
        proving_time,
    };

    HttpResponse::Ok().json(result)
}

pub async fn fibbonaci_snark_verify(data: web::Json<InputDataFibVer>) -> impl Responder {
    let pvk = decode_pvk::<Bls12_381>(&data.pvk).unwrap();
    let proof = decode_proof::<Bls12_381>(&data.proof).unwrap();
    let start = ark_std::time::Instant::now();
    // the one is for dummy input!
    let result = Groth16::<Bls12_381>::verify_with_processed_vk(
        &pvk,
        &[BlsFr::from(data.a), BlsFr::from(data.b), BlsFr::from(1)],
        &proof,
    )
    .unwrap();

    let verifying_time = start.elapsed().as_secs_f64();
    let data = OutputVerifyData {
        verifying_time,
        is_res: result,
    };

    HttpResponse::Ok().json(data)
}

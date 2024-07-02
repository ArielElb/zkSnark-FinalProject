use crate::arkworks::constraints::prime_constraints::{InputDataFib, InputDataFibVer};
use crate::arkworks::constraints::prime_constraints::{InputData, OutputData, PrimeCircut};
use actix_web::{web, HttpResponse, Responder};
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_relations::r1cs::ConstraintSystem;
use ark_snark::SNARK;
use ark_std::rand::SeedableRng;
use rand::rngs::StdRng;

pub async fn fibbonaci_snark_proof(data: web::Json<InputDataFib>) -> impl Responder {
    let mut rng = StdRng::seed_from_u64(42);
    let circuit = FibonacciCircuit{
        a: Some(data.a),
        b: Some(data.b),
        num_of_steps: data.num_of_steps,
        result: Some(data.result),

    };
//   pub numb_of_steps: usize,
//pub result: Option<F>,
    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

    let start = ark_std::time::Instant::now();
    let proof = Groth16::<Bls12_381>::prove(&pk, circuit2.clone(), &mut rng).unwrap();
    let proving_time = start.elapsed().as_secs_f64();

    let cs_too = ConstraintSystem::new_ref();
    circuit.generate_constraints(cs_too.clone()).unwrap();
    let public_input = cs_too.borrow().unwrap().instance_assignment.clone();

    let start = ark_std::time::Instant::now();
    let found_prime = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
    let verifying_time = start.elapsed().as_secs_f64();

    let result = OutputData {
        proof: format!("{:?}", proof),
        num_constraints: cs_too.num_constraints(),
        num_variables: cs_too.num_instance_variables(),
        proving_time,
        verifying_time,
        found_prime,
    };

    HttpResponse::Ok().json(result)
}

pub async fn fibbonaci_snark_verify(data: web::Json<InputDataFibVer>) -> impl Responder {
    let mut rng = StdRng::seed_from_u64(42);
    let circuit = FibonacciCircuit{
        a: Some(data.a),
        b: Some(data.b),
        num_of_steps: data.num_of_steps,
        result: Some(data.result),

    };
//   pub numb_of_steps: usize,
//pub result: Option<F>,
    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

    let proof = data.proof;


    let cs_too = ConstraintSystem::new_ref();
    circuit2.generate_constraints(cs_too.clone()).unwrap();
    let public_input = cs_too.borrow().unwrap().instance_assignment.clone();

    let start = ark_std::time::Instant::now();
    let found_prime = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
    let verifying_time = start.elapsed().as_secs_f64();

    let result = OutputVerifyData {
        num_constraints: cs_too.num_constraints(),
        num_variables: cs_too.num_instance_variables(),
        verifying_time,
        found_prime,
    };

    HttpResponse::Ok().json(result)
}
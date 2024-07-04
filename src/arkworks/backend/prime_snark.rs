// use crate::arkworks::constraints::prime_constraints::{InputData, OutputData, PrimeCircut};
// use actix_web::{web, HttpResponse, Responder};
// use ark_bls12_381::{Bls12_381, Fr as BlsFr};
// use ark_groth16::Groth16;
// use ark_relations::r1cs::ConstraintSynthesizer;
// use ark_relations::r1cs::ConstraintSystem;
// use ark_snark::SNARK;
// use ark_std::rand::SeedableRng;
// use rand::rngs::StdRng;

// pub async fn prime_snark_compute(data: web::Json<InputData>) -> impl Responder {
//     let mut rng = StdRng::seed_from_u64(42);
//     let x = BlsFr::from(data.x);
//     let num_of_rounds = data.num_of_rounds;

//     let circuit = PrimeCircut {
//         x: Some(x),
//         num_of_rounds,
//     };

//     let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

//     let circuit2 = PrimeCircut {
//         x: Some(x),
//         num_of_rounds,
//     };

//     let start = ark_std::time::Instant::now();
//     let proof = Groth16::<Bls12_381>::prove(&pk, circuit2.clone(), &mut rng).unwrap();
//     let proving_time = start.elapsed().as_secs_f64();

//     let cs_too = ConstraintSystem::new_ref();
//     circuit2.generate_constraints(cs_too.clone()).unwrap();
//     let public_input = cs_too.borrow().unwrap().instance_assignment.clone();

//     let start = ark_std::time::Instant::now();
//     let found_prime = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
//     let verifying_time = start.elapsed().as_secs_f64();

//     let result = OutputData {
//         proof: format!("{:?}", proof),
//         public_input: public_input.iter().map(|x| format!("{:?}", x)).collect(),
//         num_constraints: cs_too.num_constraints(),
//         num_variables: cs_too.num_instance_variables(),
//         proving_time,
//         verifying_time,
//         found_prime,
//     };

//     HttpResponse::Ok().json(result)
// }

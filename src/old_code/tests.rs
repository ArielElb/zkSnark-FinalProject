// // create tests:
// #[cfg(test)]
// mod tests {
//     use crate::constraints::prime_constraints::PrimeCircut;
//     use crate::miller_rabin::miller_rabin_test2;

//     use actix_web::web;
//     use ark_bls12_381::{Bls12_381, Fr as BlsFr};
//     use ark_crypto_primitives::crh::sha256::Sha256;
//     use ark_ff::{
//         field_hashers::{DefaultFieldHasher, HashToField},
//         PrimeField,
//     };
//     use ark_groth16::Groth16;
//     use ark_relations::r1cs::ConstraintSynthesizer;
//     use ark_relations::r1cs::OptimizationGoal;
//     use ark_snark::SNARK;
//     use ark_std::rand::SeedableRng;
//     use num_bigint::ToBigUint;
//     use rand::rngs::StdRng;
//     use web::trace;
//     #[test]
//     // test the prime circuit
//     fn test_prime_circuit() {
//         let mut rng = StdRng::seed_from_u64(42);
//         let x = BlsFr::from(1);
//         let num_of_rounds = 250;

//         let circuit = PrimeCircut {
//             x: Some(x),
//             num_of_rounds,
//         };

//         let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

//         let circuit2 = PrimeCircut {
//             x: Some(x),
//             num_of_rounds,
//         };

//         let proof = Groth16::<Bls12_381>::prove(&pk, circuit2, &mut rng).unwrap();

//         let cs_too = ark_relations::r1cs::ConstraintSystem::new_ref();
//         circuit2.generate_constraints(cs_too.clone()).unwrap();
//         let public_input = cs_too.borrow().unwrap().instance_assignment.clone();

//         let found_prime = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();

//         assert_eq!(found_prime, true);
//     }
//     // test the miller rabin test
//     #[test]
//     fn test_miller_rabin() {
//         // create a list of prime numbers:
//         let prime_numbers = vec![
//             2.to_biguint().unwrap(),
//             5.to_biguint().unwrap(),
//             7.to_biguint().unwrap(),
//             11.to_biguint().unwrap(),
//             13.to_biguint().unwrap(),
//             17.to_biguint().unwrap(),
//             19.to_biguint().unwrap(),
//             23.to_biguint().unwrap(),
//             29.to_biguint().unwrap(),
//             31.to_biguint().unwrap(),
//             37.to_biguint().unwrap(),
//             41.to_biguint().unwrap(),
//             43.to_biguint().unwrap(),
//             47.to_biguint().unwrap(),
//             53.to_biguint().unwrap(),
//             59.to_biguint().unwrap(),
//             61.to_biguint().unwrap(),
//             67.to_biguint().unwrap(),
//             71.to_biguint().unwrap(),
//             73.to_biguint().unwrap(),
//             79.to_biguint().unwrap(),
//             83.to_biguint().unwrap(),
//             89.to_biguint().unwrap(),
//             97.to_biguint().unwrap(),
//             101.to_biguint().unwrap(),
//             103.to_biguint().unwrap(),
//             107.to_biguint().unwrap(),
//             109.to_biguint().unwrap(),
//             113.to_biguint().unwrap(),
//             127.to_biguint().unwrap(),
//             131.to_biguint().unwrap(),
//             137.to_biguint().unwrap(),
//             139.to_biguint().unwrap(),
//             149.to_biguint().unwrap(),
//             151.to_biguint().unwrap(),
//             157.to_biguint().unwrap(),
//             163.to_biguint().unwrap(),
//             167.to_biguint().unwrap(),
//             173.to_biguint().unwrap(),
//             179.to_biguint().unwrap(),
//             181.to_biguint().unwrap(),
//             191.to_biguint().unwrap(),
//             193.to_biguint().unwrap(),
//             197.to_biguint().unwrap(),
//             199.to_biguint().unwrap(),
//             211.to_biguint().unwrap(),
//             223.to_biguint().unwrap(),
//             227.to_biguint().unwrap(),
//             229.to_biguint().unwrap(),
//             233.to_biguint().unwrap(),
//             239.to_biguint().unwrap(),
//             241.to_biguint().unwrap(),
//             251.to_biguint().unwrap(),
//             257.to_biguint().unwrap(),
//         ];

//         for prime in prime_numbers {
//             let result = miller_rabin_test2(prime.clone(), 128);
//             assert_eq!(result, true);
//         }
//     }
//     #[test]
//     fn miller_failed_test() {
//         let n = 3.to_biguint().unwrap();
//         let k = 128;
//         let result = miller_rabin_test2(n, k);
//         assert_eq!(result, false);
//     }

//     #[test]
//     // hash _to_field test
//     fn test_hash_to_field() {
//         let hasher = <DefaultFieldHasher<Sha256> as HashToField<BlsFr>>::new(&[]);
//         let preimage = vec![1, 2, 3, 4, 5];
//         let hashes: Vec<BlsFr> = hasher.hash_to_field(&preimage, 1);

//         assert_eq!(hashes.len(), 1);
//     }
//     #[test]
//     // test the constraint system for good input

//     fn test_groth16_good_input() {
//         let mut rng = StdRng::seed_from_u64(42);
//         let x = BlsFr::from(1);
//         let num_of_rounds = 250;
//         let circuit = PrimeCircut {
//             x: Some(x),
//             num_of_rounds,
//         };

//         let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

//         let circuit2 = PrimeCircut {
//             x: Some(x),
//             num_of_rounds,
//         };

//         let proof = Groth16::<Bls12_381>::prove(&pk, circuit2, &mut rng).unwrap();

//         let cs_too = ark_relations::r1cs::ConstraintSystem::new_ref();
//         circuit2.generate_constraints(cs_too.clone()).unwrap();
//         let public_input = cs_too.borrow().unwrap().instance_assignment.clone();

//         let found_prime = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
//         assert_eq!(found_prime, true);
//     }

//     #[test]
//     // test the constraint system for bad input
//     fn test_groth16_bad_input() {
//         let mut rng = StdRng::seed_from_u64(42);
//         let x = BlsFr::from(1);
//         let num_of_rounds = 50;
//         let circuit = PrimeCircut {
//             x: Some(x),
//             num_of_rounds,
//         };

//         let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();

//         let circuit2 = PrimeCircut {
//             x: Some(x),
//             num_of_rounds,
//         };

//         let proof = Groth16::<Bls12_381>::prove(&pk, circuit2, &mut rng).unwrap();

//         let cs_too = ark_relations::r1cs::ConstraintSystem::new_ref();
//         circuit2.generate_constraints(cs_too.clone()).unwrap();
//         let public_input = cs_too.borrow().unwrap().instance_assignment.clone();

//         let found_prime = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
//         assert_eq!(found_prime, false);
//     }

//     #[test]
//     // print out the constraints and variables and the time
//     fn test_constraints() {
//         let x = BlsFr::from(1);

//         let num_of_rounds = 250;
//         let circuit = PrimeCircut {
//             x: Some(x),
//             num_of_rounds,
//         };
//         let cs_too = ark_relations::r1cs::ConstraintSystem::new_ref();
//         cs_too.set_optimization_goal(OptimizationGoal::Constraints);
//         circuit.generate_constraints(cs_too.clone()).unwrap();
//         let is_satisfied = cs_too.is_satisfied().unwrap();

//         assert_eq!(is_satisfied, true);
//     }
// }

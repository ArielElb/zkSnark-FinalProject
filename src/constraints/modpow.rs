// use ark_bls12_381::Fr;
// use ark_ff::PrimeField;
// use ark_r1cs_std::boolean::Boolean;
// use ark_r1cs_std::eq::EqGadget;
// use ark_r1cs_std::fields::fp::FpVar;
// use ark_r1cs_std::fields::FieldVar;
// use ark_r1cs_std::select::CondSelectGadget;
// use ark_r1cs_std::R1CSVar;
// use ark_r1cs_std::{alloc::AllocVar, ToBitsGadget};
// use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
// use num_bigint::BigUint;
// use std::str::FromStr;

// const NUM_REDUCTIONS: usize = 10; // Set an appropriate upper bound on the number of reductions

// struct ModExpCircuit<F: PrimeField> {
//     base: F,
//     exponent: F,
//     modulus: F,
//     res: F,
// }
// fn reduce_mod<F: PrimeField>(
//     cs: ConstraintSystemRef<F>,
//     result: &FpVar<F>,
//     modulus: &FpVar<F>,
// ) -> Result<FpVar<F>, SynthesisError> {
//     // Check if result >= modulus
//     let mut final_result: FpVar<F> = result.clone();
//     // Reduce result modulo the modulus a fixed number of times
//     for _ in 0..NUM_REDUCTIONS {
//         let should_reduce = result.is_cmp(modulus, std::cmp::Ordering::Greater, true)?;

//         // Subtract modulus from result if should_reduce is true
//         let reduced_result = result.clone() - modulus;

//         // Conditionally select between reduced_result and result based on should_reduce
//         final_result =
//             CondSelectGadget::conditionally_select(&should_reduce, &reduced_result, result)?;
//     }
//     Ok(final_result)
// }

// impl<F: PrimeField> ConstraintSynthesizer<F> for ModExpCircuit<F> {
//     fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
//         // Allocate the base, exponent, and modulus as public inputs
//         let base = FpVar::new_input(cs.clone(), || Ok(self.base))?;
//         let exponent = FpVar::new_input(cs.clone(), || Ok(self.exponent))?;
//         let modulus = FpVar::new_input(cs.clone(), || Ok(self.modulus))?;
//         let zero = FpVar::constant(F::zero());
//         // Initialize the result as 1
//         let mut result = FpVar::constant(F::one());
//         // Convert exponent to bits and perform modular exponentiation
//         let exponent_bits = exponent.to_bits_le()?;
//         for bit in exponent_bits.iter().rev() {
//             // Square the current result
//             result = result.clone() * &result;

//             result = reduce_mod(cs.clone(), &result, &modulus)?;
//             // Multiply by base if the current bit is 1
//             result = bit.select(&(result.clone() * &base), &result)?;

//             // Reduce result modulo the modulus again a fixed number of times
//             result = reduce_mod(cs.clone(), &result, &modulus)?;
//         }

//         // Enforce the final result as a public input
//         let final_result = FpVar::new_input(cs.clone(), || Ok(result.value().unwrap()))?;

//         // Enforce the final result to be equal to the expected result
//         final_result.enforce_equal(&FpVar::new_input(cs.clone(), || Ok(self.res))?)?;

//         Ok(())
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use ark_relations::r1cs::ConstraintSystem;
//     use ark_std::test_rng;
//     use num_bigint::{BigUint, RandBigInt};

//     #[test]
//     fn test_mod_exp_circuit() {
//         // 2^16 mod 10 = 6 --> 2^16 = 65536 --> the maximum number of reductions is :  65536 / 10 = 6553
//         // the maximum number of reductions is :  3^7 / 7 = 2187 / 7 = 312
//         // Example parameters (for testing purposes)
//         let base = BigUint::from(2u64);
//         let exponent = BigUint::from(7u64);
//         let modulus = BigUint::from(3u64);
//         let res = base.modpow(&exponent, &modulus);

//         // Convert big integers to field elements
//         let base = Fr::from_str(&base.to_string()).unwrap();
//         let exponent = Fr::from_str(&exponent.to_string()).unwrap();
//         let modulus = Fr::from_str(&modulus.to_string()).unwrap();
//         let res = Fr::from_str(&res.to_string()).unwrap();

//         println!("base: {}", base);
//         println!("exponent: {}", exponent);
//         println!("modulus: {}", modulus);
//         println!("res: {}", res);

//         // Create the circuit
//         let circuit = ModExpCircuit {
//             base,
//             exponent,
//             modulus,
//             res,
//         };

//         // Setup the constraint system
//         let cs = ConstraintSystem::<Fr>::new_ref();

//         // Generate constraints
//         circuit.generate_constraints(cs.clone()).unwrap();

//         // Check if the constraint system is satisfied
//         assert!(cs.is_satisfied().unwrap());
//     }

//     #[test]
//     fn rand_example() {
//         let rng = &mut ark_std::test_rng();
//         //  10^2 mod 3 = 1 // number of reductions at most is 100 / 3 = 33
//         // Example parameters (for testing purposes)
//         for i in 0..1 {
//             let base = test_rng().gen_biguint(8);
//             let exponent = test_rng().gen_biguint(8);
//             let modulus = test_rng().gen_biguint(8);
//             let res = base.modpow(&exponent, &modulus);
//             println!("res: {}", res);
//             // Convert big integers to field elements
//             let base = Fr::from_str(&base.to_string()).unwrap();
//             let exponent = Fr::from_str(&exponent.to_string()).unwrap();
//             let modulus = Fr::from_str(&modulus.to_string()).unwrap();
//             let res = Fr::from_str(&res.to_string()).unwrap();
//             // Create the circuit
//             let circuit = ModExpCircuit {
//                 base,
//                 exponent,
//                 modulus,
//                 res,
//             };
//             // Setup the constraint system
//             let cs = ConstraintSystem::<Fr>::new_ref();
//             // Generate constraints
//             circuit.generate_constraints(cs.clone()).unwrap();
//             println!("i: {}", i);
//             // Check if the constraint system is satisfied
//             println!("is_satisfied: {}", cs.is_satisfied().unwrap());
//         }
//     }
// }

// use std::borrow::Borrow;

// use ark_bls12_381::{Fr, FrConfig};
// use ark_crypto_primitives::crh::sha256::constraints::{DigestVar, Sha256Gadget, UnitVar};
// use ark_ff::BigInteger;
// use ark_ff::{BigInt, Fp, PrimeField};
// use ark_r1cs_std::alloc::AllocVar;
// use ark_r1cs_std::boolean::Boolean;
// use ark_r1cs_std::eq::EqGadget;
// use ark_r1cs_std::fields::fp::FpVar;
// use ark_r1cs_std::select::CondSelectGadget;
// use ark_r1cs_std::uint8::UInt8;
// use ark_r1cs_std::{R1CSVar, ToBytesGadget};
// use ark_relations::ns;
// use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, Namespace, SynthesisError};
// use ark_std::test_rng;
// use ark_std::UniformRand;
// use num_bigint::{BigUint, RandBigInt, ToBigUint};
// use num_integer::Integer;
// use std::ops::{Add, AddAssign, Div, Mul, MulAssign};
// // import the miller_rabin2 function
// use crate::miller_rabin::miller_rabin_test2;
// use ark_ff::MontBackend;

// // miller rabin - https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test

// #[derive(Clone)]
// pub struct PrimeCircutNew<ConstraintF: PrimeField> {
//     pub x: Option<ConstraintF>, // public input - seed
//     pub num_of_rounds: u64,     // public input - number of rounds
//     n: FpVar<ConstraintF>,
//     d: FpVar<ConstraintF>,
//     two_to_s: FpVar<ConstraintF>,
//     s: FpVar<ConstraintF>,
//     k: usize,
//     a_to_power_d_mod_n_vec: Vec<FpVar<ConstraintF>>,
//     x_to_power_of_2_mod_n_vec: Vec<FpVar<ConstraintF>>,
//     y_vec: Vec<FpVar<ConstraintF>>,
//     is_prime: Boolean<ConstraintF>,
// }

// /*
// Create a function called miller-rabin for primality testing
// Input :
//         cs : ConstraintSystemRef<ConstraintF>
//         n - number to be tested - this is the hash converted to a Biginteger256
//         k - number of rounds - number of times to test the primality of n
//         d = n-1 = 2^s * d - a witness to the primality of n
//         s - number of times n-1 is divisible by 2 - a witness to the primality of n
//    Output: true if n is prime, false otherwise
// */
// fn miller_rabin_r1cs<ConstraintF: PrimeField>(
//     cs: ConstraintSystemRef<ConstraintF>,
//     n: FpVar<ConstraintF>,
//     d: FpVar<ConstraintF>,
//     two_to_s: FpVar<ConstraintF>,
//     s: FpVar<ConstraintF>,
//     k: usize,
//     a_to_power_d_mod_n_vec: Vec<FpVar<ConstraintF>>,
//     x_to_power_of_2_mod_n_vec: Vec<FpVar<ConstraintF>>,
//     y_vec: Vec<FpVar<ConstraintF>>,
//     is_prime: Boolean<ConstraintF>,
// ) -> Result<bool, SynthesisError> {
//     // if n is even, return false
//     let n_bigint = n.value()?.into_bigint();
//     let is_even = n_bigint.is_even();
//     let res = CondSelectGadget::conditionally_select(
//         &Boolean::new_witness(ark_relations::ns!(cs, "is_even"), || Ok(is_even))?,
//         &Boolean::constant(true),
//         &Boolean::constant(false),
//     )?;
//     if res.value()? {
//         return Ok(false);
//     }
//     // Now n is odd, we can write n-1 = 2^s * d
//     let n_minos_one = n.clone() - ConstraintF::one();
//     // enforce that n-1 = 2^s * d
//     n_minos_one.enforce_equal(&(&two_to_s * &d))?;
//     // s is the number of times n-1 is divisible by 2 - inner loop
//     let s_value = s.value()?.to_string().parse::<u64>().unwrap();
//     // now we need to check if n is prime
//     // we need to check if n is prime k times:
//     for i in 0..k {
//         // choose a random number a in the range [2, n-1]
//         let x = a_to_power_d_mod_n_vec
//             .get(i)
//             .ok_or(SynthesisError::AssignmentMissing)?;
//         for j in 0..s_value {
//             // enforce that y = x^2 mod n
//             let y = y_vec
//                 .get(j as usize)
//                 .ok_or(SynthesisError::AssignmentMissing)?;
//             let x_j_to_power_of_2_mod_n = x_to_power_of_2_mod_n_vec
//                 .get(j as usize)
//                 .ok_or(SynthesisError::AssignmentMissing)?;
//             y.enforce_equal(&x_j_to_power_of_2_mod_n)?;

//             // if y = 1 and x != 1 and x != n-1, return false
//             let one = FpVar::<ConstraintF>::new_constant(
//                 ark_relations::ns!(cs, "one"),
//                 ConstraintF::one(),
//             )?;
//             let y_is_one = y.is_eq(&one)?;
//             let x_is_one = x.is_eq(&one)?;
//             let x_is_n_minus_one = x.is_eq(&n_minos_one)?;

//             let condition = y_is_one
//                 .and(&x_is_one.not())?
//                 .and(&x_is_n_minus_one.not())?;
//             // condition == true if y = 1 and x != 1 and x != n-1 meaning its composite
//             is_prime.not().enforce_equal(&condition)?;
//             // if y != 1, return false
//             is_prime.enforce_equal(&y_is_one.not())?;
//             // now x = y
//             x.enforce_equal(&y)?;
//         }
//     }
//     // // if we didn't find a prime, return false
//     Ok(true)
// }
// pub fn miller_rabin3<ConstraintF: PrimeField>(
//     n: BigUint,
//     k: usize,
//     cs: ConstraintSystemRef<ConstraintF>,
//     circut: &mut PrimeCircutNew<ConstraintF>,
// ) -> Result<bool, SynthesisError>
// where
//     ark_ff::Fp<MontBackend<FrConfig, 4>, 4>: Borrow<ConstraintF>,
// {
//     let two: BigUint = 2.to_biguint().unwrap();
//     if n.eq(&two) {
//         circut.is_prime = Boolean::new_witness(cs.clone(), || Ok(true))?;
//     }
//     if n.is_even() {
//         circut.is_prime = Boolean::new_witness(cs.clone(), || Ok(false))?;
//     }

//     // init OutputData struct:

//     let _modulus = <Fr as PrimeField>::MODULUS;
//     // convert n to a biguint
//     let n_bigint = n.to_biguint().unwrap();
//     // convert n_bigint to a biguint:
//     let one: BigUint = 1.to_biguint().unwrap();

//     let n_minus_one = n_bigint.clone() - one.clone();
//     let _rng = rand::thread_rng();
//     let mut s = 0;
//     let _zero = 0.to_biguint().unwrap();

//     let mut d = n_bigint.clone() - one.clone();

//     // n-1 = 2^s * d
//     // I want to create witness 2^s and d such as n-1 = 2^s * d
//     // I will start by finding s:
//     while d.is_even() {
//         d = d.div(2.to_biguint().unwrap());
//         s = s + 1;
//     }
//     // create a field element for d:
//     let d_fr = Fr::from_le_bytes_mod_order(&_modulus.to_bytes_le());
//     // create witness for d :
//     let d_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(d_fr))?;
//     circut.d.add_assign(d_var.clone());

//     // create s_var:
//     let s_fr = Fr::from(s);
//     let s_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(s_fr))?;
//     circut.s.add_assign(s_var.clone());

//     // create field element for 2^s:
//     let two_to_s = two.pow(s).to_biguint().unwrap();
//     // create witness for 2^s:
//     let two_to_s_fr = Fr::from_le_bytes_mod_order(&two_to_s.to_bytes_le());
//     let two_to_s_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(two_to_s_fr))?;

//     // create a vector of a_to_power_d_mod_n_vec:
//     let mut a_to_power_d_mod_n_vec = Vec::<FpVar<ConstraintF>>::new();

//     // create a vector of x_to_power_of_2_mod_n_vec:
//     let mut x_to_power_of_2_mod_n_vec = Vec::<FpVar<ConstraintF>>::new();

//     // create a vector of y_vec:
//     let mut y_vec = Vec::<FpVar<ConstraintF>>::new();

//     // create a boolean variable is_prime:
//     let is_prime = Boolean::new_witness(cs.clone(), || Ok(true))?;

//     let mut y = 1.to_biguint().unwrap();
//     // create a vector of a_to_power_d_mod_n_vec:
//     for _i in 0..k {
//         let a = rand::thread_rng().gen_biguint_range(&two, &(&n_bigint.clone() - &two));
//         let a_to_pow_d_modn = a.modpow(&d, &n_bigint);
//         let a_fr = Fr::from_le_bytes_mod_order(&a_to_pow_d_modn.to_bytes_le());
//         let a_pow_d_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(a_fr))?;
//         a_to_power_d_mod_n_vec.push(a_pow_d_var);
//         // Working on x in the inner loop:
//         let mut x = a_to_pow_d_modn.clone();
//         for _j in 0..s {
//             y = x.modpow(&two, &n_bigint);

//             // create x^2 mod n and store it in x_to_power_of_2_mod_n_vec:
//             let x_to_power_of_2_mod_n = x.modpow(&two, &n_bigint);
//             let x_fr = Fr::from_le_bytes_mod_order(&x_to_power_of_2_mod_n.to_bytes_le());
//             let x_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(x_fr))?;
//             x_to_power_of_2_mod_n_vec.push(x_var);

//             // create y and store it in y_vec:
//             let y_fr = Fr::from_le_bytes_mod_order(&y.to_bytes_le());
//             let y_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(y_fr))?;
//             y_vec.push(y_var);

//             if one == y && x != one && x != n_minus_one {
//                 circut.is_prime = Boolean::new_witness(cs.clone(), || Ok(false))?;
//                 // return Ok(false);
//             }
//             x = y.clone();
//         }
//     }

//     // update the circut :
//     circut.s = s_var;
//     circut.d = d_var;
//     circut.a_to_power_d_mod_n_vec = a_to_power_d_mod_n_vec;
//     circut.x_to_power_of_2_mod_n_vec = x_to_power_of_2_mod_n_vec;
//     circut.y_vec = y_vec;

//     if circut.is_prime.value().unwrap() == false {
//         return Ok(false);
//     } else {
//         circut.is_prime = Boolean::new_witness(cs.clone(), || Ok(true))?;
//         return Ok(true);
//     }
// }

// // create tests for the miller_rabin_r1cs function
// #[cfg(test)]
// mod tests {
//     use core::num;

//     use super::*;
//     use ark_bls12_381::Fr;
//     use ark_r1cs_std::{alloc::AllocVar, boolean::Boolean, fields::fp::FpVar};
//     use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef};
//     use ark_std::test_rng;

//     #[test]
//     fn check_circut_creation() {
//         // create a constraint system
//         let cs = ConstraintSystem::<Fr>::new_ref();

//         // create a random number n
//         let _n = FpVar::<Fr>::new_input(cs.clone(), || Ok(Fr::from(5u64))).unwrap();
//         let _num_of_rounds = 200;
//         // create a vector of a_to_power_d_mod_n_vec:
//         let mut _a_to_power_d_mod_n_vec = Vec::<FpVar<Fr>>::new();

//         // create a vector of x_to_power_of_2_mod_n_vec:
//         let mut _x_to_power_of_2_mod_n_vec = Vec::<FpVar<Fr>>::new();

//         // create a vector of y_vec:
//         let mut _y_vec = Vec::<FpVar<Fr>>::new();
//         // let mut circut = PrimeCircutNew {
//         //     x : Some(Fr::from(10u64)),
//         //     n :_n,
//         //     num_of_rounds : _num_of_rounds,
//         //     d : Some(),

//         // }

//         // check the miller_rabin3:

//         // assert_eq!(result, true);

//         // check if the result
//     }
// }

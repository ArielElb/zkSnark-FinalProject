// use ark_ff::PrimeField;
// use ark_r1cs_std::uint64::UInt64;

// use ark_bls12_381::Fq as F;
// use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
// use ark_ff::BigInteger;
// use ark_r1cs_std::fields::fp::FpVar;
// use ark_r1cs_std::R1CSVar;
// use ark_relations::r1cs::SynthesisError;
// use rand::thread_rng;
// use sha2::Sha256;

// pub struct PrimeCircut<ConstraintF: PrimeField> {
//     x: UInt64<ConstraintF>, // input
//     y: UInt64<ConstraintF>, // output
// }
// // TODO: like this
// pub struct tryCircut<ConstraintF: PrimeField> {
//     x: FpVar<ConstraintF>,
//     y: FpVar<ConstraintF>,
// }

// pub fn hash_checker_fp<ConstraintF: PrimeField>(
//     try_circut: &tryCircut<ConstraintF>,
// ) -> Result<(), SynthesisError> {
//     // we need the modulus because we on a prime field
//     let _modulus = <F as PrimeField>::MODULUS;
//     let mut _rng = thread_rng();
//     let x = &try_circut.x;
//     // do x = x + 1

//     println!("x: {:?}", x.value().unwrap());
//     // get the
//     let a: ConstraintF = x.value().unwrap();
//     let bigint = a.into_bigint();
//     println!("a: {:?}", a);
//     let hasher = <DefaultFieldHasher<Sha256> as HashToField<F>>::new(&[]);
//     let preimage = bigint.to_bytes_be(); // Converting to big-endian
//     let hashes: Vec<F> = hasher.hash_to_field(&preimage, 1); // Returned vector is of size 2
//                                                              // take the actual number of the hash[0]
//     let hash = hashes[0];
//     println!("hash: {:?}", hash);
//     let res = hash;
//     println!("res: {:?}", res);

//     Ok(())
// }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn test_hash_number() {
// //         use ark_ff::{Zero,One};
// //         // create a new constraint system
// //         let cs = ConstraintSystem::<F>::new_ref();
// //         // create a new prime generator
// //         let mut z: FpVar<F> = FpVar::<F>::new_variable(cs.clone(), || Ok(F::one()), ark_r1cs_std::alloc::AllocationMode::Constant).unwrap();
// //         let mut var = FpVar::<F>::new_variable(cs.clone(), || Ok(F::zero()), ark_r1cs_std::alloc::AllocationMode::Input).unwrap();

// //         let mut try_circut = tryCircut::<F> {
// //             x: var,
// //             y:z
// //         };

// //         try_circut.x += &FpVar::<F>::new_constant(cs.clone(), F::one()).unwrap();
// //         try_circut.x *= &FpVar::<F>::new_constant(cs.clone(), F::from(2u64)).unwrap();

// //         try_circut.x = ark_r1cs_std::fields::fp::FpVar::Constant(try_circut.x.value().unwrap().pow([5u64]));

// //         // print the typeof x:
// //         println!("x: {:?}", try_circut.x);

// //         hash_checker_fp::<F>(&try_circut).unwrap();

// //     }
// // }

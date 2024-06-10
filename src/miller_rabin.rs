use ark_bls12_381::Fq as F;
use ark_ff::PrimeField;
use ark_r1cs_std::fields::fp::FpVar;
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_integer::Integer;
use std::ops::Div;
pub fn miller_rabin_test2(n: BigUint, _k: usize) -> bool {
    let two: BigUint = 2.to_biguint().unwrap();
    if n.eq(&two) {
        return true;
    }
    if n.is_even() {
        return false;
    }
    let _modulus = <F as PrimeField>::MODULUS;
    // convert n to a biguint
    let n_bigint = n.to_biguint().unwrap();
    // convert n_bigint to a biguint:
    let one: BigUint = 1.to_biguint().unwrap();

    let n_minus_one = n_bigint.clone() - one.clone();
    let _rng = rand::thread_rng();
    let mut s = 0;
    let _zero = 0.to_biguint().unwrap();

    let mut d = n_bigint.clone() - one.clone();
    // n-1 = 2^s * d
    while d.is_even() {
        d = d.div(2.to_biguint().unwrap());
        s = s + 1;
    }
    let mut y = 1.to_biguint().unwrap();
    let ubound = n_bigint.clone() - &2.to_biguint().unwrap();
    let lbound = &2.to_biguint().unwrap();
    for _ in 0.._k {
        let mut rng = rand::thread_rng();
        let a = rng.gen_biguint_range(&lbound, &ubound);
        let mut x = a.modpow(&d, &n_bigint);
        for _j in 0..s {
            y = x.modpow(&two, &n_bigint);

            if one == y && x != one && x != n_minus_one {
                return false;
            }
            x = y.clone();
        }
        if y != one {
            return false;
        }
    }
    return true;
}

// create miller_rabin in  R1CS that get the ConstraintSystemRef and the number to check if it is prime and a witness s and d such as n-1 = 2^s * d
// pub fn miller_rabin_r1cs(cs:)
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ark_bls12_381::{Bls12_381, Fr as BlsFr};
//     use ark_poly::univariate::DensePolynomial;
//     use ark_poly_commit::marlin_pc::MarlinKZG10;
//     use ark_std::{ops::*, UniformRand};
//     use ark_relations::r1cs::ConstraintSynthesizer;
//     use ark_relations::r1cs::ConstraintSystem;
//     use rand::{rngs::StdRng, SeedableRng};
//     use rand_chacha::ChaChaCore;
//     #[test]
//     fn test_prime_native() {
//         assert!(!miller_rabin_test2(10.to_biguint().unwrap(), 1));
//         assert!(miller_rabin_test2(7.to_biguint().unwrap(), 1));
//         assert!(miller_rabin_test2(11.to_biguint().unwrap(), 1));
//         assert!(!miller_rabin_test2(15.to_biguint().unwrap(), 1));
//         assert!(!miller_rabin_test2(21.to_biguint().unwrap(), 1))
//     }

// }

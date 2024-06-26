use ark_bls12_381::Fr;
use ark_bls12_381::FrConfig;
use ark_ff::biginteger;
use ark_ff::BigInteger;
use ark_ff::MontBackend;
use ark_ff::PrimeField;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::{boolean::Boolean, fields::fp::FpVar};
use ark_relations::r1cs::ConstraintSystemRef;
use ark_relations::r1cs::SynthesisError;
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_integer::Integer;
use std::borrow::Borrow;
use std::ops::Div;

// Struct of output data:
struct OutputData<ConstraintF: PrimeField> {
    d: FpVar<ConstraintF>,
    two_to_s: FpVar<ConstraintF>,
    s: FpVar<ConstraintF>,
    k: usize,
    a_to_power_d_mod_n_vec: Vec<FpVar<ConstraintF>>,
    x_to_power_of_2_mod_n_vec: Vec<FpVar<ConstraintF>>,
    y_vec: Vec<FpVar<ConstraintF>>,
    is_prime: Boolean<ConstraintF>,
}

pub fn miller_rabin_test2(n: BigUint, _k: usize) -> bool {
    let two: BigUint = 2.to_biguint().unwrap();
    if n.eq(&two) {
        return true;
    }
    if n.is_even() {
        return false;
    }
    // init OutputData struct:

    let _modulus = <Fr as PrimeField>::MODULUS;
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
    // I want to create witness 2^s and d such as n-1 = 2^s * d
    // I will start by finding s:
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

pub fn miller_rabin3<ConstraintF: PrimeField>(
    n: BigUint,
    k: usize,
    cs: ConstraintSystemRef<ConstraintF>,
) -> Result<bool, SynthesisError>
where
    ark_ff::Fp<MontBackend<FrConfig, 4>, 4>: Borrow<ConstraintF>,
{
    let two: BigUint = 2.to_biguint().unwrap();
    if n.eq(&two) {
        return Ok(true);
    }
    if n.is_even() {
        return Ok(false);
    }
    // init OutputData struct:

    let _modulus = <Fr as PrimeField>::MODULUS;
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
    // I want to create witness 2^s and d such as n-1 = 2^s * d
    // I will start by finding s:
    while d.is_even() {
        d = d.div(2.to_biguint().unwrap());
        s = s + 1;
    }
    // create a field element for d:
    let d_fr = Fr::from_le_bytes_mod_order(&_modulus.to_bytes_le());
    // create witness for d :
    let d_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(d_fr))?;

    // create s_var:
    let s_fr = Fr::from(s);
    let s_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(s_fr))?;

    // create field element for 2^s:
    let two_to_s = two.pow(s).to_biguint().unwrap();
    // create witness for 2^s:
    let two_to_s_fr = Fr::from_le_bytes_mod_order(&two_to_s.to_bytes_le());
    let two_to_s_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(two_to_s_fr))?;

    // create a vector of a_to_power_d_mod_n_vec:
    let mut a_to_power_d_mod_n_vec = Vec::<FpVar<ConstraintF>>::new();

    // create a vector of x_to_power_of_2_mod_n_vec:
    let mut x_to_power_of_2_mod_n_vec = Vec::<FpVar<ConstraintF>>::new();

    // create a vector of y_vec:
    let mut y_vec = Vec::<FpVar<ConstraintF>>::new();

    // create a boolean variable is_prime:
    let is_prime = Boolean::new_witness(cs.clone(), || Ok(true))?;

    let mut y = 1.to_biguint().unwrap();
    // create a vector of a_to_power_d_mod_n_vec:
    for _i in 0..k {
        let a = rand::thread_rng().gen_biguint_range(&two, &(&n_bigint.clone() - &two));
        let a_to_pow_d_modn = a.modpow(&d, &n_bigint);
        let a_fr = Fr::from_le_bytes_mod_order(&a_to_pow_d_modn.to_bytes_le());
        let a_pow_d_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(a_fr))?;
        a_to_power_d_mod_n_vec.push(a_pow_d_var);
        // Working on x in the inner loop:
        let mut x = a_to_pow_d_modn.clone();
        for _j in 0..s {
            y = x.modpow(&two, &n_bigint);

            // create x^2 mod n and store it in x_to_power_of_2_mod_n_vec:
            let x_to_power_of_2_mod_n = x.modpow(&two, &n_bigint);
            let x_fr = Fr::from_le_bytes_mod_order(&x_to_power_of_2_mod_n.to_bytes_le());
            let x_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(x_fr))?;
            x_to_power_of_2_mod_n_vec.push(x_var);

            // create y and store it in y_vec:
            let y_fr = Fr::from_le_bytes_mod_order(&y.to_bytes_le());
            let y_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(y_fr))?;
            y_vec.push(y_var);

            if one == y && x != one && x != n_minus_one {
                return Ok(false);
            }
            x = y.clone();
        }
    }
    let is_prime_var = Boolean::new_witness(cs.clone(), || Ok(true))?;
    // Create the output data:

    let output_data = OutputData {
        d: d_var,
        two_to_s: two_to_s_var,
        s: s_var,
        k,
        a_to_power_d_mod_n_vec,
        x_to_power_of_2_mod_n_vec,
        y_vec,
        is_prime: is_prime_var,
    };
    Ok(true)
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

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::Fr;
    use ark_r1cs_std::{alloc::AllocVar, boolean::Boolean, fields::fp::FpVar};
    use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef};
    use ark_std::test_rng;

    #[test]
    fn check_soundess_constraints() {
        // create a constraint system
        let cs = ConstraintSystem::<Fr>::new_ref();

        let num = 17.to_biguint().unwrap();
        // create a random number n
        let n = FpVar::<Fr>::new_input(cs.clone(), || {
            Ok(Fr::from_le_bytes_mod_order(&num.to_bytes_le()))
        })
        .unwrap();

        // check miller_rabin3 function:
        let result = miller_rabin3(num, 1, cs).unwrap();

        assert_eq!(result, true);

        // assert_eq!(result, true);
    }
}

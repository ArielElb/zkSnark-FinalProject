use ark_bls12_381::Fr;
use ark_ff::PrimeField;
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::select::CondSelectGadget;
use ark_r1cs_std::R1CSVar;
use ark_r1cs_std::{alloc::AllocVar, ToBitsGadget};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use num_bigint::BigUint;
use std::str::FromStr;

const NUM_BITS: usize = 32;

// for each mod reduction we need to store the number, the quotient and the remainder
#[derive(Clone)]
pub struct mod_vals {
    pub num: BigUint,
    pub q: BigUint,
    pub remainder: BigUint,
}

pub struct mod_witnesses<F: PrimeField> {
    num: F,
    div: F,
    q: F,
    remainder: F,
}
pub struct return_struct {
    pub mod_vals: Vec<mod_vals>, //
    pub mod_pow_vals: Vec<mod_vals>,
    pub result: BigUint,
    pub bits: Vec<u8>,
}

fn get_mod_vals(num: &BigUint, div: &BigUint) -> mod_vals {
    let q = num / div;
    let remainder = num % div;
    mod_vals {
        num: num.clone(), // Still necessary to clone if ownership is needed outside
        q,
        remainder,
    }
}
pub fn mod_pow_generate_witnesses(base: BigUint, div: BigUint, exp: BigUint) -> return_struct {
    let mut cur_pow = base.clone();
    let mut exp_val = exp.clone();
    let mut res = BigUint::from(1u8);
    let one = BigUint::from(1u8);
    let zero = BigUint::from(0u8);
    let def_val = mod_vals {
        num: zero.clone(),
        q: zero.clone(),
        remainder: zero.clone(),
    };
    let mut v: Vec<mod_vals> = vec![def_val.clone(); NUM_BITS];
    let mut mod_pow_vals: Vec<mod_vals> = vec![def_val; NUM_BITS];
    let mut bits: Vec<u8> = vec![0u8; NUM_BITS];

    // index for writing to the vectors
    let mut counter = 0;

    while exp_val > zero {
        let bit = &exp_val & &one;
        bits[counter] = if bit == one { 1 } else { 0 };

        // Debugging prints
        println!("Iteration: {}", counter);
        println!("bit: {}", bit);
        println!("cur_pow: {}", cur_pow);
        println!("res before: {}", res);

        // If the current bit is 1, multiply the result by the current power of base
        if bit == one {
            res = (res * &cur_pow) % &div;
        }

        // Debugging prints
        println!("res after: {}", res);
        println!("div: {}", div);

        // Store the current result
        v[counter] = get_mod_vals(&res, &div);

        // Square the current power of base for the next bit
        cur_pow = (&cur_pow * &cur_pow) % &div;

        // Store the current power of base
        mod_pow_vals[counter] = get_mod_vals(&cur_pow, &div);

        exp_val >>= 1;
        counter += 1;
    }

    let retstruct = return_struct {
        mod_vals: v,
        mod_pow_vals,
        result: res,
        bits,
    };

    return retstruct;
}
#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::*;
    use ark_relations::r1cs::ConstraintSystem;
    use num_bigint::BigUint;
    use rand::{thread_rng, Rng};

    fn generate_random_biguint(num_bytes: usize) -> BigUint {
        let mut rng = thread_rng();
        let mut bytes = vec![0u8; num_bytes];
        rng.fill(&mut bytes[..]); // Fill the vector with random bytes
        BigUint::from_bytes_le(&bytes) // Convert bytes to a BigUint
    }
    // this test generate witnesses for the modpow circuit and printing the witnesses
    #[test]
    fn check_witnesses() {
        // Example parameters (for testing purposes)
        let base = BigUint::from(10u64);
        let exponent = BigUint::from(3u64);
        let modulus = BigUint::from(2u64);
        let res: BigUint = base.modpow(&exponent, &modulus);
        println!("res: {}", res);

        let witneses = mod_pow_generate_witnesses(base.clone(), modulus.clone(), exponent.clone());
        // print the witnesses nicely:

        // mod_vals is the values of the result after each multiplication
        println!("mod_vals:");
        for i in 0..witneses.mod_vals.len() {
            println!(
                "num: {}, q: {}, remainder: {}",
                witneses.mod_vals[i].num, witneses.mod_vals[i].q, witneses.mod_vals[i].remainder
            );
        }
        // mod_pow_vals is the values of the base after each squaring
        println!("mod_pow_vals:");
        for i in 0..witneses.mod_pow_vals.len() {
            println!(
                "num: {}, q: {}, remainder: {}",
                witneses.mod_pow_vals[i].num,
                witneses.mod_pow_vals[i].q,
                witneses.mod_pow_vals[i].remainder
            );
        }
        println!("result: {}", witneses.result);
        println!("bits: {:?}", witneses.bits);
    }

    #[test]
    fn mod_pow_tests() {
        //  10^2 mod 3 = 1 // number of reductions at most is 100 / 3 = 33
        // Example parameters (for testing purposes)
        //let base = BigUint::from(11231235u64);
        //let exponent = BigUint::from(2002100u64);
        //let modulus = BigUint::from(10000310u64);
        let base = generate_random_biguint(47);
        let exponent = generate_random_biguint(47);
        let modulus = generate_random_biguint(47);

        let res = base.modpow(&exponent, &modulus);
        println!("res: {}", res);
        // Convert big integers to field elements
        let mut average_dur = 0;

        for i in 0..10 {
            let start = Instant::now();
            let res2 = mod_pow_generate_witnesses(base.clone(), modulus.clone(), exponent.clone());
            let duration = start.elapsed();
            average_dur += duration.as_millis();
            println!("cur dur {:?}", duration);
        }
        average_dur /= 1;
        println!("Time taken: {:?}", average_dur); // Prints time
                                                   //println!("res2: {}", res2);
                                                   //assert!(res == res2);
    }
}

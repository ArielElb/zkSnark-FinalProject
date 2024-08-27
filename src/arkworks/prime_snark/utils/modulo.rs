use super::constants;
use ark_r1cs_std::{alloc::AllocVar, ToBitsGadget};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use num_bigint::BigUint;
const NUM_BITS: usize = constants::NUM_BITS;

#[derive(Clone, Debug)]
pub struct ModVals {
    pub num: BigUint,
    pub q: BigUint,
    pub remainder: BigUint,
}

pub struct ReturnStruct {
    pub mod_vals: Vec<ModVals>,
    pub mod_pow_vals: Vec<ModVals>,
    pub result: BigUint,
    pub bits: Vec<u8>,
}

fn get_mod_vals(num: &BigUint, div: &BigUint) -> ModVals {
    let q = num / div;
    let remainder = num % div;
    ModVals {
        num: num.clone(), // Still necessary to clone if ownership is needed outside
        q,
        remainder,
    }
}
pub fn mod_pow_generate_witnesses(base: BigUint, div: BigUint, exp: BigUint) -> ReturnStruct {
    println!("base: {}", base);
    println!("div: {}", div);
    println!("exp: {}", exp);
    let mut elem;
    let mut cur_pow = base.clone();
    let mut power = base.clone();
    let mut exp_val = exp.clone();
    let mut res = BigUint::from(1u8);
    let one = BigUint::from(1u8);
    let zero = BigUint::from(0u8);
    let def_val = ModVals {
        num: zero.clone(),
        q: zero.clone(),
        remainder: zero.clone(),
    };
    let mut v: Vec<ModVals> = vec![def_val.clone(); NUM_BITS];
    let mut mod_pow_vals: Vec<ModVals> = vec![def_val; NUM_BITS];
    let mut bits: Vec<u8> = vec![0u8; NUM_BITS];
    for i in 0..NUM_BITS {
        power = power.clone() * power;
        mod_pow_vals[i] = get_mod_vals(&power, &div);
        power %= &div;
    }
    let mut counter = 0;
    while exp_val > zero {
        elem = &exp_val & &one;
        //println!("elem is {}",elem);
        if elem == one {
            bits[counter] = 1;
        }
        //
        res *= (&cur_pow - &one) * elem + &one;
        ////println!("res is: {}", res);
        v[counter] = get_mod_vals(&res, &div);
        if res > div {
            res %= &div;
        }
        exp_val >>= 1;
        cur_pow *= cur_pow.clone();
        //mod_pow_vals[counter] = get_mod_vals(&cur_pow, &div);
        cur_pow %= &div;
        //cur_pow=square_biguint(&cur_pow);

        counter += 1;
    }
    for i in 0..NUM_BITS - counter {
        v[i + counter] = ModVals {
            num: res.clone(),
            q: BigUint::from(0u64),
            remainder: res.clone(),
        }
    }
    let retstuct = ReturnStruct {
        mod_vals: v,
        mod_pow_vals,
        result: res,
        bits,
    };

    return retstuct;
}

#[cfg(test)]
mod test {
    use std::time::Instant;

    use super::*;
    use ark_relations::r1cs::ConstraintSystem;
    use num_bigint::BigUint;
    use rand::{thread_rng, Rng};
    /*
    #[test]
    fn test_mod_exp_circuit() {
        // 3^5 mod 7 = 5
        // the maximum number of reductions is :  3^7 / 7 = 2187 / 7 = 312
        // Example parameters (for testing purposes)
        let base = BigUint::from(3u64);
        let exponent = BigUint::from(5u64);
        let modulus = BigUint::from(7u64);
        let res = base.modpow(&exponent, &modulus);

        // Convert big integers to field elements
        let base = Fr::from_str(&base.to_string()).unwrap();
        let exponent = Fr::from_str(&exponent.to_string()).unwrap();
        let modulus = Fr::from_str(&modulus.to_string()).unwrap();
        let res = Fr::from_str(&res.to_string()).unwrap();

        println!("base: {}", base);
        println!("exponent: {}", exponent);
        println!("modulus: {}", modulus);
        println!("res: {}", res);

        // Create the circuit
        let circuit = ModExpCircuit {
            base,
            exponent,
            modulus,
            res,
        };

        // Setup the constraint system
        let cs = ConstraintSystem::<Fr>::new_ref();

        // Generate constraints
        circuit.generate_constraints(cs.clone()).unwrap();

        // Check if the constraint system is satisfied
        assert!(cs.is_satisfied().unwrap());
    }
    */
    /*
    #[test]
    fn second_example() {
        //  10^2 mod 3 = 1 // number of reductions at most is 100 / 3 = 33
        // Example parameters (for testing purposes)
        let base = BigUint::from(10u64);
        let exponent = BigUint::from(2u64);
        let modulus = BigUint::from(3u64);
        let res = base.modpow(&exponent, &modulus);
        println!("res: {}", res);
        // Convert big integers to field elements
        let base = Fr::from_str(&base.to_string()).unwrap();
        let exponent = Fr::from_str(&exponent.to_string()).unwrap();
        let modulus = Fr::from_str(&modulus.to_string()).unwrap();
        let res = Fr::from_str(&res.to_string()).unwrap();
        // Create the circuit
        let circuit = ModExpCircuit {
            base,
            exponent,
            modulus,
            res,
        };

        // Setup the constraint system
        let cs = ConstraintSystem::<Fr>::new_ref();
        // Generate constraints
        circuit.generate_constraints(cs.clone()).unwrap();

        // Check if the constraint system is satisfied
        assert!(cs.is_satisfied().unwrap());
    }
     */
    fn generate_random_biguint(num_bytes: usize) -> BigUint {
        let mut rng = thread_rng();
        let mut bytes = vec![0u8; num_bytes];
        rng.fill(&mut bytes[..]); // Fill the vector with random bytes
        BigUint::from_bytes_le(&bytes) // Convert bytes to a BigUint
    }
    #[test]
    fn mod_pow_tests() {
        //  10^2 mod 3 = 1 // number of reductions at most is 100 / 3 = 33
        // Example parameters (for testing purposes)
        let base = BigUint::from(5u64);
        let exponent = BigUint::from(3u64);
        let modulus = BigUint::from(4u64);
        //let base = generate_random_biguint(47);
        //let exponent= generate_random_biguint(47);
        //let modulus= generate_random_biguint(47);

        let res = base.modpow(&exponent, &modulus);
        println!("res: {}", res);
        // Convert big integers to field elements
        let mut average_dur = 0;

        for i in 0..1 {
            let start = Instant::now();
            let res2 = mod_pow_generate_witnesses(base.clone(), modulus.clone(), exponent.clone());
            let result = res2.result;
            assert!(res == result);
            println!("{}", result);
            //println!("mods vals is: {:?}",res2.mod_vals);
            println!("mod pow vals is: {:?}", res2.mod_pow_vals);
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

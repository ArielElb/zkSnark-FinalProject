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

const NUM_REDUCTIONS: usize = 33; // Set an appropriate upper bound on the number of reductions
const NUM_BITS:usize = 381;

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
    pub mod_vals: Vec<mod_vals>,
    pub mod_pow_vals: Vec<mod_vals>,
    pub result: BigUint,
}
struct ModExpCircuit<F: PrimeField> {
    base: F,
    exponent: F,
    modulus: F,
    res: F,
}

fn get_mod_vals(num: &BigUint, div: &BigUint) -> mod_vals {
    let q = num / div;
    let remainder = num - div * &q;
    mod_vals {
        num: num.clone(),  // Still necessary to clone if ownership is needed outside
        q,
        remainder,
    }
}


pub fn mod_pow_generate_witnesses(base: BigUint, div: BigUint, exp:BigUint)->return_struct{
    let mut elem;
    let mut cur_pow = base.clone(); 
    let mut exp_val = exp.clone();
    let mut res = BigUint::from(1u8);
    let one =  BigUint::from(1u8);
    let zero = BigUint::from(0u8);
    let def_val = mod_vals{
        num:zero.clone(),
        q:zero.clone(),
        remainder:zero.clone(),
    };
    let mut v: Vec<mod_vals> =  vec![def_val.clone(); 382];
    let mut mod_pow_vals: Vec<mod_vals> =  vec![def_val; 382];
    let mut bits: Vec<u8> =  vec![0u8; 382];

    let mut counter = 0;
    while exp_val>zero{
        elem = &exp_val & &one;
        //println!("elem is {}",elem);
        if elem == one{
            bits[counter] = 1;
        }
//
        res *= (elem - &one)*&cur_pow + &one;
        ////println!("res is: {}", res);
        v[counter] = get_mod_vals(&res, &div);
        if res > div {
            res %= &div;
        }
        exp_val >>= 1;
        cur_pow *= cur_pow.clone();
        mod_pow_vals[counter] = get_mod_vals(&cur_pow, &div);
        cur_pow %= &div;
        //cur_pow=square_biguint(&cur_pow);

        counter += 1;
    }
    let retstuct = return_struct{
        mod_vals: v,
        mod_pow_vals,
        result: res,
    };

    return retstuct;
}

fn reduce_mod<F: PrimeField>(
    cs: ConstraintSystemRef<F>,
    result: &FpVar<F>,
    modulus: &FpVar<F>,
) -> Result<FpVar<F>, SynthesisError> {
    // Check if result >= modulus
    let should_reduce = result.is_cmp_unchecked(modulus, std::cmp::Ordering::Greater, true)?;

    // Subtract modulus from result if should_reduce is true
    let reduced_result = result.clone() - modulus;

    // Conditionally select between reduced_result and result based on should_reduce
    let final_result = FpVar::conditionally_select(&should_reduce, &reduced_result, result)?;

    Ok(final_result)
}

impl<F: PrimeField> ConstraintSynthesizer<F> for ModExpCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        // Allocate the base, exponent, and modulus as public inputs
        let base = FpVar::new_input(cs.clone(), || Ok(self.base))?;
        let exponent = FpVar::new_input(cs.clone(), || Ok(self.exponent))?;
        let modulus = FpVar::new_input(cs.clone(), || Ok(self.modulus))?;
        let zero = FpVar::constant(F::zero());

        // Initialize the result as 1
        let mut result = FpVar::constant(F::one());

        // Convert exponent to bits and perform modular exponentiation
        let exponent_bits = exponent.to_bits_le()?;
        for bit in exponent_bits.iter().rev() {
            // Square the current result 
            result = result.clone() * &result;

            // Reduce result modulo the modulus a fixed number of times
            for _ in 0..NUM_REDUCTIONS {
                result = reduce_mod(cs.clone(), &result, &modulus)?;
            }

            // Multiply by base if the current bit is 1
            result = bit.select(&(result.clone() * &base), &result)?;

            // Reduce result modulo the modulus again a fixed number of times
            for _ in 0..NUM_REDUCTIONS {
                result = reduce_mod(cs.clone(), &result, &modulus)?;
            }
        }

        // Enforce the final result as a public input
        let final_result = FpVar::new_input(cs.clone(), || Ok(result.value().unwrap()))?;

        // Enforce the final result to be equal to the expected result
        final_result.enforce_equal(&FpVar::new_input(cs.clone(), || Ok(self.res))?)?;

        Ok(())
    }
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
        rng.fill(&mut bytes[..]);  // Fill the vector with random bytes
        BigUint::from_bytes_le(&bytes)  // Convert bytes to a BigUint
    }
    #[test]
    fn mod_pow_tests() {
        //  10^2 mod 3 = 1 // number of reductions at most is 100 / 3 = 33
        // Example parameters (for testing purposes)
        //let base = BigUint::from(11231235u64);
        //let exponent = BigUint::from(2002100u64);
        //let modulus = BigUint::from(10000310u64);
        let base = generate_random_biguint(47);
        let exponent= generate_random_biguint(47);
        let modulus= generate_random_biguint(47);    
        
        let res = base.modpow(&exponent, &modulus);
        println!("res: {}", res);
        // Convert big integers to field elements
        let mut average_dur = 0;
        
        for i in 0..10{
            let start = Instant::now();
            let res2 = mod_pow_generate_witnesses(base.clone(),modulus.clone(),exponent.clone());
            let duration = start.elapsed();
            average_dur += duration.as_millis();
            println!("cur dur {:?}",duration);
        }
        average_dur /= 1;
        println!("Time taken: {:?}", average_dur); // Prints time
        //println!("res2: {}", res2);
        //assert!(res == res2);
    }
     
}
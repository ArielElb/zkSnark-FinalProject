use ark_bls12_381::Fr;
use ark_crypto_primitives::crh::sha256::constraints::{DigestVar, Sha256Gadget, UnitVar};
use ark_ff::BigInteger;
use ark_ff::{BigInt, Fp, PrimeField};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::select::CondSelectGadget;
use ark_r1cs_std::uint8::UInt8;
use ark_r1cs_std::{R1CSVar, ToBytesGadget};
use ark_relations::ns;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, Namespace, SynthesisError};
use ark_std::test_rng;
use ark_std::UniformRand;
// import the miller_rabin2 function
use crate::miller_rabin::miller_rabin_test2;

// miller rabin - https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
/*
Create a function called miller-rabin for primality testing
Input :
        cs : ConstraintSystemRef<ConstraintF>
        n - number to be tested - this is the hash converted to a Biginteger256
        k - number of rounds - number of times to test the primality of n
        d = n-1 = 2^s * d - a witness to the primality of n
        s - number of times n-1 is divisible by 2 - a witness to the primality of n
   Output: true if n is prime, false otherwise
*/
fn miller_rabin_r1cs<ConstraintF: PrimeField>(
    cs: ConstraintSystemRef<ConstraintF>,
    n: FpVar<ConstraintF>,
    d: FpVar<ConstraintF>,
    two_to_s: FpVar<ConstraintF>,
    s: FpVar<ConstraintF>,
    k: usize,
    a_to_power_d_mod_n_vec: Vec<FpVar<ConstraintF>>,
    x_to_power_of_2_mod_n_vec: Vec<FpVar<ConstraintF>>,
    y_vec: Vec<FpVar<ConstraintF>>,
    is_prime: Boolean<ConstraintF>,
) -> Result<bool, SynthesisError> {
    // if n is even, return false
    let n_bigint = n.value()?.into_bigint();
    let is_even = n_bigint.is_even();
    let res = CondSelectGadget::conditionally_select(
        &Boolean::new_witness(ark_relations::ns!(cs, "is_even"), || Ok(is_even))?,
        &Boolean::constant(true),
        &Boolean::constant(false),
    )?;
    if res.value()? {
        return Ok(false);
    }
    // Now n is odd, we can write n-1 = 2^s * d
    let n_minos_one = n.clone() - ConstraintF::one();
    // enforce that n-1 = 2^s * d
    n_minos_one.enforce_equal(&(&two_to_s * &d))?;
    // s is the number of times n-1 is divisible by 2 - inner loop
    let s_value = s.value()?.to_string().parse::<u64>().unwrap();
    // now we need to check if n is prime
    // we need to check if n is prime k times:
    for i in 0..k {
        // choose a random number a in the range [2, n-1]
        let x = a_to_power_d_mod_n_vec
            .get(i)
            .ok_or(SynthesisError::AssignmentMissing)?;
        for j in 0..s_value {
            // enforce that y = x^2 mod n
            let y = y_vec
                .get(j as usize)
                .ok_or(SynthesisError::AssignmentMissing)?;
            let x_j_to_power_of_2_mod_n = x_to_power_of_2_mod_n_vec
                .get(j as usize)
                .ok_or(SynthesisError::AssignmentMissing)?;
            y.enforce_equal(&x_j_to_power_of_2_mod_n)?;

            // if y = 1 and x != 1 and x != n-1, return false
            let one = FpVar::<ConstraintF>::new_constant(
                ark_relations::ns!(cs, "one"),
                ConstraintF::one(),
            )?;
            let y_is_one = y.is_eq(&one)?;
            let x_is_one = x.is_eq(&one)?;
            let x_is_n_minus_one = x.is_eq(&n_minos_one)?;

            let condition = y_is_one
                .and(&x_is_one.not())?
                .and(&x_is_n_minus_one.not())?;
            // condition == true if y = 1 and x != 1 and x != n-1 meaning its composite
            is_prime.not().enforce_equal(&condition)?;
            // if y != 1, return false
            is_prime.enforce_equal(&y_is_one.not())?;
            // now x = y
            x.enforce_equal(&y)?;
        }
    }
    // // if we didn't find a prime, return false
    Ok(true)
}

// create tests for the miller_rabin_r1cs function
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

        // create a random number n
        let n = FpVar::<Fr>::new_input(cs.clone(), || Ok(Fr::from(5u64))).unwrap();

        // let is_prime = Boolean::new_witness(cs.clone(), || {

        // })
        // .unwrap();

        // println!("is_prime: {:?}", is_prime.value().unwrap());
        // // call the miller_rabin_r1cs function
        // let result = super::miller_rabin_r1cs(
        //     cs.clone(),
        //     n,
        //     d,
        //     two_to_s,
        //     s,
        //     k,
        //     a_to_power_d_mod_n_vec,
        //     x_to_power_of_2_mod_n_vec,
        //     y_vec,
        //     is_prime,
        // )
        // .unwrap();

        // assert_eq!(result, true);

        // check if the result
    }
}

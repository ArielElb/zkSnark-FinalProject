use std::borrow::Borrow;

use ark_bls12_381::{Fr, FrConfig};
use ark_crypto_primitives::crh::sha256::constraints::{DigestVar, Sha256Gadget, UnitVar};
use ark_ff::BigInteger;
use ark_ff::{BigInt, Fp, PrimeField};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::select::CondSelectGadget;
use ark_r1cs_std::uint8::UInt8;
use ark_r1cs_std::{R1CSVar, ToBytesGadget};
use ark_relations::ns;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, Namespace, SynthesisError};
use ark_std::test_rng;
use ark_std::UniformRand;
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_integer::Integer;
use num_traits::cast::ToPrimitive;

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Shl, Shr};

// import the miller_rabin2 function
use crate::miller_rabin::miller_rabin_test2;
use ark_ff::MontBackend;

// miller rabin - https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test

#[derive(Clone)]
pub struct Modulo<ConstraintF: PrimeField> {
    x: FpVar<ConstraintF>, // divided num
    q: FpVar<ConstraintF>, // div res
    r: FpVar<ConstraintF>, //remainder
    y: FpVar<ConstraintF>, //divisor
}

#[derive(Clone)]
// create new circut as values of ConstraintF and not FpVar<ConstraintF>:
pub struct PrimeCircutNotFpVar<ConstraintF: PrimeField> {
    pub x: Option<ConstraintF>,                  // public input - seed
    pub num_of_rounds: u64,                      // public input - number of rounds
    n: ConstraintF,                              // number to be tested
    d: ConstraintF,                              // n-1 = 2^s * d
    two_to_s: ConstraintF,                       // 2^s
    s: ConstraintF,                              // number of times n-1 is divisible by 2
    k: usize,                                    // number of rounds
    a_vector: Vec<ConstraintF>, // a -  vector of random numbers in the range [2, n-2]
    a_to_power_d_mod_n_vec: Vec<ConstraintF>, // a^d mod n
    x_to_power_of_2_mod_n_vec: Vec<ConstraintF>, // x^2 mod n
    // vec of stucts of Modulo for a^d mod n vec
    modulo_a_to_pow_d: Vec<Modulo<ConstraintF>>, // vec of stucts of Modulo for x^2 mod n vec
    modulo_x_to_pow_2: Vec<Modulo<ConstraintF>>, //  vec of stucts of Modulo for x^2 mod n vec
    y_vec: Vec<ConstraintF>,                     // y
    is_prime: bool,                              // true if n is prime, false otherwise
}

// [.........] 256 bits   1x1 + 0/1x
impl<ConstraintF: PrimeField> PrimeCircutNotFpVar<ConstraintF> {
    pub fn miller_rabin_r1cs_using_circut_val(
        &self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // convert the circut values to FpVar<ConstraintF>:
        let n = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.n))?;
        let d = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.d))?;
        let two_to_s = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.two_to_s))?;
        let s = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.s))?;
        let k = self.k;
        let a_vec = self
            .a_vector
            .iter()
            .map(|a| FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(*a)))
            .collect::<Result<Vec<FpVar<ConstraintF>>, SynthesisError>>()?;
        let a_to_power_d_mod_n_vec = self
            .a_to_power_d_mod_n_vec
            .iter()
            .map(|a| FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(*a)))
            .collect::<Result<Vec<FpVar<ConstraintF>>, SynthesisError>>()?;
        let x_to_power_of_2_mod_n_vec = self
            .x_to_power_of_2_mod_n_vec
            .iter()
            .map(|a| FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(*a)))
            .collect::<Result<Vec<FpVar<ConstraintF>>, SynthesisError>>()?;
        let y_vec = self
            .y_vec
            .iter()
            .map(|a| FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(*a)))
            .collect::<Result<Vec<FpVar<ConstraintF>>, SynthesisError>>()?;
        let is_prime = Boolean::new_witness(cs.clone(), || Ok(self.is_prime))?;
        // Now use the same logic as in the miller_rabin_r1cs function:
        miller_rabin_r1cs(
            cs,
            n,
            d,
            two_to_s,
            s,
            k,
            a_vec,
            a_to_power_d_mod_n_vec,
            x_to_power_of_2_mod_n_vec,
            y_vec,
            is_prime,
        )?;
        Ok(())
    }
}
// implement the ConstraintSynthesizer trait for the PrimeCircutNotFpVar struct:
fn modulo<ConstraintF: PrimeField>(
    x: FpVar<ConstraintF>, // divided num
    q: FpVar<ConstraintF>, // div res
    r: FpVar<ConstraintF>, //remainder
    y: FpVar<ConstraintF>, //divisor
) -> Result<(), SynthesisError> {
    r.is_cmp(&y, std::cmp::Ordering::Less, true)?
        .enforce_equal(&Boolean::constant(true))?;
    let mul = y * q;
    let num = mul + r;
    let _ = num.enforce_equal(&x);
    Ok(())
}

fn pow<ConstraintF: PrimeField>(
    n: FpVar<ConstraintF>,
    exp: FpVar<ConstraintF>,
    expected: FpVar<ConstraintF>,
) -> Result<(), SynthesisError> {
    let res = FpVar::<ConstraintF>::new_witness(n.cs().clone(), || {
        let n_val = n.value()?;
        let exp_val = exp.value()?;
        Ok(n_val.pow(exp_val.into_bigint()))
    })?;
    res.enforce_equal(&expected)?;
    Ok(())
}


impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF>
    for PrimeCircutNotFpVar<ConstraintF>
{
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let _ = self.miller_rabin_r1cs_using_circut_val(cs);
        Ok(())
    }
}
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
    a_vector: Vec<FpVar<ConstraintF>>,
    a_to_power_d_mod_n_vec: Vec<FpVar<ConstraintF>>,
    x_to_power_of_2_mod_n_vec: Vec<FpVar<ConstraintF>>,
    y_vec: Vec<FpVar<ConstraintF>>,
    is_prime: Boolean<ConstraintF>,
) -> Result<bool, SynthesisError> {
    // if n is even, return false
    let n_bigint = n.value()?.into_bigint();
    let is_even = n_bigint.is_even();

    let is_even_var = Boolean::new_witness(cs.clone(), || Ok(is_even))?;

    println!("is_even = {:?}", is_even);
    println!("is_even_var = {:?}", is_even_var.value()?);
    // enforce not equal to is_prime:
    is_prime.enforce_equal(&is_even_var.not())?;
    // // if n is even, return false:
    if is_even_var.value()? {
        return Ok(false);
    }

    // Now n is odd, we can write n-1 = 2^s * d
    let n_minos_one = n.clone() - ConstraintF::one();
    // enforce that n-1 = 2^s * d
    n_minos_one.enforce_equal(&(&two_to_s * &d))?;

    //
    // s is the number of times n-1 is divisible by 2 - inner loop
    let s_value = s.value()?.to_string().parse::<u64>().unwrap();

    println!("s_value = {:?}", s_value);
    // now we need to check if n is prime
    // we need to check if n is prime k times:
    for i in 0..k {
        // choose a random number a in the range [2, n-1]
        let x = a_to_power_d_mod_n_vec
            .get(i)
            .ok_or(SynthesisError::AssignmentMissing)?;
        println!("x = {:?}", x.value()?);
        // Now we need to call pow function to calculate x^d mod n
        pow(x.clone(), d.clone(), a_vector[i].clone())?;
        for j in 0..s_value {
            // enforce that y = x^2 mod n
            let y = y_vec
                .get(j as usize)
                .ok_or(SynthesisError::AssignmentMissing)?;
            let x_j_to_power_of_2_mod_n = x_to_power_of_2_mod_n_vec
                .get(j as usize)
                .ok_or(SynthesisError::AssignmentMissing)?;
            println!("y = {:?}", y.value()?);
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
            // condition == true if y = 1 and x != 1 and x != n-1 meaning its  : BUG HERE
            println!("condition = {:?}", condition.value()?);
            is_prime.not().enforce_equal(&condition)?;
            // if y != 1, return false
            //
            println!("is_prime = {:?}", is_prime.value()?);
            println!("Y_is_one.not = {:?}", y_is_one.not().value()?);
            is_prime.not().enforce_equal(&y_is_one.not())?;
            if condition.value()? {
                return Ok(false);
            }
            // now x = y
            x.enforce_equal(&y)?;
        }
    }
    // // if we didn't find a prime, return false
    is_prime.enforce_equal(&Boolean::constant(true))?;
    Ok(true)
}

pub fn miller_rabin_witness_creation_as_fr<ConstraintF: PrimeField>(
    n: BigUint,
    k: usize,
    cs: ConstraintSystemRef<ConstraintF>,
    circut: &mut PrimeCircutNotFpVar<ConstraintF>,
) -> Result<(), SynthesisError> {
    let two: BigUint = 2.to_biguint().unwrap();
    if n.eq(&two) {
        circut.is_prime = true;
        return Ok(());
    }
    if n.is_even() {
        circut.is_prime = false;
        return Ok(());
    }

    // Convert n to a BigUint
    let n_bigint = n.to_biguint().unwrap();
    let one: BigUint = 1.to_biguint().unwrap();
    let n_minus_one = n_bigint.clone() - one.clone();

    let mut s = 0;
    let mut d = n_bigint.clone() - one.clone();

    // n-1 = 2^s * d
    while d.is_even() {
        d = d.div(2.to_biguint().unwrap());
        s += 1;
    }
    // Convert to ConstraintF
    let d_fr = ConstraintF::from_le_bytes_mod_order(&d.to_bytes_le());
    let s_fr = ConstraintF::from(s);
    let two_to_s = two.pow(s).to_biguint().unwrap();
    let two_to_s_fr = ConstraintF::from_le_bytes_mod_order(&two_to_s.to_bytes_le());

    // Initialize vectors
    let mut a_to_power_d_mod_n_vec = Vec::<ConstraintF>::new();
    let mut x_to_power_of_2_mod_n_vec = Vec::<ConstraintF>::new();
    let mut y_vec = Vec::<ConstraintF>::new();
    let mut a_vec = Vec::<ConstraintF>::new();

    let mut y = one.clone();

    for _i in 0..k {
        let a = rand::thread_rng().gen_biguint_range(&two, &(n_bigint.clone() - &two));
        // add a to the a_vec
        let a_fr = ConstraintF::from_le_bytes_mod_order(&a.to_bytes_le());
        a_vec.push(a_fr);

        // let a_to_d = a.pow(d);
        // a^d mod n : Separate to a^d and a^d mod n:
        let a_to_pow_d_modn = a.modpow(&d, &n_bigint); // r
        // qy+r = x  --> r = a_to_pow_d_modn , y= n , x = ? , q = ?
        
        let a_fr = ConstraintF::from_le_bytes_mod_order(&a_to_pow_d_modn.to_bytes_le());
        a_to_power_d_mod_n_vec.push(a_fr);
        let mut x = a_to_pow_d_modn.clone();
        for _j in 0..s {
            y = x.modpow(&two, &n_bigint);

            let x_to_power_of_2_mod_n = x.modpow(&two, &n_bigint);
            let x_fr = ConstraintF::from_le_bytes_mod_order(&x_to_power_of_2_mod_n.to_bytes_le());
            x_to_power_of_2_mod_n_vec.push(x_fr);

            let y_fr = ConstraintF::from_le_bytes_mod_order(&y.to_bytes_le());
            y_vec.push(y_fr);

            if one == y && x != one && x != n_minus_one {
                circut.is_prime = false;
                return Ok(());
            }
            x = y.clone();
        }
    }
    // Update the circuit
    circut.d = d_fr;
    circut.s = s_fr;
    circut.two_to_s = two_to_s_fr;
    circut.a_to_power_d_mod_n_vec = a_to_power_d_mod_n_vec;
    circut.x_to_power_of_2_mod_n_vec = x_to_power_of_2_mod_n_vec;
    circut.y_vec = y_vec;
    circut.is_prime = true;
    Ok(())
}

// create tests for the miller_rabin_r1cs function
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::Fr;
    use ark_r1cs_std::{alloc::AllocVar, boolean::Boolean, fields::fp::FpVar};
    use ark_relations::r1cs::ConstraintLayer;
    use ark_relations::r1cs::{ConstraintSystem, ConstraintSystemRef};
    use ark_std::test_rng;
    use ark_std::Zero;
    use core::num;
    #[test]
    fn pow_tests() -> Result<(), SynthesisError> {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let n = FpVar::<Fr>::new_witness(cs.clone(), || Ok(Fr::from(2u64)))?;
        let exp = FpVar::<Fr>::new_witness(cs.clone(), || Ok(Fr::from(4u64)))?;
        let expected = FpVar::<Fr>::new_witness(cs.clone(), || Ok(Fr::from(16u64)))?;
        pow(n, exp, expected).unwrap();
        assert!(cs.is_satisfied().unwrap());
        Ok(())
    }
    #[test]
    fn modolo() -> Result<(), SynthesisError> {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let x = FpVar::<Fr>::new_witness(cs.clone(), || Ok(Fr::from(10u64)))?;
        let q = FpVar::<Fr>::new_witness(cs.clone(), || Ok(Fr::from(2u64)))?;
        let r = FpVar::<Fr>::new_witness(cs.clone(), || Ok(Fr::from(0u64)))?;
        let y = FpVar::<Fr>::new_witness(cs.clone(), || Ok(Fr::from(5u64)))?;
        // qy+r = x
        modulo(x, q, r, y).unwrap();
        assert!(cs.is_satisfied().unwrap());
        Ok(())
    }
}

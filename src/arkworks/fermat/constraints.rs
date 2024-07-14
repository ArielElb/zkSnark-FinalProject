use crate::arkworks::fermat::modpow_circut::{mod_witnesses, modpow_ver_circuit};
use modulo::{mod_pow_generate_witnesses, mod_vals, return_struct};

use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::sponge::DuplexSpongeMode;
use ark_ff::{Field, PrimeField};
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::select::CondSelectGadget;
use ark_r1cs_std::R1CSVar;
use ark_r1cs_std::ToBitsGadget;
use ark_r1cs_std::{alloc::AllocVar, fields::FieldVar};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use num_bigint::{BigUint, ToBigInt, ToBigUint};
use std::ops::AddAssign;
use std::{char::from_u32, ops::MulAssign};

use rand::SeedableRng;
const NUM_BITS: usize = 381;

use super::modpow_circut::structInitializer;
use super::modulo;

// struct for fermat circuit:
#[derive(Clone)]
pub struct fermat_circuit<ConstraintF: PrimeField> {
    pub n: ConstraintF,
    pub a: ConstraintF,  // randomness
    result: ConstraintF, // result of the modpow
    pub is_prime: bool,  // witness if the number is prime
    modpow_ver_circuit: modpow_ver_circuit<ConstraintF>,
}

fn check_bits_is_exp<ConstraintF: PrimeField>(
    cs: ConstraintSystemRef<ConstraintF>,
    bits: Vec<ConstraintF>,
    exp: FpVar<ConstraintF>,
) {
    let mut res =
        FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(ConstraintF::zero())).unwrap();
    let two = FpVar::<ConstraintF>::constant(ConstraintF::one() + ConstraintF::one());
    let mut cur_pow = FpVar::<ConstraintF>::constant(ConstraintF::one());
    for i in 0..NUM_BITS {
        res.add_assign(&cur_pow * bits[i]);
        cur_pow.mul_assign(&two);
    }
    //println!("{:?}",res.value().unwrap());
    //println!("{:?}",exp.value().unwrap());
    res.enforce_equal(&exp).unwrap();
}
// function that get modpow_ver_circuit and create the constraints for  modpow
fn modpow<ConstraintF: PrimeField>(
    cs: ConstraintSystemRef<ConstraintF>,
    modpow_ver_circuit: &modpow_ver_circuit<ConstraintF>,
    base: FpVar<ConstraintF>,
    divisor: FpVar<ConstraintF>,
    exp: FpVar<ConstraintF>,
) -> Result<(), SynthesisError> {
    let mut cur_pow = base.clone();
    let bits = &modpow_ver_circuit.bits;
    let result: FpVar<ConstraintF> =
        FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(modpow_ver_circuit.result))?;
    let one = &base * &base.inverse().unwrap();
    let mut calculated_res = one.clone();
    check_bits_is_exp(cs.clone(), bits.clone(), exp);
    for i in 0..NUM_BITS {
        let elem_val = &bits[i];
        let elem = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(elem_val))?;
        calculated_res.mul_assign(elem * (&cur_pow - &one) + &one);

        let cur_q: FpVar<ConstraintF> = FpVar::<ConstraintF>::new_witness(cs.clone(), || {
            Ok(modpow_ver_circuit.modulo_witnesses[i].q)
        })?;
        let cur_remainder: FpVar<ConstraintF> =
            FpVar::<ConstraintF>::new_witness(cs.clone(), || {
                Ok(modpow_ver_circuit.modulo_witnesses[i].remainder)
            })?;
        let result_of_vars = cur_q * &divisor + &cur_remainder;

        result_of_vars.enforce_equal(&calculated_res)?;
        let cmp_res = cur_remainder.is_cmp_unchecked(&divisor, std::cmp::Ordering::Less, false)?;

        calculated_res = cur_remainder;

        cur_pow.mul_assign(cur_pow.clone());
        let cur_q: FpVar<ConstraintF> = FpVar::<ConstraintF>::new_witness(cs.clone(), || {
            Ok(modpow_ver_circuit.modulo_of_pow_witnesses[i].q)
        })?;
        let cur_remainder: FpVar<ConstraintF> =
            FpVar::<ConstraintF>::new_witness(cs.clone(), || {
                Ok(modpow_ver_circuit.modulo_of_pow_witnesses[i].remainder)
            })?;
        let result_of_vars = cur_q * &divisor + &cur_remainder;
        result_of_vars.enforce_equal(&cur_pow)?;
        let cmp_res = cur_remainder.is_cmp_unchecked(&divisor, std::cmp::Ordering::Less, false)?;

        cur_pow = cur_remainder;
    }
    calculated_res.enforce_equal(&result)?;

    Ok(())
}

// implement the constraints for the fermat circuit:
impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for fermat_circuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // let rng = rand::SeedableRng::from_seed(self.a.to_bytes());
        let n = FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(self.n))?;
        let a = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.a))?;
        let result = FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(self.result))?;
        let is_prime = Boolean::<ConstraintF>::new_witness(cs.clone(), || Ok(self.is_prime))?;
        let other_res =
            FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.modpow_ver_circuit.result))?;
        let modpow_ver_circuit = self.modpow_ver_circuit;
        result.enforce_equal(&other_res)?;
        let n_minus_one = n.clone() - FpVar::<ConstraintF>::constant(ConstraintF::one());
        let _ = modpow(cs.clone(), &modpow_ver_circuit, a, n, n_minus_one)?;
        let one = FpVar::<ConstraintF>::constant(ConstraintF::one());
        result
            .is_eq(&one)?
            .conditional_enforce_equal(&is_prime, &Boolean::constant(true))?; // result == 1 => is_prime
        Ok(())
    }
}
fn Fermat_test(a: BigUint, p: BigUint) -> bool {
    let one_val = BigUint::from(1u32);
    if a.modpow(&(&p - &one_val), &p) == one_val {
        return true;
    }
    return false;
}

pub fn fermat_constructor<ConstraintF: PrimeField>(
    a: BigUint,
    n: BigUint,
) -> fermat_circuit<ConstraintF> {
    let modpow_circuit = structInitializer::<ConstraintF>(a.clone(), n.clone() - 1u32, n.clone());
    return fermat_circuit {
        n: ConstraintF::from(n.clone()),
        a: ConstraintF::from(a.clone()),
        is_prime: Fermat_test(a, n),
        result: modpow_circuit.result.clone(),
        modpow_ver_circuit: modpow_circuit,
    };
}

// add tests :
#[cfg(test)]
mod tests {
    use super::*;
    use crate::arkworks::fermat::modpow_circut::bits_vector_convertor;
    use crate::arkworks::fermat::modpow_circut::vector_convertor;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_relations::r1cs::ConstraintSystemRef;
    use ark_relations::r1cs::SynthesisError;
    use ark_std::test_rng;
    use ark_std::UniformRand;

    #[test]
    fn test_fermat_circuit() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let rng = &mut test_rng();

        // create the witnesses for the modpow circuit:
        let base_val = BigUint::from(7u32);
        let exp = BigUint::from(16u32); // number is 17 to check
        let modulus = BigUint::from(17u32);

        let res = base_val.modpow(&exp, &modulus);
        println!("res is: {}", res);
        let returnted_val =
            mod_pow_generate_witnesses(base_val.clone(), modulus.clone(), exp.clone());
        let base = Fr::from(base_val);
        let exponent = Fr::from(exp);
        let result = Fr::from(res);
        let divisor = Fr::from(modulus);
        let mod_wits = returnted_val.mod_vals;
        let mod_pow_wits = returnted_val.mod_pow_vals;
        let circuit = modpow_ver_circuit {
            base,
            exponent,
            result,
            divisor,
            modulo_witnesses: vector_convertor::<Fr>(mod_wits),
            modulo_of_pow_witnesses: vector_convertor::<Fr>(mod_pow_wits),
            bits: bits_vector_convertor::<Fr>(returnted_val.bits),
        };

        let n = Fr::from(17u32); //
        let a = Fr::from(7u32);
        let result = Fr::from(1u32);
        let is_prime = true;

        let fermat_circuit = fermat_circuit {
            n,
            a,
            result,
            is_prime,
            modpow_ver_circuit: circuit,
        };

        assert!(fermat_circuit
            .clone()
            .generate_constraints(cs.clone())
            .is_ok());
        assert!(cs.is_satisfied().unwrap());
    }
}

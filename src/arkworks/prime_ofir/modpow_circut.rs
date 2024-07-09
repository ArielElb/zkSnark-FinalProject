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
use sha2::digest::consts::True;
use std::str::FromStr;
use std::{char::from_u32, ops::MulAssign};

use super::modulo::mod_vals;
const NUM_BITS: usize = 381;

#[derive(Clone)]
pub struct mod_witnesses<ConstraintF: PrimeField> {
    n: ConstraintF,
    //div: FpVar<ConstraintF>,
    q: ConstraintF,
    remainder: ConstraintF,
}
pub struct modpow_ver_circuit<ConstraintF: PrimeField> {
    base: ConstraintF,
    exponent: ConstraintF,
    result: ConstraintF,
    divisor: ConstraintF,
    modulo_witnesses: Vec<mod_witnesses<ConstraintF>>,
    modulo_of_pow_witnesses: Vec<mod_witnesses<ConstraintF>>,
    bits: Vec<ConstraintF>,
}
fn modVals_to_modWitness<ConstraintF: PrimeField>(modVal: mod_vals) -> mod_witnesses<ConstraintF> {
    let witness = mod_witnesses {
        n: ConstraintF::from(modVal.num),
        q: ConstraintF::from(modVal.q),
        remainder: ConstraintF::from(modVal.remainder),
    };
    return witness;
}
fn vector_convertor<ConstraintF: PrimeField>(
    mod_vals: Vec<mod_vals>,
) -> Vec<mod_witnesses<ConstraintF>> {
    let vec_wits: Vec<mod_witnesses<ConstraintF>> = mod_vals
        .iter()
        .map(|elem| modVals_to_modWitness(elem.clone()))
        .collect();
    return vec_wits;
}
fn bits_vector_convertor<ConstraintF: PrimeField>(bit_vec: Vec<u8>) -> Vec<ConstraintF> {
    let vec_wits: Vec<ConstraintF> = bit_vec
        .iter()
        .map(|elem| ConstraintF::from(elem.to_biguint().unwrap()))
        .collect();
    return vec_wits;
}
impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF>
    for modpow_ver_circuit<ConstraintF>
{
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let base: FpVar<ConstraintF> =
            FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(self.base))?;
        let mut cur_pow = base.clone();
        let exp: FpVar<ConstraintF> =
            FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(self.exponent))?;
        let divisor: FpVar<ConstraintF> =
            FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(self.divisor))?;
        //let mut exp_val = exp.clone();
        let bits = self.bits;
        let result: FpVar<ConstraintF> =
            FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.result))?;
        let one = &base * &base.inverse().unwrap();
        let mut calculated_res = one.clone(); //= FpVar::new_constant(cs, 1);
        let mod_witnesses = self.modulo_witnesses;
        let mod_of_pow_witnesses = self.modulo_of_pow_witnesses;
        for i in 0..NUM_BITS {
            let elem_val = &bits[i];
            let elem = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(elem_val))?;
            calculated_res.mul_assign(elem * (&cur_pow - &one) + &one);

            //checks the correctness of mod
            //let current_witness = FpVar::<ConstraintF>::new_variable(&cs, || );
            let constraintF_witness = mod_witnesses[i].clone();
            let cur_q: FpVar<ConstraintF> =
                FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(constraintF_witness.q))?;
            let cur_remainder: FpVar<ConstraintF> = FpVar::<ConstraintF>::new_witness(
                cs.clone(),
                || Ok(constraintF_witness.remainder),
            )?;
            let result_of_vars = cur_q * &divisor + &cur_remainder;
            result_of_vars.enforce_equal(&calculated_res)?;
            let cmp_res =
                cur_remainder.is_cmp_unchecked(&divisor, std::cmp::Ordering::Less, false)?;

            calculated_res = cur_remainder;

            cur_pow.mul_assign(cur_pow.clone());
            //checks the correctness of mod
            let current_witness = mod_of_pow_witnesses[i].clone();
            let cur_q: FpVar<ConstraintF> =
                FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(constraintF_witness.q))?;
            let cur_remainder: FpVar<ConstraintF> = FpVar::<ConstraintF>::new_witness(
                cs.clone(),
                || Ok(constraintF_witness.remainder),
            )?;
            let result_of_vars = cur_q * &divisor + &cur_remainder;
            result_of_vars.enforce_equal(&calculated_res)?;
            let cmp_res =
                cur_remainder.is_cmp_unchecked(&divisor, std::cmp::Ordering::Less, false)?;

            cur_pow = cur_remainder;
        }
        calculated_res.enforce_equal(&result)?;

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use crate::arkworks::prime_ofir::modulo;

    use super::*;
    use ark_bls12_381::Fr;
    use ark_ff::fields::PrimeField;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_std::{One, Zero};
    use modulo::{mod_pow_generate_witnesses, mod_vals, return_struct};
    use num_bigint::BigUint;
    use rand::{thread_rng, Rng};
    /// Generates a random field element
    fn random_fe<R: rand::Rng>(rng: &mut R) -> Fr {
        Fr::from(rng.gen::<u64>())
    }
    fn generate_random_biguint(num_bytes: usize) -> BigUint {
        let mut rng = thread_rng();
        let mut bytes = vec![0u8; num_bytes];
        rng.fill(&mut bytes[..]); // Fill the vector with random bytes
        BigUint::from_bytes_le(&bytes) // Convert bytes to a BigUint
    }
    #[test]
    fn test_modpow_circuit_correct() {
        let base_val = generate_random_biguint(47);
        let exp = generate_random_biguint(47);
        let modulus = generate_random_biguint(47);

        let res = base_val.modpow(&exp, &modulus);
        let returnted_val =
            mod_pow_generate_witnesses(base_val.clone(), modulus.clone(), exp.clone());
        let base = Fr::from(base_val);
        let mut rng = thread_rng();
        let exponent = Fr::from(exp);
        let result = Fr::from(res);
        let divisor = Fr::from(modulus);
        let cs = ConstraintSystem::<Fr>::new_ref();
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

        assert!(circuit.generate_constraints(cs.clone()).is_ok());
        assert!(cs.is_satisfied().unwrap());
    }
}
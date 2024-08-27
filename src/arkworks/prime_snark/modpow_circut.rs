use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::{Field, PrimeField};
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

use super::utils::constants;
use super::utils::modulo::mod_pow_generate_witnesses;
use super::utils::modulo::ModVals;
use super::utils::modulo::ReturnStruct;
const NUM_BITS: usize = constants::NUM_BITS;
#[derive(Clone)]
pub struct ModWitnesses<ConstraintF: PrimeField> {
    pub n: ConstraintF,
    pub q: ConstraintF,
    pub remainder: ConstraintF,
}
#[derive(Clone)]
pub struct ModpowVerCircuit<ConstraintF: PrimeField> {
    pub base: ConstraintF,
    pub exponent: ConstraintF,
    pub result: ConstraintF,
    pub divisor: ConstraintF,
    pub modulo_witnesses: Vec<ModWitnesses<ConstraintF>>,
    pub modulo_of_pow_witnesses: Vec<ModWitnesses<ConstraintF>>,
    pub bits: Vec<ConstraintF>,
}
pub fn mod_vals_to_mod_witness<ConstraintF: PrimeField>(
    mod_val: ModVals,
) -> ModWitnesses<ConstraintF> {
    let witness = ModWitnesses {
        n: ConstraintF::from(mod_val.num),
        q: ConstraintF::from(mod_val.q),
        remainder: ConstraintF::from(mod_val.remainder),
    };
    return witness;
}
pub fn vector_convertor<ConstraintF: PrimeField>(
    mod_vals: Vec<ModVals>,
) -> Vec<ModWitnesses<ConstraintF>> {
    let vec_wits: Vec<ModWitnesses<ConstraintF>> = mod_vals
        .iter()
        .map(|elem| mod_vals_to_mod_witness(elem.clone()))
        .collect();
    return vec_wits;
}
pub fn bits_vector_convertor<ConstraintF: PrimeField>(bit_vec: Vec<u8>) -> Vec<ConstraintF> {
    let vec_wits: Vec<ConstraintF> = bit_vec
        .iter()
        .map(|elem| ConstraintF::from(elem.to_biguint().unwrap()))
        .collect();
    return vec_wits;
}
// fn mod_pow_constraints
impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for ModpowVerCircuit<ConstraintF> {
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
        let bits = self.bits;
        let result: FpVar<ConstraintF> =
            FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.result))?;
        let one = &base * &base.inverse().unwrap();
        let mut calculated_res = one.clone();
        for i in 0..NUM_BITS {
            let elem_val = &bits[i];
            let elem = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(elem_val))?;
            calculated_res.mul_assign(elem * (&cur_pow - &one) + &one);

            let cur_q: FpVar<ConstraintF> =
                FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.modulo_witnesses[i].q))?;
            let cur_remainder: FpVar<ConstraintF> =
                FpVar::<ConstraintF>::new_witness(cs.clone(), || {
                    Ok(self.modulo_witnesses[i].remainder)
                })?;
            let result_of_vars = cur_q * &divisor + &cur_remainder;

            result_of_vars.enforce_equal(&calculated_res)?;
            let cmp_res =
                cur_remainder.is_cmp_unchecked(&divisor, std::cmp::Ordering::Less, false)?;

            calculated_res = cur_remainder;

            cur_pow.mul_assign(cur_pow.clone());
            let cur_q: FpVar<ConstraintF> = FpVar::<ConstraintF>::new_witness(cs.clone(), || {
                Ok(self.modulo_of_pow_witnesses[i].q)
            })?;
            let cur_remainder: FpVar<ConstraintF> =
                FpVar::<ConstraintF>::new_witness(cs.clone(), || {
                    Ok(self.modulo_of_pow_witnesses[i].remainder)
                })?;
            let result_of_vars = cur_q * &divisor + &cur_remainder;
            result_of_vars.enforce_equal(&cur_pow)?;
            let cmp_res =
                cur_remainder.is_cmp_unchecked(&divisor, std::cmp::Ordering::Less, false)?;

            cur_pow = cur_remainder;
        }
        calculated_res.enforce_equal(&result)?;

        Ok(())
    }
}
pub fn struct_initializer<ConstraintF: PrimeField>(
    base: BigUint,
    exp: BigUint,
    modulo: BigUint,
) -> ModpowVerCircuit<ConstraintF> {
    let res = base.modpow(&exp, &modulo);
    let returnted_val: ReturnStruct =
        mod_pow_generate_witnesses(base.clone(), modulo.clone(), exp.clone());
    let base = ConstraintF::from(base);
    let exponent = ConstraintF::from(exp);
    let result = ConstraintF::from(res);
    let divisor = ConstraintF::from(modulo);
    //let cs = ConstraintSystem::<Fr>::new_ref();
    let mod_wits = returnted_val.mod_vals;
    let mod_pow_wits = returnted_val.mod_pow_vals;
    let circuit = ModpowVerCircuit {
        base,
        exponent,
        result,
        divisor,
        modulo_witnesses: vector_convertor::<ConstraintF>(mod_wits),
        modulo_of_pow_witnesses: vector_convertor::<ConstraintF>(mod_pow_wits),
        bits: bits_vector_convertor::<ConstraintF>(returnted_val.bits),
    };
    return circuit;
}
#[cfg(test)]
mod tests {
    use super::super::utils::modulo;
    use super::*;
    use ark_bls12_381::Fr;
    use ark_ff::fields::PrimeField;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_std::{One, Zero};
    use modulo::{mod_pow_generate_witnesses, ModVals, ReturnStruct};
    use num_bigint::BigUint;
    use rand::{thread_rng, Rng};
    /// Generates a random field element
    pub fn random_fe<R: rand::Rng>(rng: &mut R) -> Fr {
        Fr::from(rng.gen::<u64>())
    }
    pub fn generate_random_biguint(num_bytes: usize) -> BigUint {
        let mut rng = thread_rng();
        let mut bytes = vec![0u8; num_bytes];
        rng.fill(&mut bytes[..]); // Fill the vector with random bytes
        BigUint::from_bytes_le(&bytes) // Convert bytes to a BigUint
    }
    #[test]
    fn test_modpow_circuit_correct() {
        let base_val = generate_random_biguint(37);
        let exp = generate_random_biguint(37);
        let modulus = generate_random_biguint(37);
        //let base_val = BigUint::from(5u64);
        //let exp = BigUint::from(3u64);
        //let modulus = BigUint::from(4u64);

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
        let circuit = ModpowVerCircuit {
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

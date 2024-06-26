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

#[derive(Clone)]
pub struct ModExpCircuit<F: PrimeField> {
    pub base: F,
    pub exp: F,
    pub modulus: F,
    pub result: F,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for ModExpCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let base_var = FpVar::new_input(cs.clone(), || Ok(self.base))?;
        let exp_var = FpVar::new_input(cs.clone(), || Ok(self.exp))?;
        let modulus_var = FpVar::new_input(cs.clone(), || Ok(self.modulus))?;
        let result_var = FpVar::new_input(cs, || Ok(self.result))?;

        // Initialize the accumulator
        let mut acc = base_var.clone();

        // Perform modular exponentiation
        for i in (0..F::MODULUS_BIT_SIZE).rev() {
            acc = acc.square()?;
            acc = modulo_reduce(&acc, &modulus_var)?;
            if exp_var.to_bits_le()?.get(i).value().unwrap_or(false) {
                acc = &acc * &base_var;
                acc = modulo_reduce(&acc, &modulus_var)?;
            }
        }

        // Enforce that the result matches the expected result
        acc.enforce_equal(&result_var)?;
        Ok(())
    }
}

/// Custom modular reduction function
fn modulo_reduce<F: PrimeField>(
    x: &FpVar<F>,
    modulus: &FpVar<F>,
) -> Result<FpVar<F>, SynthesisError> {
    let div_result = x.div_by(modulus)?;
    let quotient = div_result.floor()?.to_field_var()?;
    let prod = quotient * modulus;
    let remainder = x - prod;
    Ok(remainder)
}

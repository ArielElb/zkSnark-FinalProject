use std::{char::from_u32, ops::MulAssign};
use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::sponge::DuplexSpongeMode;
use ark_ff::{Field, PrimeField};
use ark_r1cs_std::{alloc::AllocVar, fields::FieldVar};
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use sha2::digest::consts::True;
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::select::CondSelectGadget;
use ark_r1cs_std::R1CSVar;
use ark_r1cs_std::{ToBitsGadget};
use num_bigint::BigUint;
use std::str::FromStr;
const NUM_BITS:usize = 381;

#[derive(Clone)]
pub struct mod_witnesses<ConstraintF:PrimeField>{
    n: FpVar<ConstraintF>,
    //div: FpVar<ConstraintF>,
    q: FpVar<ConstraintF>,
    remainder: FpVar<ConstraintF>
}

pub struct modpow_ver_circuit<ConstraintF: PrimeField> {
    base: ConstraintF,
    exponent: ConstraintF,
    result: ConstraintF,
    divisor: ConstraintF,
    modulo_witnesses: Vec<mod_witnesses<ConstraintF>>,
    modulo_of_pow_witnesses: Vec<mod_witnesses<ConstraintF>>,
    bits: Vec<FpVar<ConstraintF>>
}

impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF>
    for modpow_ver_circuit<ConstraintF>
{
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let base: FpVar<ConstraintF> = FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(self.base))?;
        let mut cur_pow = base.clone();
        let exp: FpVar<ConstraintF> = FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(self.exponent))?;
        let divisor: FpVar<ConstraintF> = FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(self.divisor))?;
        //let mut exp_val = exp.clone();
        let bits = self.bits;
        let result: FpVar<ConstraintF> = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.result))?;
        let one = &base * &base.inverse().unwrap();
        let mut calculated_res = one.clone();//= FpVar::new_constant(cs, 1);
        let mod_witnesses = self.modulo_witnesses;
        let mod_of_pow_witnesses = self.modulo_of_pow_witnesses;
        for i in 0..NUM_BITS{
            let elem = &bits[i];
            calculated_res.mul_assign(elem * (&cur_pow-&one) + &one);

            //checks the correctness of mod
            let current_witness = mod_witnesses[i].clone();
            let result_of_vars = current_witness.q*&divisor+&current_witness.remainder;
            result_of_vars.enforce_equal(&calculated_res)?;
            let cmp_res = current_witness.remainder.is_cmp_unchecked(&divisor, std::cmp::Ordering::Less , false)?;

            calculated_res = mod_witnesses[i].remainder.clone();
            cur_pow.mul_assign(cur_pow.clone());

            //checks the correctness of mod
            cur_pow = mod_of_pow_witnesses[i].remainder.clone();
            let current_witness = mod_of_pow_witnesses[i].clone();
            let result_of_vars = current_witness.q*&divisor+&current_witness.remainder;
            result_of_vars.enforce_equal(&calculated_res)?;
            let cmp_res = current_witness.remainder.is_cmp_unchecked(&divisor, std::cmp::Ordering::Less , false)?;
        }
        calculated_res.enforce_equal(&result)?;
        
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
    #[test]
    fn mod_circuit_test() {
        
    }
     
}
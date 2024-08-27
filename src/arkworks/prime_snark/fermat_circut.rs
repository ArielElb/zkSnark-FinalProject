use super::utils::constants;
use crate::arkworks::prime_snark::modpow_circut::{ModWitnesses, ModpowVerCircuit};
use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::{Field, PrimeField};
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::select::CondSelectGadget;
use ark_r1cs_std::R1CSVar;
use ark_r1cs_std::ToBitsGadget;
use ark_r1cs_std::{alloc::AllocVar, fields::FieldVar};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use num_bigint::RandBigInt;
use num_bigint::{BigUint, ToBigInt, ToBigUint};

use std::ops::{AddAssign, MulAssign};
const K: usize = constants::K;
const NUM_BITS: usize = constants::NUM_BITS;
use super::modpow_circut::struct_initializer;
use super::utils::hasher::generate_bases_a;
use super::utils::hasher::generate_bases_native;
use crate::arkworks::prime_snark::utils::modulo;

// struct for fermat circuit:
#[derive(Clone)]
pub struct FermatCircuit<ConstraintF: PrimeField> {
    pub n: ConstraintF, // the modulus and the number we want to check if it is prime
    pub a: ConstraintF, // randomness
    results: Vec<ConstraintF>, // result of the modpow
    pub is_prime: bool, // witness if the number is prime
    modpow_ver_circuits: Vec<ModpowVerCircuit<ConstraintF>>,
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
    modpow_ver_circuit: &ModpowVerCircuit<ConstraintF>,
    base: &FpVar<ConstraintF>,
    divisor: &FpVar<ConstraintF>,
    exp: FpVar<ConstraintF>,
) -> Result<(), SynthesisError> {
    let mut cur_pow = base.clone();
    let bits = &modpow_ver_circuit.bits;
    let result: FpVar<ConstraintF> =
        FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(modpow_ver_circuit.result))?;
    let one = base * &base.inverse().unwrap();
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
        let result_of_vars = cur_q * divisor + &cur_remainder;
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
        let result_of_vars = cur_q * divisor + &cur_remainder;
        result_of_vars.enforce_equal(&cur_pow)?;
        let cmp_res = cur_remainder.is_cmp_unchecked(&divisor, std::cmp::Ordering::Less, false)?;
        cur_pow = cur_remainder;
    }
    calculated_res.enforce_equal(&result)?;

    Ok(())
}

// implement the constraints for the fermat circuit:
impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for FermatCircuit<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let n = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.n))?;
        let a = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.a))?;
        let bases = generate_bases_a(cs.clone(), &a);
        let one = FpVar::<ConstraintF>::constant(ConstraintF::one());
        for i in 0..K {
            let result = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.results[i]))?;
            let is_prime = Boolean::<ConstraintF>::new_witness(cs.clone(), || Ok(self.is_prime))?;
            let other_res = FpVar::<ConstraintF>::new_witness(cs.clone(), || {
                Ok(self.modpow_ver_circuits[i].result)
            })?;
            let modpow_ver_circuit = self.modpow_ver_circuits[i].clone();
            result.enforce_equal(&other_res)?;
            let n_minus_one = n.clone() - FpVar::<ConstraintF>::constant(ConstraintF::one());
            let _ = modpow(cs.clone(), &modpow_ver_circuit, &bases[i], &n, n_minus_one)?;
            result
                .is_eq(&one)?
                .conditional_enforce_equal(&is_prime, &Boolean::constant(true))?;
        }
        // result == 1 => is_prime
        Ok(())
    }
}
fn fermat_test(a: &BigUint, p: &BigUint) -> bool {
    let one_val = BigUint::from(1u32);
    let bases = generate_bases_native(a);
    for i in 0..K {
        if bases[i].modpow(&(p - &one_val), p) == one_val {
            return true;
        }
    }
    return false;
}

pub fn fermat_constructor<ConstraintF: PrimeField>(
    a: BigUint,
    n: BigUint,
) -> FermatCircuit<ConstraintF> {
    let modpow_circuit = struct_initializer::<ConstraintF>(a.clone(), n.clone() - 1u32, n.clone());
    let mut circuits = vec![modpow_circuit; K];
    let mut results = vec![ConstraintF::from(0u8); K];
    // we need to make sure that each base is between 1 and n-1
    let bases = generate_bases_native(&a);
    
    for i in 0..K {
        circuits[i] =
            struct_initializer::<ConstraintF>(bases[i].clone(), n.clone() - 1u32, n.clone());
        results[i] = circuits[i].result.clone();
    }
    return FermatCircuit {
        is_prime: fermat_test(&a, &n),
        n: ConstraintF::from(n),
        a: ConstraintF::from(a),
        results: results,
        modpow_ver_circuits: circuits,
    };
}

// add tests :
#[cfg(test)]
mod tests {
    use super::*;
    use crate::arkworks::prime_snark::modpow_circut::bits_vector_convertor;
    use crate::arkworks::prime_snark::modpow_circut::vector_convertor;
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
        let base_val = BigUint::from(13123u32);
        //let exp = BigUint::from(1231231u32); // number is 17 to check
        let modulus = BigUint::from(1213231u32);
        let circ = fermat_constructor::<Fr>(base_val, modulus);
        let cs = ConstraintSystem::<Fr>::new_ref();
        assert!(circ.generate_constraints(cs.clone()).is_ok());
        assert!(cs.is_satisfied().unwrap());
    }
}

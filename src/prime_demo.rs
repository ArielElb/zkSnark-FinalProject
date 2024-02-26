

use ark_ff::{ Field, One, PrimeField, Zero};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::{fields::fp::FpVar, R1CSVar};
use ark_r1cs_std::bits::uint32::UInt32;

use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};


#[derive(Copy, Clone)]
struct PrimeCircut<F: Field> {
    n: Option<F>,
    d: Option<F>,
    s: u64,
    num_constraints: usize,
    num_variables: usize,
}

// GENERATE CONSTRAINTS

impl<F: PrimeField> ConstraintSynthesizer<F> for PrimeCircut<F> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<F>,
    ) -> Result<(), SynthesisError> {
        // we need 2^s = d
        let mut d = FpVar::<F>::new_input(cs.clone(), || self.d.ok_or(SynthesisError::AssignmentMissing))?;
        let mut two = FpVar::<F>::new_input(cs.clone(), || Ok(F::from(2u64)))?;
        let s = UInt32::new_input(cs.clone(), || Ok(self.s as u32))?;
        let mut curr_var: FpVar<F> = FpVar::<F>::new_variable(cs.clone(), || Ok(F::one()), ark_r1cs_std::alloc::AllocationMode::Constant)?;
        let n = FpVar::<F>::new_input(cs.clone(), || self.n.ok_or(SynthesisError::AssignmentMissing))?;

        print!("curr: {:?}\n", curr_var.value().unwrap().into_bigint());
        print!("twoBig: {:?}\n", two.value().unwrap().into_bigint());
        print!("s: {:?}\n", self.s);
        print!("d: {:?}\n", d.value().unwrap().into_bigint());

        // curr_var = curr_var ^ s  : pow
        print!("curr_var: {:?}\n", curr_var.value().unwrap().into_bigint());
        for i in 0..self.s {
            curr_var = curr_var.clone() * two.clone();
        }
        print!("curr_var after\n: {:?}", curr_var.value().unwrap().into_bigint());

        curr_var = curr_var * d;

        print!("curr_var after d\n: {:?}", curr_var.value().unwrap().into_bigint());


        // enforce equal to n

        print!("n end: {:?}\n", n.value().unwrap().into_bigint());

        n.enforce_equal(&curr_var)?;

        Ok(())

    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Bls12_381, Fr as BlsFr};
    use ark_poly::univariate::DensePolynomial;
    use ark_poly_commit::marlin_pc::MarlinKZG10;
    use ark_std::{ops::*, UniformRand};
    use ark_relations::r1cs::ConstraintSynthesizer;
    use ark_relations::r1cs::ConstraintSystem;

    #[test]
    fn test_prime_native() {

        let cs = ConstraintSystem::<BlsFr>::new_ref();
        let n = BlsFr::from(12u8);
        let d = BlsFr::from(3u8);
        let s = 2u64;
        let circuit = PrimeCircut { n: Some(n), d: Some(d), s, num_constraints: 10, num_variables: 10 };

        assert!(circuit.generate_constraints(cs.clone()).is_ok());}
}


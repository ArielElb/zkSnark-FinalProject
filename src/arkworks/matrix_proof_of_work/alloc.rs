use ark_ff::PrimeField;
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::{
    fields::fp::FpVar,
    prelude::{AllocVar, AllocationMode, Boolean, EqGadget},
    uint8::UInt8,
    R1CSVar, ToBitsGadget,
};
use ark_relations::r1cs::{Namespace, SynthesisError};
use std::borrow::Borrow;

pub struct FpVarArray<const N: usize, F: PrimeField>(pub [FpVar<F>; N]);
pub struct FpVar2D<const N: usize, F: PrimeField>(pub [[FpVar<F>; N]; N]);

// allocates memory for Fpvar1D in our constrains system
impl<const N: usize, F: PrimeField> AllocVar<[u64; N], F> for FpVarArray<N, F> {
    fn new_variable<T: Borrow<[u64; N]>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        let cs = cs.into();
        // create array of FpVar:
        let mut array =
            [(); N].map(|_| FpVar::new_variable(cs.clone(), || Ok(F::zero()), mode).unwrap());

        let value = f().map_or([0; N], |f| *f.borrow());
        for (i, v) in value.into_iter().enumerate() {
            array[i] = FpVar::new_variable(cs.clone(), || Ok(F::from(v)), mode)?;
        }
        let contraint_array = FpVarArray(array);
        Ok(contraint_array)
    }
}

// allocates memory for Fpvar2D in our constrains system:
impl<const N: usize, F: PrimeField> AllocVar<[[u64; N]; N], F> for FpVar2D<N, F> {
    fn new_variable<T: Borrow<[[u64; N]; N]>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        let cs = cs.into();
        let row = [(); N].map(|_| FpVar::new_variable(cs.clone(), || Ok(F::zero()), mode).unwrap());
        let mut contraint_array = FpVar2D([(); N].map(|_| row.clone()));
        let value = f().map_or([[0; N]; N], |f| *f.borrow());
        for (i, row) in value.into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                contraint_array.0[i][j] =
                    FpVar::new_variable(cs.clone(), || Ok(F::from(cell)), mode)?;
            }
        }
        Ok(contraint_array)
    }
}

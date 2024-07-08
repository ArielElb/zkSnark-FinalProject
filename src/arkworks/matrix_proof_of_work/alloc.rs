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
#[derive(Clone)]
pub struct FpVarVec<F: PrimeField>(pub Vec<FpVar<F>>);
#[derive(Clone)]
pub struct FpVar2DVec<F: PrimeField>(pub Vec<Vec<FpVar<F>>>);

// allocates memory for Fpvar1D in our constrains system
impl<F: PrimeField> AllocVar<Vec<u64>, F> for FpVarVec<F> {
    fn new_variable<T: Borrow<Vec<u64>>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        let cs = cs.into();
        let value = f().map_or(Vec::new(), |f| f.borrow().clone());
        let mut vec = Vec::with_capacity(value.len());
        for &v in &value {
            vec.push(FpVar::new_variable(cs.clone(), || Ok(F::from(v)), mode)?);
        }
        Ok(FpVarVec(vec))
    }
}

// allocates memory for Fpvar2D in our constrains system:
impl<F: PrimeField> AllocVar<Vec<Vec<u64>>, F> for FpVar2DVec<F> {
    fn new_variable<T: Borrow<Vec<Vec<u64>>>>(
        cs: impl Into<Namespace<F>>,
        f: impl FnOnce() -> Result<T, SynthesisError>,
        mode: AllocationMode,
    ) -> Result<Self, SynthesisError> {
        let cs = cs.into();
        let value = f().map_or(Vec::new(), |f| f.borrow().clone());
        let mut vec2d = Vec::with_capacity(value.len());
        for row in value {
            let mut vec = Vec::with_capacity(row.len());
            for cell in row {
                vec.push(FpVar::new_variable(cs.clone(), || Ok(F::from(cell)), mode)?);
            }
            vec2d.push(vec);
        }
        Ok(FpVar2DVec(vec2d))
    }
}

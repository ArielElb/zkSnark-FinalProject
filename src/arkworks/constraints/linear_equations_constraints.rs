use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::{Field, PrimeField};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
// import zero
use ark_ff::Zero;

use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_snark::{CircuitSpecificSetupSNARK, SNARK};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct LinearEquationCircuit<ConstraintF: PrimeField> {
    pub a: Vec<Vec<ConstraintF>>, // Matrix A
    pub b: Vec<ConstraintF>,      // Vector b
    pub x: Vec<ConstraintF>,      // Solution vector x
    pub len_b: usize,             // Length of the vector b
    pub len_a: usize,             // Length of the matrix A
}
// output struct for the web server
#[derive(Serialize)]
pub struct OutputData {
    pub proof: String,
    pub public_input: Vec<String>,
    pub num_constraints: usize,
    pub num_variables: usize,
    pub proving_time: f64,
    pub verifying_time: f64,
}
// input struct for the web server : the matrix A, the vector b.
#[derive(Deserialize)]
pub struct InputData {
    pub a: Vec<Vec<u64>>,
    pub b: Vec<u64>,
}

// Implement ConstraintSynthesizer trait for LinearEquationCircuit
impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF>
    for LinearEquationCircuit<ConstraintF>
{
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // Implement constraints to enforce the properties of the linear equations Ax = b

        // Example:
        // Ensure that Ax = b
        for i in 0..self.len_b {
            let mut sum =
                FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(ConstraintF::zero()))?;
            for j in 0..self.len_a {
                let a_var = FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(self.a[i][j]))?;
                let x_var = FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(self.x[j]))?;
                let product = a_var * &x_var;
                sum += &product;
            }
            let b_var = FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(self.b[i]))?;
            sum.enforce_equal(&b_var)?;
        }

        Ok(())
    }
}

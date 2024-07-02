use std::any::type_name;
use ark_r1cs_std::{
    prelude::{AllocVar, EqGadget, R1CSVar},
};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError, ConstraintSynthesizer};
use ark_ff::PrimeField;
use ark_groth16::Groth16;
use ark_bls12_381::{Bls12_381, Fr};
use rand::rngs::StdRng;
use rand::SeedableRng;
use ark_r1cs_std::fields::fp::FpVar;
use std::time::Instant;
use std::mem;
use std::env;
use ark_snark::SNARK;
use ark_std::str::FromStr;
use std::sync::Mutex;
use lazy_static::lazy_static;
//static mut GLOBAL_STRING: &str = "Your global string here";
lazy_static! {
    static ref GLOBAL_STRING: Mutex<String> = Mutex::new(String::new());
}
//static mut GLOBAL_VARIABLE:Option<Fr>  = Option::from(Fr::from_str("2").unwrap());
#[derive(Clone)]
struct FibonacciCircuit<F: PrimeField> {
    pub a: Option<F>,
    pub b: Option<F>,
    pub numb_of_steps: usize,
    pub result: Option<F>,
}
fn fibonacci_steps(a: u64, b: u64, steps: u32) -> u64 {
    let mut x = a;
    let mut y = b;

    for _ in 0..steps {
        let next = x + y;
        x = y;
        y = next;
    }

    x
}
impl<F: PrimeField> ConstraintSynthesizer<F> for FibonacciCircuit<F> {
    fn generate_constraints(mut self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let mut fi_minus_one =
            FpVar::<F>::new_input(cs.clone(), || self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let mut fi_minus_two =
            FpVar::<F>::new_input(cs.clone(), || self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let saved_result = FpVar::<F>::new_witness(cs.clone(), || self.result.ok_or(SynthesisError::AssignmentMissing))?;

        // initialize fi as public input
        let mut fi = FpVar::<F>::new_witness(cs.clone(), || Ok(F::zero()))?;
        // do the loop only when verifying the circuit
        for _i in 0..self.num_of_steps-1 {
            fi = fi_minus_one.clone() + &fi_minus_two;
            fi.enforce_equal(&(&fi_minus_one + &fi_minus_two))?;
            fi_minus_two = fi_minus_one;
            fi_minus_one = fi.clone();
        }
        match fi.value() {
            Ok(val) => unsafe {
                // Do something with the value
                println!("Value of fi: {:?}", val.to_string());
                let val_str = val.to_string();
                let mut global_str = GLOBAL_STRING.lock().unwrap();
                *global_str = val_str;
            },
            Err(e) => {
                if e == SynthesisError::AssignmentMissing {
                    // Handle the AssignmentMissing error
                } else {
                    // Handle other types of errors
                }
            }
        }
       // println!("{}",saved_result.value().unwrap());
             fi.enforce_equal(&(&saved_result))?;

        Ok(())
    }
}


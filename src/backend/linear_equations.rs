use crate::constraints::linear_equations_constraints::{
    InputData, LinearEquationCircuit, OutputData,
};
use actix_web::{web, HttpResponse, Responder};
use ark_bls12_381::{Bls12_381, Fr};
use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSystem;
// groth trace:
use ark_relations::r1cs::ConstraintTrace;
use ark_relations::r1cs::TracingMode;
use ark_snark::SNARK;
use ark_std::perf_trace;
use ark_std::rand::SeedableRng;
use ark_std::start_timer;
use ark_std::Zero;
use rand::rngs::StdRng;
use std::time::Instant;

// create a function to solve the linear equations and find a witness x:
// Assume that the matrix A is a square matrix
fn solve_linear_equations(a: Vec<Vec<Fr>>, b: Vec<Fr>) -> Vec<Fr> {
    // create a vector to store the solution
    let mut x: Vec<Fr> = vec![Fr::zero(); b.len()];

    // iterate over the rows of the matrix A
    for i in 0..a.len() {
        // create a variable to store the sum of the products of the elements of the row and the solution vector x
        let mut sum = Fr::zero();

        // iterate over the elements of the row
        for j in 0..a[i].len() {
            // add the product of the element and the corresponding element of the solution vector x to the sum
            sum += a[i][j] * x[j];
        }

        // calculate the value of the solution vector x[i] using the corrected formula
        x[i] = (b[i] - sum) / a[i][i];
    }

    // return the solution vector x
    x
}

pub async fn prove_linear_equations(data: web::Json<InputData>) -> impl Responder {
    // enable tracing using the traceing mode.

    // Extract data from the request
    let a = data.a.clone();
    let b = data.b.clone();
    let len_b = b.len();
    let len_a = a[0].len();

    // Convert the input data to the required format
    let a: Vec<Vec<Fr>> = a
        .iter()
        .map(|row| row.iter().map(|x| Fr::from(*x)).collect())
        .collect();
    let b: Vec<Fr> = b.iter().map(|x| Fr::from(*x)).collect();

    // Solve the linear equations to find the solution vector x
    let x = solve_linear_equations(a.clone(), b.clone());
    // Create a constraint system
    let cs = ConstraintSystem::<Fr>::new_ref();

    // Obtain the public input:
    // [a[0][0], a[0][1], b[0], a[1][0], a[1][1], b[1]]
    let mut public_input: Vec<Fr> = vec![];
    for i in 0..a.clone().len() {
        for j in 0..a[0].len() {
            public_input.push(a[i][j]);
        }
        public_input.push(b[i]);
    }
    // Create a linear equation circuit
    let circuit = LinearEquationCircuit {
        a,
        b,
        x,
        len_b,
        len_a,
    };

    // Generate proving and verifying keys
    let mut rng = StdRng::seed_from_u64(0u64);

    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

    // Create a proof
    let start_proving = ark_std::time::Instant::now();
    let proof = Groth16::<Bls12_381>::prove(&pk, circuit.clone(), &mut rng).unwrap();
    let proving_time = start_proving.elapsed().as_secs_f64();

    let start_verifying = ark_std::time::Instant::now();
    let is_correct = Groth16::<Bls12_381>::verify(&vk, &public_input, &proof).unwrap();
    let verifying_time = start_verifying.elapsed().as_secs_f64();

    // Create the output data
    let result = OutputData {
        proof: format!("{:?}", proof),
        public_input: public_input.iter().map(|x| format!("{:?}", x)).collect(),
        num_constraints: cs.num_constraints(),
        num_variables: cs.num_instance_variables(),
        proving_time: 0.0,
        verifying_time: 0.0,
    };
    HttpResponse::Ok().json(result)
}
//
/*
use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::{Field, PrimeField};
use ark_groth16::Groth16;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::R1CSVar;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_snark::{CircuitSpecificSetupSNARK, SNARK};
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;
use ark_std::Zero;
// Define the linear equation circuit
#[derive(Clone)]
struct LinearEquationCircuit<ConstraintF: PrimeField> {
    a: Vec<Vec<ConstraintF>>, // Matrix A
    b: Vec<ConstraintF>,      // Vector b
    x: Vec<ConstraintF>,      // Solution vector x
    len_b: usize,             // Length of the vector b
    len_a: usize,             // Length of the matrix A
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


make it accsesiable by the web like i did here:
use crate::constraints::{InputData, OutputData, PrimeCircut};
use actix_web::{post, web, HttpResponse, Responder};
use ark_bls12_381::{Bls12_381, Fr as BlsFr};

use ark_groth16::Groth16;
use ark_relations::r1cs::ConstraintSynthesizer;
use ark_relations::r1cs::ConstraintSystem;
use ark_snark::SNARK;
use ark_std::rand::SeedableRng;
use rand::rngs::StdRng;
use std::time::Instant;

pub async fn prime_snark_compute(data: web::Json<InputData>) -> impl Responder {
    let mut rng = StdRng::seed_from_u64(42);
    let x = BlsFr::from(data.x);
    let num_of_rounds = data.num_of_rounds;

    let circuit = PrimeCircut {
        x: Some(x),
        num_of_rounds,
    };

    let start = Instant::now();
    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng).unwrap();
    let setup_time = start.elapsed().as_secs_f64();

    let circuit2 = PrimeCircut {
        x: Some(x),
        num_of_rounds,
    };

    let start = Instant::now();
    let proof = Groth16::<Bls12_381>::prove(&pk, circuit2, &mut rng).unwrap();
    let proving_time = start.elapsed().as_secs_f64();

    let cs_too = ConstraintSystem::new_ref();
    circuit2.generate_constraints(cs_too.clone()).unwrap();
    let public_input = cs_too.borrow().unwrap().instance_assignment.clone();

    let start = Instant::now();
    let found_prime = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
    let verifying_time = start.elapsed().as_secs_f64();

    let result = OutputData {
        proof: format!("{:?}", proof),
        public_input: public_input.iter().map(|x| format!("{:?}", x)).collect(),
        num_constraints: cs_too.num_constraints(),
        num_variables: cs_too.num_instance_variables(),
        proving_time,
        verifying_time,
        found_prime,
    };

    HttpResponse::Ok().json(result)
}

*/
#[cfg(test)]
mod tests {
    use ark_relations::r1cs::ConstraintSystem;

    use super::*;

    #[test]
    fn test_linear_equation_circuit() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        // Define the matrix A
        let a: Vec<Vec<Fr>> = vec![
            vec![Fr::from(2u32), Fr::from(1u32)],
            vec![Fr::from(1u32), Fr::from(3u32)],
        ];

        // Define the vector b
        let b: Vec<Fr> = vec![Fr::from(3u32), Fr::from(4u32)];
        let mut public_input: Vec<Fr> = vec![];
        // create a public input like this : first push a row of matrix A, then push the 1 element of vector b:
        // [a[0][0], a[0][1], b[0], a[1][0], a[1][1], b[1]]
        // the code:
        for i in 0..a.len() {
            for j in 0..a[0].len() {
                public_input.push(a[i][j]);
            }
            public_input.push(b[i]);
        }
        println!("Public input my {:?}", public_input);

        let len_b = b.len();
        let len_a = a[0].len();
        // Define the solution vector x
        let x: Vec<Fr> = vec![Fr::from(1u32), Fr::from(1u32)];

        // Create a linear equation circuit
        let circuit = LinearEquationCircuit {
            a,
            b: b.clone(),
            x,
            len_b,
            len_a,
        };
        println!("Public input real {:?}", public_input);

        // Generate proving and verifying keys
        let mut rng = StdRng::seed_from_u64(0u64);
        let (pk, vk) =
            Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

        // Create a proof
        let proof = Groth16::<Bls12_381>::prove(&pk, circuit.clone(), &mut rng).unwrap();
        // the code:
        print!("Public input {:?}", public_input);
        // Verify the proof
        // let res = cs.is_satisfied().unwrap();
        // assert!(res);
        let is_correct = Groth16::<Bls12_381>::verify(&vk, &public_input, &proof).unwrap();
        assert!(is_correct);
    }
}

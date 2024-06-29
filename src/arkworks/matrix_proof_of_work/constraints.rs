use crate::matrix_proof_of_work::alloc::{FpVar2D, FpVarArray};
use crate::matrix_proof_of_work::cmp::CmpGadget;
use crate::matrix_proof_of_work::hasher::{hasher, hasher_var};
use ark_bls12_381::Fr;
use ark_ff::{Fp, PrimeField, Zero};
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::prelude::AllocationMode;
use ark_r1cs_std::R1CSVar;
use ark_r1cs_std::{
    prelude::{AllocVar, Boolean, EqGadget},
    uint8::UInt8,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use std::collections::hash_map;
use std::ops::{Add, AddAssign, Div, Mul, MulAssign};
// 1. The matrix is a square matrix
// 2. The matrix is a Uint8 matrix

pub struct MatrixCircuit<const N: usize, ConstraintF: PrimeField> {
    matrix_a: [[u64; N]; N],
    matrix_b: [[u64; N]; N],
    hash_of_c: ConstraintF,
}

// implement clone for MatrixCircuit
impl<const N: usize, ConstraintF: PrimeField> Clone for MatrixCircuit<N, ConstraintF> {
    fn clone(&self) -> Self {
        Self {
            matrix_a: self.matrix_a.clone(),
            matrix_b: self.matrix_b.clone(),
            hash_of_c: self.hash_of_c.clone(),
        }
    }
}
// create a function that thake 2 matrix and multiply them
// Matrixa is  2DFpVar
// Matrixb is  2DFpVar
// Matrixc is  2DFpVar

// i wanna implement the get_element function for the FpVar2D
impl<const N: usize, ConstraintF: PrimeField> FpVar2D<N, ConstraintF> {
    pub fn get_element(&self, i: usize, j: usize) -> Result<FpVar<ConstraintF>, SynthesisError> {
        Ok(self.0[i][j].clone())
    }
}
// i wanna implement the get_element function for the FpVarArray
impl<const N: usize, ConstraintF: PrimeField> FpVarArray<N, ConstraintF> {
    pub fn get_element(&self, i: usize) -> Result<FpVar<ConstraintF>, SynthesisError> {
        Ok(self.0[i].clone())
    }
}
fn matrix_mul<const N: usize, ConstraintF: PrimeField>(
    cs: ConstraintSystemRef<ConstraintF>,
    matrix_a: FpVar2D<N, ConstraintF>,
    matrix_b: FpVar2D<N, ConstraintF>,
) -> FpVar2D<N, ConstraintF> {
    // create a new variable to hold the result of the multiplication
    let mut matrix_c = FpVar2D::new_witness(cs.clone(), || Ok([[0u64; N]; N])).unwrap();
    // implement the multiplication of the two matrices
    for i in 0..N {
        for j in 0..N {
            let mut sum =
                FpVar::<ConstraintF>::new_witness(cs.clone(), || Ok(ConstraintF::zero())).unwrap();
            for k in 0..N {
                let ij = matrix_a.get_element(i, k).unwrap();
                let jk = matrix_b.get_element(k, j).unwrap();
                let product = ij.clone() * jk.clone();
                sum.add_assign(&product);
                ij.mul_equals(&jk, &product).unwrap();
            }
            matrix_c.0[i][j] = sum;
        }
    }
    matrix_c
}

// take a struct that hold the input and implement the ConstraintSynthesizer trait:
impl<const N: usize, ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF>
    for MatrixCircuit<N, ConstraintF>
{
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // Implement constraints for multiplying two boolean matrices A and B:
        // 1. The matrices are square matrices
        // create new 2 witness variables for matrix A and B:
        // 1. matrix_a: FpVar2D
        // 2. matrix_b: FpVar2D
        // 3. output new_variable for the result of the multiplication
        // 4. Ensure that the result of the multiplication is equal to the hash of the matrix C
        // use self.matrix_a and self.matrix_b to create the witness variables:
        let matrix_a_var: FpVar2D<N, ConstraintF> =
            FpVar2D::new_witness(cs.clone(), || Ok(self.matrix_a)).unwrap();
        let matrix_b_var: FpVar2D<N, ConstraintF> =
            FpVar2D::new_witness(cs.clone(), || Ok(self.matrix_b)).unwrap();

        // create a new variable to hold the result of the multiplication
        let matrix_c_var = matrix_mul(cs.clone(), matrix_a_var, matrix_b_var);

        // hash the matrix_c_var and ensure that it is equal to the hash_of_c
        let hash = &hasher_var::<N, ConstraintF>(cs.clone(), &matrix_c_var).unwrap()[0];

        // create a public input for the hash:
        let hash_public_input = FpVar::<ConstraintF>::new_input(cs.clone(), || {
            println!("{}", self.hash_of_c);
            Ok(self.hash_of_c)
        })
        .unwrap();
        println!(
            "{:?}",
            hash_public_input
                .value()
                .unwrap_or_else(|_| ConstraintF::zero())
        );
        // ensure that the hash of the matrix_c_var is equal to the hash_of_c
        hash.enforce_equal(&hash_public_input).unwrap();

        Ok(())
    }
}

// create tests for the matrix multiplication:
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Bls12_381, Config, Fr as Fp};
    use ark_ec::bls12::Bls12;
    use ark_groth16::prepare_verifying_key;
    use ark_groth16::{Groth16, Proof};
    use ark_relations::r1cs::{ConstraintLayer, ConstraintSystem, TracingMode};
    use ark_snark::{CircuitSpecificSetupSNARK, SNARK};
    use ark_std::test_rng;
    use rand::{RngCore, SeedableRng};
    use tracing_subscriber::layer::SubscriberExt;

    #[test]
    //
    fn test_matrix_multiplication() {
        let rng = &mut test_rng();
        let cs = ConstraintSystem::<Fp>::new_ref();

        let matrix_a = [[1u64; 2]; 2];
        let matrix_b = [[1u64; 2]; 2];
        println!("matrix_a: {:?}", matrix_a);
        println!("matrix_b: {:?}", matrix_b);

        let matrix_c = matrix_mul(
            cs.clone(),
            FpVar2D::new_witness(cs.clone(), || Ok(matrix_a)).unwrap(),
            FpVar2D::new_witness(cs.clone(), || Ok(matrix_b)).unwrap(),
        );

        assert_eq!(matrix_c.0[0][0].value().unwrap(), Fp::from(2u64));
        assert_eq!(matrix_c.0[0][1].value().unwrap(), Fp::from(2u64));
        assert_eq!(matrix_c.0[1][0].value().unwrap(), Fp::from(2u64));
        assert_eq!(matrix_c.0[1][1].value().unwrap(), Fp::from(2u64));
    }
    #[test]
    fn is_satisfied_constraints() {
        let rng = &mut test_rng();
        let cs = ConstraintSystem::<Fp>::new_ref();

        // matrix A :
        //  (1 0
        //   0 1)
        // create matrix A:
        let matrix_a = [[1, 2], [3, 4]];
        let matrix_b = [[4, 3], [2, 1]];
        println!("matrix_a: {:?}", matrix_a);
        println!("matrix_b: {:?}", matrix_b);

        let matrix_c = matrix_mul(
            cs.clone(),
            FpVar2D::new_witness(cs.clone(), || Ok(matrix_a)).unwrap(),
            FpVar2D::new_witness(cs.clone(), || Ok(matrix_b)).unwrap(),
        );

        // create a new instance of the MatrixCircuit
        let hash = hasher(&matrix_c).unwrap();
        // take the first element of the hash
        let hash_value = hash[0];
        let circuit = MatrixCircuit {
            matrix_a,
            matrix_b,
            hash_of_c: hash_value,
        };
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }
    #[test]
    fn groth16_correctness_and_soundness() {
        let cs = ConstraintSystem::<Fp>::new_ref();
        let matrix_a = [[1, 2], [3, 4]];
        let matrix_b = [[4, 3], [2, 1]];
        let matrix_c = matrix_mul(
            cs.clone(),
            FpVar2D::new_witness(cs.clone(), || Ok(matrix_a)).unwrap(),
            FpVar2D::new_witness(cs.clone(), || Ok(matrix_b)).unwrap(),
        );
        let hash = hasher(&matrix_c).unwrap();
        let hash_value = hash[0];
        let circuit = MatrixCircuit {
            matrix_a,
            matrix_b,
            hash_of_c: hash_value,
        };
        // generate the proof
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
        let (pk, vk) = Groth16::<Bls12_381>::setup(circuit.clone(), &mut rng).unwrap();
        let pvk = prepare_verifying_key::<Bls12_381>(&vk);
        let proof: Proof<Bls12<Config>> =
            Groth16::<Bls12_381>::prove(&pk, circuit, &mut rng).unwrap();

        // test some verification checks
        assert!(
            Groth16::<Bls12_381>::verify_with_processed_vk(&pvk, &[hash_value], &proof).unwrap()
        );
        let false_hash = Fr::zero();
        assert!(
            !Groth16::<Bls12_381>::verify_with_processed_vk(&pvk, &[false_hash], &proof).unwrap()
        );
    }
    #[test]
    fn big_matrix_20x20() {
        let cs = ConstraintSystem::<Fp>::new_ref();
        let mut matrix_a = [[0u64; 20]; 20];
        let mut matrix_b = [[0u64; 20]; 20];
        for i in 0..20 {
            for j in 0..20 {
                matrix_a[i][j] = i as u64;
                matrix_b[i][j] = j as u64;
            }
        }
        let matrix_c = matrix_mul(
            cs.clone(),
            FpVar2D::new_witness(cs.clone(), || Ok(matrix_a)).unwrap(),
            FpVar2D::new_witness(cs.clone(), || Ok(matrix_b)).unwrap(),
        );
        let hash = hasher(&matrix_c).unwrap();
        let hash_value = hash[0];
        let circuit = MatrixCircuit {
            matrix_a,
            matrix_b,
            hash_of_c: hash_value,
        };
        // generate the proof
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
        let (pk, vk) = Groth16::<Bls12_381>::setup(circuit.clone(), &mut rng).unwrap();
        let pvk = prepare_verifying_key::<Bls12_381>(&vk);
        let proof: Proof<Bls12<Config>> =
            Groth16::<Bls12_381>::prove(&pk, circuit, &mut rng).unwrap();

        // test some verification checks
        assert!(
            Groth16::<Bls12_381>::verify_with_processed_vk(&pvk, &[hash_value], &proof).unwrap()
        );
    }
}

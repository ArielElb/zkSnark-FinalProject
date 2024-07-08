use crate::arkworks::matrix_proof_of_work::hasher::{hasher, hasher_var};
use crate::arkworks::matrix_proof_of_work::io::{read_proof, write_proof_to_file};
use ark_bls12_381::Fr;
use ark_ff::PrimeField;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::prelude::{AllocVar, EqGadget};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use std::ops::{Add, AddAssign, Div, Mul, MulAssign};

use super::alloc::FpVar2DVec;
use super::alloc::FpVarVec;
// 1. The matrix is a square matrix
// 2. The matrix is a Uint8 matrix

pub struct MatrixCircuit<F: PrimeField> {
    matrix_a: Vec<Vec<u64>>, // witness
    matrix_b: Vec<Vec<u64>>, // witness
    hash_of_a: F,            // public input
    hash_of_b: F,            // public input
    hash_of_c: F,            // public input
}
#[derive(Clone, Debug)]
pub struct MatrixCircuit2<F: PrimeField> {
    matrix_a: Vec<Vec<u64>>,
    matrix_b: Vec<Vec<u64>>,
    hash_of_c: F,
    num_constraints: usize,
    num_variables: usize,
}

// implement new for MatrixCircuit:
impl<F: PrimeField> MatrixCircuit<F> {
    pub fn new(
        matrix_a: Vec<Vec<u64>>,
        matrix_b: Vec<Vec<u64>>,
        hash_of_a: F,
        hash_of_b: F,
        hash_of_c: F,
    ) -> Self {
        Self {
            matrix_a,
            matrix_b,
            hash_of_a,
            hash_of_b,
            hash_of_c,
        }
    }
}
// implement clone for MatrixCircuit
impl<ConstraintF: PrimeField> Clone for MatrixCircuit<ConstraintF> {
    fn clone(&self) -> Self {
        Self {
            matrix_a: self.matrix_a.clone(),
            matrix_b: self.matrix_b.clone(),
            hash_of_a: self.hash_of_a.clone(),
            hash_of_b: self.hash_of_b.clone(),
            hash_of_c: self.hash_of_c.clone(),
        }
    }
}
// create a function that thake 2 matrix and multiply them
// Matrixa is  2DFpVar
// Matrixb is  2DFpVar
// Matrixc is  2DFpVar

impl<F: PrimeField> FpVarVec<F> {
    pub fn get_element(&self, i: usize) -> Result<FpVar<F>, SynthesisError> {
        Ok(self.0[i].clone())
    }
}

impl<F: PrimeField> FpVar2DVec<F> {
    pub fn get_element(&self, i: usize, j: usize) -> Result<FpVar<F>, SynthesisError> {
        Ok(self.0[i][j].clone())
    }
}
pub fn matrix_mul<F: PrimeField>(
    cs: ConstraintSystemRef<F>,
    matrix_a: FpVar2DVec<F>,
    matrix_b: FpVar2DVec<F>,
) -> FpVar2DVec<F> {
    let n = matrix_a.0.len();
    let mut matrix_c = FpVar2DVec::new_witness(cs.clone(), || Ok(vec![vec![0u64; n]; n])).unwrap();
    for i in 0..n {
        for j in 0..n {
            let mut sum = FpVar::<F>::new_witness(cs.clone(), || Ok(F::zero())).unwrap();
            for k in 0..n {
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

impl<F: PrimeField> ConstraintSynthesizer<F> for MatrixCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let hash_public_input_a = FpVar::<F>::new_input(cs.clone(), || Ok(self.hash_of_a)).unwrap();
        let hash_public_input_b = FpVar::<F>::new_input(cs.clone(), || Ok(self.hash_of_b)).unwrap();

        let matrix_a_var: FpVar2DVec<F> =
            FpVar2DVec::new_witness(cs.clone(), || Ok(self.matrix_a)).unwrap();
        let matrix_b_var: FpVar2DVec<F> =
            FpVar2DVec::new_witness(cs.clone(), || Ok(self.matrix_b)).unwrap();
        // hash matrix a:
        let hash_a = &hasher_var::<F>(cs.clone(), &matrix_a_var).unwrap()[0];
        // hash matrix b:
        let hash_b = &hasher_var::<F>(cs.clone(), &matrix_b_var).unwrap()[0];
        hash_a.enforce_equal(&hash_public_input_a).unwrap();
        hash_b.enforce_equal(&hash_public_input_b).unwrap();

        let matrix_c_var = matrix_mul(cs.clone(), matrix_a_var.clone(), matrix_b_var.clone());

        // hash the matrix c
        let hash_c = &hasher_var::<F>(cs.clone(), &matrix_c_var).unwrap()[0];

        let hash_public_input_c = FpVar::<F>::new_input(cs.clone(), || Ok(self.hash_of_c)).unwrap();

        hash_c.enforce_equal(&hash_public_input_c).unwrap();

        Ok(())
    }
}

impl<F: PrimeField> ConstraintSynthesizer<F> for MatrixCircuit2<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let matrix_a_var: FpVar2DVec<F> =
            FpVar2DVec::new_witness(cs.clone(), || Ok(self.matrix_a)).unwrap();
        let matrix_b_var: FpVar2DVec<F> =
            FpVar2DVec::new_witness(cs.clone(), || Ok(self.matrix_b)).unwrap();

        // let hash_public_input_a = FpVar::<F>::new_input(cs.clone(), || Ok(self.hash_of_a)).unwrap();
        // let hash_public_input_b = FpVar::<F>::new_input(cs.clone(), || Ok(self.hash_of_b)).unwrap();
        let matrix_c_var = matrix_mul(cs.clone(), matrix_a_var, matrix_b_var);

        let hash = &hasher_var::<F>(cs.clone(), &matrix_c_var).unwrap()[0];

        let hash_public_input = FpVar::<F>::new_input(cs.clone(), || Ok(self.hash_of_c)).unwrap();

        hash.enforce_equal(&hash_public_input).unwrap();

        Ok(())
    }
}
// create tests for the matrix multiplication:
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Bls12_381, Config, Fr as Fp};
    use ark_crypto_primitives::sponge::CryptographicSponge;
    use ark_ec::bls12::Bls12;
    use ark_groth16::prepare_verifying_key;
    use ark_groth16::{Groth16, Proof};
    use ark_marlin::{Marlin, SimplePoseidonRng};
    use ark_poly::polynomial::univariate::DensePolynomial;
    use ark_poly_commit::marlin_pc::MarlinKZG10;
    use ark_r1cs_std::R1CSVar;
    use ark_relations::r1cs::{ConstraintLayer, ConstraintSystem, TracingMode};
    use ark_snark::{CircuitSpecificSetupSNARK, SNARK};
    use ark_std::ops::MulAssign;
    use ark_std::test_rng;
    use itertools::Itertools;
    use rand::{RngCore, SeedableRng};
    use tracing_subscriber::layer::SubscriberExt;
    type S = SimplePoseidonRng<Fr>;
    type MultiPC = MarlinKZG10<Bls12_381, DensePolynomial<Fr>, S>;
    type MarlinInst = Marlin<Fr, MultiPC, S>;

    #[test]
    //
    fn test_matrix_multiplication() {
        let rng = &mut test_rng();
        let cs = ConstraintSystem::<Fp>::new_ref();

        let matrix_a = vec![vec![1u64, 1], vec![1, 1]];
        let matrix_b = vec![vec![1u64, 1], vec![1, 1]];
        println!("matrix_a: {:?}", matrix_a);
        println!("matrix_b: {:?}", matrix_b);

        let matrix_c = matrix_mul(
            cs.clone(),
            FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_a)).unwrap(),
            FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_b)).unwrap(),
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
        let matrix_a = vec![vec![1u64, 2], vec![3, 4]];
        let matrix_b = vec![vec![4u64, 3], vec![2, 1]];
        let matrix_a_var = FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_a.clone())).unwrap();
        let matrix_b_var = FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_b.clone())).unwrap();
        println!("matrix_a: {:?}", matrix_a);
        println!("matrix_b: {:?}", matrix_b);

        let matrix_c = matrix_mul(cs.clone(), matrix_a_var.clone(), matrix_b_var.clone());
        // create hash_a :
        let hash_a = hasher(&matrix_a_var).unwrap()[0];
        // create hash_b :
        let hash_b = hasher(&matrix_b_var).unwrap()[0];
        // create a new instance of the MatrixCircuit
        let hash = hasher(&matrix_c).unwrap();
        // take the first element of the hash
        let hash_value = hash[0];
        let circuit = MatrixCircuit {
            matrix_a,
            matrix_b,
            hash_of_c: hash_value,
            hash_of_a: hash_a,
            hash_of_b: hash_b,
        };
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }
    #[test]
    fn groth16_correctness_and_soundness() {
        let cs = ConstraintSystem::<Fp>::new_ref();
        let matrix_a = vec![vec![1u64, 2], vec![3, 4]]; // witness
        let matrix_b = vec![vec![4u64, 3], vec![2, 1]]; // witness
                                                        // multiply the matrices
        let matrix_a_var = FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_a.clone())).unwrap();
        let matrix_b_var = FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_b.clone())).unwrap();
        let matrix_c = matrix_mul(cs.clone(), matrix_a_var.clone(), matrix_b_var.clone());
        // create hash_a :
        let hash_a = hasher(&matrix_a_var).unwrap()[0];
        // create hash_b :
        let hash_b = hasher(&matrix_b_var).unwrap()[0];
        // create a new instance of the MatrixCircuit
        let hash_c = hasher(&matrix_c).unwrap()[0];
        let circuit = MatrixCircuit {
            matrix_a,
            matrix_b,
            hash_of_c: hash_c,
            hash_of_a: hash_a,
            hash_of_b: hash_b,
        };

        // generate the proof
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
        let (pk, vk) = Groth16::<Bls12_381>::setup(circuit.clone(), &mut rng).unwrap();
        let pvk = Groth16::<Bls12_381>::process_vk(&vk).unwrap();
        let proof: Proof<Bls12<Config>> =
            Groth16::<Bls12_381>::prove(&pk, circuit, &mut rng).unwrap();
        // test IO
        let file_path = "./proof.bin";
        write_proof_to_file(&proof, file_path).unwrap();
        let read_proof: Proof<Bls12<Config>> = read_proof::<Bls12_381>(file_path).unwrap();

        // test some verification checks
        assert!(Groth16::<Bls12_381>::verify_with_processed_vk(
            &pvk,
            &[hash_a, hash_b, hash_c],
            &proof
        )
        .unwrap());
    }
    #[test]
    fn big_matrix_20x20() {
        let cs = ConstraintSystem::<Fp>::new_ref();
        // create vector of vectors of size 20x20:
        let mut matrix_a = vec![vec![0u64; 20]; 20];
        let mut matrix_b = vec![vec![0u64; 20]; 20];

        for i in 0..20 {
            for j in 0..20 {
                matrix_a[i][j] = i as u64;
                matrix_b[i][j] = j as u64;
            }
        }
        // multiply the matrices
        let matrix_a_var = FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_a.clone())).unwrap();
        let matrix_b_var = FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_b.clone())).unwrap();
        let matrix_c = matrix_mul(cs.clone(), matrix_a_var.clone(), matrix_b_var.clone());
        // create hash_a :
        let hash_a = hasher(&matrix_a_var).unwrap()[0];
        // create hash_b :
        let hash_b = hasher(&matrix_b_var).unwrap()[0];
        // create a new instance of the MatrixCircuit
        let hash_c = hasher(&matrix_c).unwrap()[0];
        let circuit = MatrixCircuit {
            matrix_a,
            matrix_b,
            hash_of_c: hash_c,
            hash_of_a: hash_a,
            hash_of_b: hash_b,
        };

        // generate the proof
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
        let (pk, vk) = Groth16::<Bls12_381>::setup(circuit.clone(), &mut rng).unwrap();
        let pvk = Groth16::<Bls12_381>::process_vk(&vk).unwrap();
        let proof: Proof<Bls12<Config>> =
            Groth16::<Bls12_381>::prove(&pk, circuit, &mut rng).unwrap();

        // test some verification checks
        assert!(Groth16::<Bls12_381>::verify_with_processed_vk(
            &pvk,
            &[hash_a, hash_b, hash_c],
            &proof
        )
        .unwrap());
    }
    #[test]
    fn big_matrix_10x10() {
        let cs = ConstraintSystem::<Fp>::new_ref();
        // generate random matrix of size 10x10
        let rng = &mut test_rng();
        let mut matrix_a = vec![vec![0u64; 10]; 10];
        let mut matrix_b = vec![vec![0u64; 10]; 10];
        for i in 0..10 {
            for j in 0..10 {
                matrix_a[i][j] = rng.next_u64();
                matrix_b[i][j] = rng.next_u64();
            }
        }
        // multiply the matrices
        let matrix_a_var = FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_a.clone())).unwrap();
        let matrix_b_var = FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_b.clone())).unwrap();
        let matrix_c = matrix_mul(cs.clone(), matrix_a_var.clone(), matrix_b_var.clone());
        // create hash_a :
        let hash_a = hasher(&matrix_a_var).unwrap()[0];
        // create hash_b :
        let hash_b = hasher(&matrix_b_var).unwrap()[0];
        // create a new instance of the MatrixCircuit
        let hash_c = hasher(&matrix_c).unwrap()[0];
        let circuit = MatrixCircuit {
            matrix_a,
            matrix_b,
            hash_of_c: hash_c,
            hash_of_a: hash_a,
            hash_of_b: hash_b,
        };

        // generate the proof
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
        let (pk, vk) = Groth16::<Bls12_381>::setup(circuit.clone(), &mut rng).unwrap();
        let pvk = Groth16::<Bls12_381>::process_vk(&vk).unwrap();
        let proof: Proof<Bls12<Config>> =
            Groth16::<Bls12_381>::prove(&pk, circuit, &mut rng).unwrap();

        // test some verification checks
        assert!(Groth16::<Bls12_381>::verify_with_processed_vk(
            &pvk,
            &[hash_a, hash_b, hash_c],
            &proof
        )
        .unwrap());
    }
    #[test]
    fn marlin_proof_system() {
        let mut rng_seed = ark_std::test_rng();
        let mut rng: SimplePoseidonRng<Fr> = SimplePoseidonRng::default();
        rng.absorb(&rng_seed.next_u64());

        let cs = ConstraintSystem::<Fp>::new_ref();
        let matrix_a = vec![vec![1u64, 2], vec![3, 4]]; // witness
        let matrix_b = vec![vec![4u64, 3], vec![2, 1]]; // witness
                                                        // multiply the matrices
        let matrix_c = matrix_mul(
            cs.clone(),
            FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_a.clone())).unwrap(),
            FpVar2DVec::new_witness(cs.clone(), || Ok(matrix_b.clone())).unwrap(),
        );
        // calculate the hash of the matrix - native
        let hash = hasher(&matrix_c).unwrap();
        let hash_value = hash[0];

        let circuit = MatrixCircuit2 {
            matrix_a,              // witness
            matrix_b,              // witness
            hash_of_c: hash_value, // public input
            num_constraints: 1000,
            num_variables: 250,
        };
        circuit.clone().generate_constraints(cs.clone()).unwrap();

        let num_constraints = cs.num_constraints();
        println!("Number of constraints: {}", num_constraints);

        let universal_srs = MarlinInst::universal_setup(1000, 25, 300, &mut rng).unwrap();

        let (index_pk, index_vk) = MarlinInst::index(&universal_srs, circuit.clone()).unwrap();
        println!("Called index");

        let proof = MarlinInst::prove(&index_pk, circuit, &mut rng).unwrap();
        println!("Called prover");
        assert!(MarlinInst::verify(&index_vk, &[hash_value], &proof, &mut rng).unwrap());
        println!("Called verifier");
    }
}

use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::sponge::CryptographicSponge;
use ark_ff::PrimeField;
use ark_marlin::{Marlin, SimplePoseidonRng};
use ark_poly::polynomial::univariate::DensePolynomial;
use ark_poly_commit::marlin_pc::MarlinKZG10;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::prelude::{AllocVar, EqGadget};
use ark_r1cs_std::R1CSVar;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_std::ops::MulAssign;
use ark_std::rand::RngCore;
use itertools::Itertools;
#[derive(Clone)]
pub struct FibonacciCircuit<F: PrimeField> {
    pub a: Option<F>,
    pub b: Option<F>,
    pub num_of_steps: usize,
    pub result: Option<F>,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for FibonacciCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let mut fi_minus_one = FpVar::<F>::new_input(cs.clone(), || {
            self.b.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let mut fi_minus_two = FpVar::<F>::new_input(cs.clone(), || {
            self.a.ok_or(SynthesisError::AssignmentMissing)
        })?;
        // create one dummy variable for making it 2^n - 1
        let _dummy = FpVar::<F>::new_input(cs.clone(), || Ok(F::one()))?;
        let saved_result = FpVar::<F>::new_witness(cs.clone(), || {
            self.result.ok_or(SynthesisError::AssignmentMissing)
        })?;
        // Initialize fi as a witness variable
        let mut fi = FpVar::<F>::new_witness(cs.clone(), || Ok(F::zero()))?;
        // Do the loop only when verifying the circuit
        for _i in 1..self.num_of_steps {
            fi = fi_minus_one.clone() + &fi_minus_two;
            fi.enforce_equal(&(&fi_minus_one + &fi_minus_two))?;
            fi_minus_two = fi_minus_one;
            fi_minus_one = fi.clone();
        }

        fi.enforce_equal(&saved_result)?;
        // println!("fi {}", fi.value().unwrap());

        Ok(())
    }
}

mod marlin {
    use super::*;
    use ark_crypto_primitives::sponge::CryptographicSponge;
    use ark_marlin::{Marlin, SimplePoseidonRng};
    use ark_relations::r1cs::ConstraintSystem;
    use itertools::Itertools;

    use ark_bls12_381::{Bls12_381, Fr};
    use ark_poly::polynomial::univariate::DensePolynomial;
    use ark_poly_commit::marlin_pc::MarlinKZG10;
    use ark_std::ops::MulAssign;
    use ark_std::rand::RngCore;

    type S = SimplePoseidonRng<Fr>;
    type MultiPC = MarlinKZG10<Bls12_381, DensePolynomial<Fr>, S>;
    type MarlinInst = Marlin<Fr, MultiPC, S>;

    fn test_fibonacci_circuit(num_of_steps: usize) {
        let mut rng_seed = ark_std::test_rng();
        let mut rng: SimplePoseidonRng<Fr> = SimplePoseidonRng::default();
        rng.absorb(&rng_seed.next_u64());

        let universal_srs = MarlinInst::universal_setup(300, 300, 300, &mut rng).unwrap();

        let a = Fr::from(1u64);
        let b = Fr::from(1u64);
        let mut fi = Fr::from(0u64);
        let mut fi_minus_one = b;
        let mut fi_minus_two = a;

        // witness - the result of the fibonacci
        for _ in 1..num_of_steps {
            let new_fi = fi_minus_one + fi_minus_two;
            fi_minus_two = fi_minus_one;
            fi_minus_one = new_fi;
            fi = new_fi;
        }
        // Ensure the number of public inputs matches 2^n - 1
        // Here we have 2 inputs (a, b), we need to adjust accordingly
        let _num_inputs = 2 + 1; // a, b, and result
        let mut inputs = vec![a, b];
        inputs.push(Fr::from(1));

        let circ = FibonacciCircuit {
            a: Some(a),
            b: Some(b),
            num_of_steps,
            result: Some(fi),
        };

        let (index_pk, index_vk) = MarlinInst::index(&universal_srs, circ.clone()).unwrap();
        println!("Called index");

        let proof = MarlinInst::prove(&index_pk, circ.clone(), &mut rng).unwrap();
        println!("Called prover");

        assert!(MarlinInst::verify(&index_vk, &inputs, &proof, &mut rng).unwrap());
        println!("Called verifier");
        inputs.clear();
        inputs.push(a);
        inputs.push(b);
        // will make the result 0 - meaning failed to
        inputs.push(Fr::from(0));
        // should fail
        assert!(!MarlinInst::verify(&index_vk, &inputs, &proof, &mut rng).unwrap());

        // create "wastefull cs" to check gow many constraints are:
        let cs = ConstraintSystem::<Fr>::new_ref();

        circ.clone().generate_constraints(cs.clone()).unwrap();
        println!("Number of constraints: {}", cs.num_constraints());
        println!(
            "Number of  public input variables: {}",
            cs.num_instance_variables() - 1
        );

        println!(
            "Number of privte input variables :  {}",
            cs.num_witness_variables()
        );
    }

    #[test]
    fn prove_and_verify_fibonacci() {
        test_fibonacci_circuit(100);
    }
}

mod groth16 {
    use super::*;
    use ark_bls12_381::{Bls12_381, Fr as BlsFr};
    use ark_groth16::{prepare_verifying_key, Groth16};

    use ark_relations::r1cs::ConstraintSystem;
    use ark_snark::SNARK;
    use ark_std::rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn check_constraints() {
        let cs = ConstraintSystem::<BlsFr>::new_ref();
        let a = BlsFr::from(0u64);
        let b = BlsFr::from(1u64);
        let num_of_steps = 10;
        let mut fi_minus_one = b;
        let mut fi_minus_two = a;
        let mut fi = BlsFr::from(0);

        // witness - the result of the fibonacci
        for _ in 1..num_of_steps {
            fi = fi_minus_one + fi_minus_two;
            fi_minus_two = fi_minus_one;
            fi_minus_one = fi;
        }

        // Ensure the number of public inputs matches 2^n - 1
        // Here we have 2 inputs (a, b), we need to adjust accordingly
        let num_inputs = 2 + 1; // a, b, and result
        let mut inputs = vec![a, b];
        inputs.push(BlsFr::from(1));

        println!("Inputs: {:?}", inputs);

        println!("Result: {:?}", fi);
        let circ = FibonacciCircuit {
            a: Some(a),
            b: Some(b),
            num_of_steps,
            result: Some(fi),
        };

        circ.generate_constraints(cs.clone()).unwrap();
        println!("Number of constraints: {}", cs.num_constraints());
        println!(
            "Number of  public input variables: {}",
            cs.num_instance_variables() - 1
        );

        let res = cs.is_satisfied().unwrap();
        println!("Constraints satisfied: {}", res);
    }
    #[test]
    fn groth16() {
        let rng = &mut StdRng::seed_from_u64(0);
        let cs = ConstraintSystem::<BlsFr>::new_ref();
        let a = BlsFr::from(0u64);
        let b = BlsFr::from(1u64);
        let num_of_steps = 50;
        let mut fi = BlsFr::from(0);
        let mut fi_minus_one = a;
        let mut fi_minus_two = b;

        // witness - the result of the fibonacci
        for _ in 1..num_of_steps {
            fi = fi_minus_one + fi_minus_two;
            fi_minus_two = fi_minus_one;
            fi_minus_one = fi;
        }
        // Ensure the number of public inputs matches 2^n - 1
        // Here we have 2 inputs (a, b), we need to adjust accordingly
        let num_inputs = 2 + 1; // a, b, and result
        let mut inputs = vec![a, b];
        inputs.push(BlsFr::from(1));

        println!("Inputs: {:?}", inputs);

        println!("Result: {:?}", fi);
        let circ = FibonacciCircuit {
            a: Some(a),
            b: Some(b),
            num_of_steps,
            result: Some(fi),
        };

        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circ.clone(), rng).unwrap();
        let pvk = prepare_verifying_key(&vk);

        let proof = Groth16::<Bls12_381>::prove(&pk, circ.clone(), rng).unwrap();
        assert!(Groth16::<Bls12_381>::verify_with_processed_vk(&pvk, &inputs, &proof).unwrap());
    }
}

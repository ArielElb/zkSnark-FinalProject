use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::Field;
use ark_groth16::Groth16;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_snark::CircuitSpecificSetupSNARK;
use ark_snark::SNARK;
use ark_std::rand::RngCore;
use ark_std::rand::{Rng, SeedableRng};
use ark_std::UniformRand;

use ark_relations::r1cs::{
    ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, SynthesisError,
};

// Define the factorization circuit
// derive clone
#[derive(Clone)]
struct FactorizationCircuit {
    pub a: Option<Fr>,
    pub b: Option<Fr>,
    pub product: Option<Fr>,
}

impl ConstraintSynthesizer<Fr> for FactorizationCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let a = FpVar::new_witness(ark_relations::ns!(cs, "a"), || {
            self.a.ok_or(SynthesisError::AssignmentMissing)
        })
        .unwrap();
        let b: FpVar<ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4>> =
            FpVar::new_witness(ark_relations::ns!(cs, "b"), || {
                self.b.ok_or(SynthesisError::AssignmentMissing)
            })
            .unwrap();
        let product = FpVar::new_input(ark_relations::ns!(cs, "product"), || {
            self.product.ok_or(SynthesisError::AssignmentMissing)
        })
        .unwrap();
        product.enforce_equal(&(&a * &b))?;
        Ok(())
    }
}

fn main() {
    // Generate random parameters for the Groth16 proof system
    let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(ark_std::test_rng().next_u64());

    let circut = FactorizationCircuit {
        a: None,
        b: None,
        product: None,
    };

    // Generate a pk vk
    let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(circut.clone(), &mut rng).unwrap();
    // Prove :
    // Generate random inputs for the factorization circuit
    let a_ = Fr::rand(&mut rng);
    let b_ = Fr::rand(&mut rng);
    let product_ = a_ * b_;

    // Create an instance of the factorization circuit
    let circuit2 = FactorizationCircuit {
        a: Some(a_),
        b: Some(b_),
        product: Some(product_),
    };

    // Create a proof of the factorization circuit
    let proof = Groth16::<Bls12_381>::prove(&pk, circuit2, &mut rng).unwrap();

    // fail here
    let false_product = product_.double();
    // Verify the proof
    let is_valid = Groth16::<Bls12_381>::verify(&vk, &vec![product_], &proof).unwrap();

    assert!(is_valid);

    // Print the proof
    // println!("Proof: {:?}", proof);

    // Add a tracesub function to print nicely formatted information
    fn tracesub<T: std::fmt::Debug>(name: &str, value: T) {
        println!("{}: {:?}", name, value);
    }

    // Example usage of tracesub
    tracesub("a", a_);
    tracesub("b", b_);
    tracesub("product", product_);
}

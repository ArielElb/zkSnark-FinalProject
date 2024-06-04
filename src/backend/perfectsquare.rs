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

use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

// Define the perfect square circuit
#[derive(Clone)]
pub struct PerfectSquareCircuit {
    pub x: Option<Fr>, // The witness (square root)
    pub n: Option<Fr>, // The public input (number to verify)
}

impl ConstraintSynthesizer<Fr> for PerfectSquareCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        let x = FpVar::new_witness(ark_relations::ns!(cs, "x"), 
        || {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let n = FpVar::new_input(ark_relations::ns!(cs, "n"), || {
            self.n.ok_or(SynthesisError::AssignmentMissing)
        })?;
        let x_squared = &x * &x;
        x_squared.enforce_equal(&n)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::test_rng;

    #[test]
    fn test_perfect_square_circuit() {
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());

        let circuit = PerfectSquareCircuit { x: None, n: None };

        // Generate a pk vk
        let (pk, vk) =
            Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

        // Example number and its square root
        let x = Fr::from(5u32); // Square root
        let n = x * x; // Perfect square

        // Create an instance of the perfect square circuit
        let circuit = PerfectSquareCircuit {
            x: Some(x),
            n: Some(n),
        };

        // Create a proof of the perfect square circuit
        let proof = Groth16::<Bls12_381>::prove(&pk, circuit, &mut rng).unwrap();

        // Verify the proof
        let is_valid = Groth16::<Bls12_381>::verify(&vk, &vec![n], &proof).unwrap();
        assert!(is_valid);

        // Add a tracesub function to print nicely formatted information
        fn tracesub<T: std::fmt::Debug>(name: &str, value: T) {
            println!("{}: {:?}", name, value);
        }

        // Example usage of tracesub
        tracesub("Square root (x)", x);
        tracesub("Perfect square (n)", n);
    }
}

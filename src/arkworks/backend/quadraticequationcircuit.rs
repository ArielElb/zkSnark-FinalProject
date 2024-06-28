use ark_bls12_381::{Bls12_381, Fr};
use ark_ff::Field;
use ark_groth16::Groth16;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_snark::{CircuitSpecificSetupSNARK, SNARK};
use ark_std::rand::rngs::StdRng;
use ark_std::rand::SeedableRng;

#[derive(Clone)]
pub struct QuadraticEquationCircuit {
    pub x: Option<Fr>, // The witness (solution)
    pub a: Fr,         // Coefficient a
    pub b: Fr,         // Coefficient b
    pub c: Fr,         // Coefficient c
}

impl ConstraintSynthesizer<Fr> for QuadraticEquationCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Allocate the witness (solution)
        let x = FpVar::new_witness(ark_relations::ns!(cs, "x"), || {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate the public inputs (coefficients)
        let a = FpVar::new_input(ark_relations::ns!(cs, "a"), || Ok(self.a))?;
        let b = FpVar::new_input(ark_relations::ns!(cs, "b"), || Ok(self.b))?;
        let c = FpVar::new_input(ark_relations::ns!(cs, "c"), || Ok(self.c))?;

        // Compute ax^2
        let ax2 = &a * &x * &x;

        // Compute bx
        let bx = &b * &x;

        // Compute ax^2 + bx + c
        let result = ax2 + bx + c;

        // Enforce ax^2 + bx + c = 0
        result.enforce_equal(&FpVar::zero())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::rand::RngCore;
    use ark_std::test_rng;

    #[test]
    fn test_quadratic_equation_circuit() {
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());

        let a = Fr::from(1u32); // Coefficient a
        let b = Fr::from(-3); // Coefficient b
        let c = Fr::from(2u32); // Coefficient c
        let x = Fr::from(1u32); // Solution (one of the roots)

        let circuit = QuadraticEquationCircuit {
            x: Some(x),
            a,
            b,
            c,
        };

        // Generate proving and verifying keys
        let (pk, vk) =
            Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

        // Create a proof
        let proof = Groth16::<Bls12_381>::prove(&pk, circuit, &mut rng).unwrap();

        // Verify the proof
        let is_valid = Groth16::<Bls12_381>::verify(&vk, &[a, b, c], &proof).unwrap();
        assert!(is_valid);

        // Print nicely formatted information
        fn tracesub<T: std::fmt::Debug>(name: &str, value: T) {
            println!("{}: {:?}", name, value);
        }

        // Example usage of tracesub
        tracesub("Coefficient a", a);
        tracesub("Coefficient b", b);
        tracesub("Coefficient c", c);
        tracesub("Solution x", x);
    }
}

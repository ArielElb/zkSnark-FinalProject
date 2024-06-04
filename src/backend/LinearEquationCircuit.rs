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
#[cfg(test)]
mod tests {
    use ark_crypto_primitives::sponge::poseidon::PoseidonSponge;
    use ark_relations::r1cs::ConstraintSystem;
    use blake2::Blake2b512;

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
        circuit.clone().generate_constraints(cs.clone()).unwrap();
        let public_input = ConstraintSystemRef::borrow(&cs)
            .unwrap()
            .instance_assignment
            .clone();
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
        let is_correct = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
        assert!(is_correct);
    }
    // #[test]
    // fn marlin_proof_system() {
    // use super::*;
    // // bls12_381 is a pairing-friendly elliptic curve
    // // Fr is the base field of bls12_381
    // use ark_bls12_381::{Bls12_381, Fr as BlsFr};
    // use ark_crypto_primitives::sponge::poseidon::PoseidonSponge;
    // use ark_marlin::Marlin;
    // use ark_poly::univariate::DensePolynomial;
    // use ark_poly_commit::marlin_pc::MarlinKZG10;
    // use ark_std::{ops::*, UniformRand};
    // use blake2::Blake2s;
    // type MultiPC = MarlinKZG10<Bls12_381, DensePolynomial<Fr>, PoseidonSponge<Fr>>;
    // type MarlinInst = Marlin<BlsFr, MultiPC, Blake2s<Blake2b512>>;
    // let rng = &mut ark_std::test_rng();
    // // calculate the proof by passing witness variable value
    // let universal_srs = MarlinInst::universal_setup(10, 10, 10, rng).unwrap();
    // let x = BlsFr::from(3);
    // let circuit_cubic_instance = CubicDemoCircuit { x: Some(x) };
    // let proof = MarlinInst::prove(&index_pk, circuit_cubic_instance, rng).unwrap();
    // proof.print_size_info();
    // // validate the proof
    // assert!(MarlinInst::verify(&index_vk, &[BlsFr::from(35)], &proof, rng).unwrap());

    // multiply circuit
}
// }

use ark_ff::{ Field, One, PrimeField, Zero};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::{fields::fp::FpVar, R1CSVar};
use ark_r1cs_std::bits::uint32::UInt32;
use ark_snark::SNARK;
use ark_relations::{
    lc,
    r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};


#[derive(Copy, Clone)]
struct PrimeCircut<ConstraintF: PrimeField> {
    x: Option<ConstraintF>, // x is the number to be checked
    num_of_rounds: u64, }



// GENERATE CONSTRAINTS
impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for PrimeCircut<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        let num_of_rounds = self.num_of_rounds;
        let x = FpVar::<ConstraintF>::new_input(cs.clone(), || self.x.ok_or(SynthesisError::AssignmentMissing))?;

        // we want to check of hash(x) is prime
        let mut curr_var: FpVar<ConstraintF> = x.clone();

        




        



        Ok(())

    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Bls12_381, Fr as BlsFr};
    use ark_groth16::Groth16;

    use ark_std::{ops::*, UniformRand};
    use ark_relations::r1cs::ConstraintSynthesizer;
    use ark_relations::r1cs::ConstraintSystem;
    use rand::{rngs::StdRng, SeedableRng};
    #[test]
    fn test_prime_native() {

        let cs = ConstraintSystem::<BlsFr>::new_ref();
        let x = BlsFr::from(12u8);
        // let the number of rounds be 3
        let num_of_rounds = 3;
        let circuit = PrimeCircut { x: Some(x), num_of_rounds };
        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    // #[test]
    // fn test_groth16_circuit() {
    //     let seed = [0u8; 32];
    //     let mut rng = StdRng::from_seed(seed);
    //     let n = BlsFr::from(12u8);
    //     let d = BlsFr::from(3u8);
    //     let s = 2u64;
    //     let circuit = PrimeCircut { n: Some(n), d: Some(d), s, num_constraints: 10, num_variables: 10 };
    //     let circutproof =circuit.clone();        // generate the setup parameters
    //     let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(
    //         circuit,
    //         &mut rng,
    //     )
    //     .unwrap();

    //     // calculate the proof by passing witness variable value
    //     let proof = Groth16::<Bls12_381>::prove(
    //         &pk,
    //         circutproof,
    //         &mut rng,
    //     ).unwrap();
    //     let mut inputs = Vec::new();
    //     inputs.push(n);
    //     let pvk = Groth16::<Bls12_381>::process_vk(&vk).unwrap();
    //     if let Err(_err) = Groth16::<Bls12_381>::verify_with_processed_vk(&pvk, &inputs, &proof) {
    //         eprintln!("Verification failed: your circuit constraints are not satisfied.");
    //         println!("Error: {:?}", _err);
    //     }
    //     else {
    //         eprintln!("Verification sucess: your circuit constraints are  satisfied.");

    //     }
    // }



}




use ark_ff::{ Field, One, PrimeField, Zero};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::{fields::fp::FpVar, R1CSVar};
use  sha2::Sha256;
use ark_ff::BigInteger;
use crate::miller_rabin::miller_rabin_test2;
// use crate::check_hash::hash_checker_fp;
use ark_ff::field_hashers::{DefaultFieldHasher, HashToField};
use ark_snark::SNARK;
use ark_relations::{
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

        // we want to check of hash(x) or hash(x+1) or hash(x+2) or ... hash(x+num_of_rounds) is prime
        let mut curr_var: FpVar<ConstraintF> = x.clone();
        let hasher = <DefaultFieldHasher<Sha256> as HashToField<ConstraintF>>::new(&[]);


        // i want to hash(x) check if x is prime then hash(x+1) and check if hash(x+1) is prime
        for i in 0..num_of_rounds {
            // hash the current value
            let preimage = curr_var.value().unwrap().into_bigint().to_bytes_be(); // Converting to big-endian
            let hashes: Vec<ConstraintF> = hasher.hash_to_field(&preimage, 1); // Returned vector is of size 2
            // take the actual number of the hash[0]
            let hash = hashes[0];
            // print!("hash: {:?}\n", hash);
            // check if hash is prime
            let hash_bigint = hash.into_bigint();
            let is_prime = miller_rabin_test2(hash_bigint.into(), 1);
            if is_prime == true {
                println!("hash is prime: {:?}\n", hash_bigint.into());
                let hash_var = FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(hash))?;
                // check if hash is prime
                hash_var.enforce_equal(&hash_var)?;
            }
            // if hash is prime then hash the next value
            curr_var = curr_var + FpVar::<ConstraintF>::new_input(cs.clone(), || Ok(ConstraintF::one()))?;
        }
    
        Ok(())

    }
}


fn create_pub_input<ConstraintF: PrimeField>(x: ConstraintF, num_of_rounds: u64) -> Vec<ConstraintF> {
    let mut pub_input = Vec::new();
    // add x to the public input
    pub_input.push(x);
    // add hash(x) , hash(x+1), hash(x+2), ... hash(x+num_of_rounds) to the public input:
    let hasher = <DefaultFieldHasher<Sha256> as HashToField<ConstraintF>>::new(&[]);
    let mut curr_var = x;
    for _ in 0..num_of_rounds {
        let preimage = curr_var.into_bigint().to_bytes_be(); // Converting to big-endian
        let hashes: Vec<ConstraintF> = hasher.hash_to_field(&preimage, 1); // Returned vector is of size 2
        let hash = hashes[0];
        pub_input.push(hash);
        curr_var = curr_var + ConstraintF::one();
    }
    pub_input

}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Bls12_381, Fr as BlsFr};
    use ark_groth16::Groth16;

    use ark_std::{ops::*, UniformRand};
    use ark_relations::r1cs::ConstraintSynthesizer;
    use ark_relations::r1cs::ConstraintSystem;
    use num_bigint::ToBigUint;
    use rand::{rngs::StdRng, SeedableRng};
    #[test]
    fn test_prime_native() {
        let cs = ConstraintSystem::<BlsFr>::new_ref();
        let x = BlsFr::from(227u8);
        // let the number of rounds be 3
        let num_of_rounds = 1000;
        let circuit = PrimeCircut { x: Some(x), num_of_rounds };
        circuit.generate_constraints(cs.clone()).unwrap();
        // print the number of constraints
        println!("Number of constraints: {:?}", cs.num_constraints());
        println!("Number of variables: {:?}", cs.num_instance_variables());
        // print the matrix nicely
        cs.finalize();
        // print the matrix nicely
        let matrix = cs.to_matrices().unwrap();
        println!("Matrix A: {:?}", matrix.a);
        println!("Matrix B: {:?}", matrix.b);


        // print the number 
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_groth16_circuit() {
        let seed = [0u8; 32];
        let mut rng = StdRng::from_seed(seed);
        let n = BlsFr::from(3u8);
        let circuit = PrimeCircut { x: Some(n), num_of_rounds: 3 };
        let circutproof =circuit.clone();        // generate the setup parameters
        let (pk, vk) = Groth16::<Bls12_381>::circuit_specific_setup(
            circuit,
            &mut rng,
        )
        .unwrap();

        // calculate the proof by passing witness variable value
        let proof = Groth16::<Bls12_381>::prove(
            &pk,
            circutproof,
            &mut rng,
        ).unwrap();
        let  inputs = create_pub_input(n, 3);
        // add number of roundes + 1 elements to the input vector:

        let pvk = Groth16::<Bls12_381>::process_vk(&vk).unwrap();
        if let Err(_err) = Groth16::<Bls12_381>::verify_with_processed_vk(&pvk, &inputs, &proof) {
            eprintln!("Verification failed: your circuit constraints are not satisfied.");
            println!("Error: {:?}", _err);
        }
        else {
            eprintln!("Verification sucess: your circuit constraints are  satisfied.");

        }
    }



}




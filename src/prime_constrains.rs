use ark_ff::{PrimeField};
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
use ark_groth16::{prepare_verifying_key, Groth16, ProvingKey};


#[derive(Clone)]
struct PrimeCircut<ConstraintF: PrimeField> {
    pub x: Option<ConstraintF>, // x is the number to be checked
    pub num_of_rounds: usize, }




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
        for _ in 0..num_of_rounds {
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
            curr_var = curr_var + ConstraintF::one();
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
        if miller_rabin_test2(hash.into_bigint().into(), 1) == true {
            pub_input.push(hash);
        }
        curr_var = curr_var + ConstraintF::one();
    }
    pub_input

}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Bls12_381, Fr as BlsFr};
    use ark_crypto_primitives::Error;
    use ark_groth16::ProvingKey;
    use ark_relations::r1cs::ConstraintSynthesizer;
    use ark_relations::r1cs::ConstraintSystem;
    use rand::{rngs::StdRng, SeedableRng};
    fn is_proof_satisfied<F: PrimeField, C: ConstraintSynthesizer<F>>(
        circuit: C,
    ) -> Result<bool, Error> {
        let cs = ConstraintSystem::<BlsFr>::new_ref();
        let x = BlsFr::from(227u8);
        // let the number of rounds be 3
        let num_of_rounds = 1000;
        let circuit = PrimeCircut { x: Some(x), num_of_rounds };
        circuit.generate_constraints(cs.clone()).unwrap();
        // print the number of constraints
        println!("Number of constraints: {:?}", cs.num_constraints());
        println!("Number of variables(public inputs): {:?}", cs.num_instance_variables());
        // print the matrix nicely
        cs.finalize();
        // print the matrix nicely
        let matrix = cs.to_matrices().unwrap();
        println!("Matrix A: {:?}", matrix.a);
        println!("Matrix B: {:?}", matrix.b);


        // print the number 
        Ok(cs.is_satisfied().unwrap())
        
    }



    #[test]
    fn verify_proof() -> Result<(), Error> {
        let rng = &mut StdRng::seed_from_u64(0u64);
        let circuit = PrimeCircut {
            x: Some(BlsFr::from(227u8)),
            num_of_rounds: 1000,
        };
        let proving_key: ProvingKey<Bls12_381> =
        // bug here
            Groth16::<Bls12_381>::generate_random_parameters_with_reduction(circuit.clone(), rng)?;

        if !is_proof_satisfied(circuit.clone())? {
            // throw a random error
            return Err(("Err").into());
        }

        let proof =
            Groth16::<Bls12_381>::create_random_proof_with_reduction(circuit, &proving_key, rng)?;

        let pvk = prepare_verifying_key(&proving_key.vk);
        assert!(Groth16::<Bls12_381>::verify_proof(&pvk, &proof, &[],)?);

        Ok(())
    }




}



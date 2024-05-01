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
    r1cs::{ConstraintSystem,ConstraintSynthesizer, ConstraintSystemRef, SynthesisError},
};
use ark_groth16::{prepare_verifying_key, Groth16, ProvingKey};
use ark_bls12_381::{Bls12_381, Fr as BlsFr};
use ark_crypto_primitives::Error;
use rand::{rngs::StdRng, SeedableRng};

#[derive(Clone)]
struct PrimeCircut<ConstraintF: PrimeField> {
    pub x: Option<ConstraintF>, // x is the number to be checked
    pub num_of_rounds: usize,
}




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
                let hash_var = FpVar::<ConstraintF>::new_variable(cs.clone(), || Ok(hash),ark_r1cs_std::alloc::AllocationMode::Witness)?;
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
    println!("pub_input: {:?}", pub_input.len());
    pub_input

}
#[cfg(test)]
mod tests {
    use ark_snark::CircuitSpecificSetupSNARK;

    use super::*;


    #[test]
    fn groth16(){
        use rand::RngCore;
        use ark_std::test_rng;
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
        let circuit = PrimeCircut {
            x: None,
            num_of_rounds: 0,
        };
        let (pk, vk) = Groth16::<Bls12_381>::setup(circuit ,&mut rng).unwrap();
        let  pub_input = vec![BlsFr::from(227u8)];

        let circut2 = PrimeCircut {
            x: Some(BlsFr::from(227u8)),
            num_of_rounds: 1000,
       };
        let proof = Groth16::<Bls12_381>::prove(&pk, circut2, &mut rng).unwrap();
        let proof_valid = Groth16::<Bls12_381>::verify(&vk, &pub_input, &proof).unwrap();
        print!("proof_valid: {:?}\n", proof_valid);
    }


}



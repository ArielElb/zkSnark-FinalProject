use crate::matrix_proof_of_work::alloc::{FpVar2D, FpVarArray};
use ark_bls12_381::fr::Fr;
use ark_crypto_primitives::sponge::constraints::CryptographicSpongeVar as CryptographicSpongeVarTrait;
use ark_crypto_primitives::sponge::poseidon::constraints::PoseidonSpongeVar;
use ark_crypto_primitives::sponge::Absorb;
use ark_crypto_primitives::sponge::{
    poseidon::PoseidonSponge, CryptographicSponge, FieldBasedCryptographicSponge,
};

pub use crate::matrix_proof_of_work::hashing::hashing_utils::poseidon_parameters_for_test;
// use ark_crypto_primitives::{absorb, absorb_gadget};
use ark_ff::PrimeField;
use ark_r1cs_std::ToBytesGadget;
use ark_r1cs_std::{boolean::Boolean, fields::fp::FpVar, R1CSVar};
use ark_relations::r1cs::{ConstraintSystemRef, SynthesisError};
use std::ops::ShrAssign;
// // calculates the hash
pub fn hasher<const N: usize, ConstraintF: PrimeField + Absorb>(
    c: &FpVar2D<N, ConstraintF>,
) -> Result<Vec<Fr>, SynthesisError> {
    let sponge_param: ark_crypto_primitives::sponge::poseidon::PoseidonConfig<_> =
        poseidon_parameters_for_test();
    let mut sponge: PoseidonSponge<Fr> = PoseidonSponge::<Fr>::new(&sponge_param);
    let flattened_matrix = flatten_fpvar(c).unwrap();
    sponge.absorb(&flattened_matrix);
    let hash = sponge.squeeze_native_field_elements(1).to_vec();
    Ok(hash)
}

// Calculate the hash using FpVar2D
pub fn hasher_var<const N: usize, ConstraintF: PrimeField>(
    cs: ConstraintSystemRef<ConstraintF>,
    c: &FpVar2D<N, ConstraintF>,
) -> Result<Vec<FpVar<ConstraintF>>, SynthesisError> {
    let sponge_param = poseidon_parameters_for_test();
    let mut sponge = PoseidonSpongeVar::<ConstraintF>::new(cs, &sponge_param);
    let flattened_matrix = flatten_fpvar2d_var(c)?;
    sponge.absorb(&flattened_matrix)?;
    let hash = sponge.squeeze_field_elements(1)?;
    Ok(hash)
}

// Flatten FpVar2D into a vector of FpVar
pub fn flatten_fpvar2d_var<const N: usize, ConstraintF: PrimeField>(
    c: &FpVar2D<N, ConstraintF>,
) -> Result<Vec<&FpVar<ConstraintF>>, SynthesisError> {
    let mut flattened_matrix = Vec::new();
    for i in 0..N {
        for j in 0..N {
            flattened_matrix.push(&c.0[i][j]);
        }
    }
    Ok(flattened_matrix)
}
pub fn flatten_fpvar<const N: usize, ConstraintF: PrimeField>(
    c: &FpVar2D<N, ConstraintF>,
) -> Result<Vec<ConstraintF>, SynthesisError> {
    let mut flattened_matrix = Vec::new();
    for i in 0..N {
        for j in 0..N {
            let element = &c.0[i][j];
            let element_value = element.value()?;
            flattened_matrix.push(element_value);
        }
    }
    Ok(flattened_matrix)
}

// create tests:
#[cfg_attr(feature = "test_feature1", cfg(test))]
mod tests {
    use super::*;
    use ark_bls12_381::Fr as F;
    use ark_r1cs_std::alloc::AllocVar;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_std::test_rng;
    use rand::RngCore;

    #[test]
    fn hash_matrix_c() {
        let cs = ConstraintSystem::<F>::new_ref();
        let c = [[2, 5], [2, 2]];
        let matrix_c = FpVar2D::new_witness(cs.clone(), || Ok(c)).unwrap();
        let hash = &hasher_var(cs.clone(), &matrix_c).unwrap();
        let hash0 = hash[0].clone();
        println!("Hash: {:?}", hash.value().unwrap());

        // now hash using the non-var hasher:
        let hash2 = hasher(&matrix_c).unwrap();
        println!("Hash2: {:?}", hash2);
        assert_eq!(hash.value().unwrap(), hash2);
    }

    #[test]
    fn test_hashing_full_matrix() {
        let cs = ConstraintSystem::<F>::new_ref();
        let c = [[2, 2, 2], [2, 2, 2], [2, 2, 2]];
        let c_var = FpVar2D::new_witness(cs.clone(), || Ok(c)).unwrap();
        let hash = hasher_var(cs.clone(), &c_var).unwrap();
        assert!(!hash.is_empty());
    }
    #[test]
    fn test_hashing_different_matrices() {
        let cs = ConstraintSystem::<F>::new_ref();
        let c1 = [[2, 2], [2, 2]];
        let c2 = [[2, 3], [2, 2]];
        let c_var1 = FpVar2D::new_witness(cs.clone(), || Ok(c1)).unwrap();
        let c_var2 = FpVar2D::new_witness(cs.clone(), || Ok(c2)).unwrap();
        let hash1 = hasher_var(cs.clone(), &c_var1).unwrap();
        let hash2 = hasher_var(cs.clone(), &c_var2).unwrap();
        assert_ne!(hash1.value().unwrap(), hash2.value().unwrap());
    }
    #[test]
    fn test_hashing_one_changed_element() {
        let cs = ConstraintSystem::<F>::new_ref();
        let c1 = [[2, 2], [2, 2]];
        let mut c2 = c1.clone();
        c2[1][1] = 3; // Change one element
        let c_var1 = FpVar2D::new_witness(cs.clone(), || Ok(c1)).unwrap();
        let c_var2 = FpVar2D::new_witness(cs.clone(), || Ok(c2)).unwrap();
        let hash1 = hasher_var(cs.clone(), &c_var1).unwrap();
        let hash2 = hasher_var(cs.clone(), &c_var2).unwrap();
        assert_ne!(hash1.value().unwrap(), hash2.value().unwrap());
    }
    #[test]
    fn test_hash_field_elements() {
        let cs = ConstraintSystem::<F>::new_ref();
        let c = [[2, 2], [2, 2]];
        let c_var = FpVar2D::new_witness(cs.clone(), || Ok(c)).unwrap();
        let hash = hasher(&c_var).unwrap();
        assert!(!hash.is_empty());
    }
}

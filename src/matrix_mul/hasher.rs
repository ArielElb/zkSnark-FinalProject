use crate::matrix_mul::alloc::{FpVar2D, FpVarArray};
use ark_bls12_381::fr::Fr;
use ark_crypto_primitives::sponge::constraints::CryptographicSpongeVar as CryptographicSpongeVarTrait;
use ark_crypto_primitives::sponge::poseidon::constraints::PoseidonSpongeVar;
use ark_crypto_primitives::sponge::Absorb;
use ark_crypto_primitives::sponge::{
    poseidon::PoseidonSponge, CryptographicSponge, FieldBasedCryptographicSponge,
};

pub use crate::matrix_mul::hashing::hashing_utils::poseidon_parameters_for_test;
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
    let sponge_param = poseidon_parameters_for_test();
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
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::Fr as F;
    use ark_r1cs_std::alloc::AllocVar;
    use ark_relations::r1cs::ConstraintSystem;
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
}

// #[test]
// fn mod_gen_hash_test() {
//     use ark_bls12_381::Fq as F;
//     use ark_r1cs_std::alloc::AllocVar;
//     use ark_relations::r1cs::ConstraintSystem;

//     let adj_matrix = [
//         [false, true, true, false],   //               [0]
//         [false, false, true, false],  //               / \
//         [false, false, false, true],  //             [1]->[2] -> 3
//         [false, false, false, false], //
//     ];

//     let cs = ConstraintSystem::<F>::new_ref();
//     let adj_matrix_var = Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix)).unwrap();

//     let hash1 = hasher(&adj_matrix_var).unwrap();
//     let hash2 = hasher(&adj_matrix_var).unwrap();

//     // Check if hashes are consistent for the same input
//     assert_eq!(hash1, hash2);

//     // Modify the adjacency matrix
//     let adj_matrix_modified = [
//         [true, true, false, false],   //              [0]
//         [false, false, true, false],  //              /  \
//         [false, false, false, true],  //             [1]->[2] -> 3
//         [false, false, false, false], //
//     ];
//     let adj_matrix_var_modified =
//         Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix_modified)).unwrap();
//     let hash_modified = hasher(&adj_matrix_var_modified).unwrap();

//     // Check if hash changes with different input
//     assert_ne!(hash1, hash_modified);
// }

// #[test]
// fn test_hashing_empty_matrix() {
//     use ark_bls12_381::Fq as F;
//     use ark_r1cs_std::alloc::AllocVar;
//     use ark_relations::r1cs::ConstraintSystem;

//     let adj_matrix = [[false; 4]; 4];

//     let cs = ConstraintSystem::<F>::new_ref();
//     let adj_matrix_var = Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix)).unwrap();
//     let hash = hasher(&adj_matrix_var).unwrap();

//     // Ensure hash is not empty or null
//     assert!(!hash.is_empty());
// }

// #[test]
// fn test_hashing_full_matrix() {
//     use ark_bls12_381::Fq as F;
//     use ark_r1cs_std::alloc::AllocVar;
//     use ark_relations::r1cs::ConstraintSystem;

//     let adj_matrix = [[true; 4]; 4];

//     let cs = ConstraintSystem::<F>::new_ref();
//     let adj_matrix_var = Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix)).unwrap();
//     let hash = hasher(&adj_matrix_var).unwrap();

//     // Assert the hash is generated successfully
//     assert!(!hash.is_empty());
// }

// #[test]
// fn test_hashing_different_matrices() {
//     use ark_bls12_381::Fq as F;
//     use ark_r1cs_std::alloc::AllocVar;
//     use ark_relations::r1cs::ConstraintSystem;

//     let adj_matrix_1 = [[false, true], [true, false]];
//     let adj_matrix_2 = [[true, false], [false, true]];

//     let cs = ConstraintSystem::<F>::new_ref();
//     let adj_matrix_var_1 = Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix_1)).unwrap();
//     let adj_matrix_var_2 = Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix_2)).unwrap();

//     let hash1 = hasher(&adj_matrix_var_1).unwrap();
//     let hash2 = hasher(&adj_matrix_var_2).unwrap();

//     assert_ne!(hash1, hash2);
// }

// #[test]
// fn test_hashing_one_changed_element() {
//     use ark_bls12_381::Fq as F;
//     use ark_r1cs_std::alloc::AllocVar;
//     use ark_relations::r1cs::ConstraintSystem;

//     let adj_matrix_1 = [[false; 3]; 3];
//     let mut adj_matrix_2 = adj_matrix_1.clone();
//     adj_matrix_2[1][1] = true; // Change one element

//     let cs = ConstraintSystem::<F>::new_ref();
//     let adj_matrix_var_1 = Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix_1)).unwrap();
//     let adj_matrix_var_2 = Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix_2)).unwrap();

//     let hash1 = hasher(&adj_matrix_var_1).unwrap();
//     let hash2 = hasher(&adj_matrix_var_2).unwrap();

//     assert_ne!(hash1, hash2);
// }

// #[test]
// fn test_hashing_inverted_matrices() {
//     use ark_bls12_381::Fq as F;
//     use ark_r1cs_std::alloc::AllocVar;
//     use ark_relations::r1cs::ConstraintSystem;

//     let adj_matrix = [[true, false], [false, true]];
//     let inverted_matrix = adj_matrix.map(|row| row.map(|elem| !elem));

//     let cs = ConstraintSystem::<F>::new_ref();
//     let adj_matrix_var = Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix)).unwrap();
//     let inverted_matrix_var =
//         Boolean2DArray::new_witness(cs.clone(), || Ok(inverted_matrix)).unwrap();

//     let hash1 = hasher(&adj_matrix_var).unwrap();
//     let hash2 = hasher(&inverted_matrix_var).unwrap();

//     assert_ne!(hash1, hash2);
// }

// #[test]
// fn test_hashing_large_diagonal_matrices() {
//     use ark_bls12_381::Fq as F;
//     use ark_r1cs_std::alloc::AllocVar;
//     use ark_relations::r1cs::ConstraintSystem;

//     const N: usize = 50; // Large size
//     let mut adj_matrix = [[false; N]; N];

//     // Diagonal true values
//     for i in 0..N {
//         adj_matrix[i][i] = true;
//     }

//     let cs = ConstraintSystem::<F>::new_ref();
//     let adj_matrix_var_1 =
//         Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix.clone())).unwrap();
//     let adj_matrix_var_2 =
//         Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix.clone())).unwrap();

//     let hash1 = hasher(&adj_matrix_var_1).unwrap();
//     let hash2 = hasher(&adj_matrix_var_2).unwrap();

//     assert_eq!(hash1, hash2);
// }

// #[test]
// fn test_hashing_large_sparse_matrices() {
//     use ark_bls12_381::Fq as F;
//     use ark_r1cs_std::alloc::AllocVar;
//     use ark_relations::r1cs::ConstraintSystem;

//     const N: usize = 60; // Large size
//     let mut adj_matrix = [[false; N]; N];

//     // Sparse true values
//     for i in (0..N).step_by(10) {
//         for j in (0..N).step_by(15) {
//             adj_matrix[i][j] = true;
//         }
//     }

//     let cs = ConstraintSystem::<F>::new_ref();
//     let adj_matrix_var_1 =
//         Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix.clone())).unwrap();
//     let adj_matrix_var_2 =
//         Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix.clone())).unwrap();

//     let hash1 = hasher(&adj_matrix_var_1).unwrap();
//     let hash2 = hasher(&adj_matrix_var_2).unwrap();

//     assert_eq!(hash1, hash2);
// }

// Test failing because matrix is too large

// #[test]
// fn test_hashing_large_identical_matrices() {
//     use ark_bls12_381::Fq as F;
//     use ark_relations::r1cs::ConstraintSystem;
//     use ark_r1cs_std::alloc::AllocVar;

//     const N: usize = 100; // Large size
//     let mut adj_matrix_1 = [[false; N]; N];
//     let mut adj_matrix_2 = [[false; N]; N];

//     // Initialize both matrices with the same pattern
//     for i in 0..N {
//         for j in 0..N {
//             if i % 2 == 0 && j % 3 == 0 {
//                 adj_matrix_1[i][j] = true;
//                 adj_matrix_2[i][j] = true;
//             }
//         }
//     }

//     let cs = ConstraintSystem::<F>::new_ref();
//     let adj_matrix_var_1 = Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix_1)).unwrap();
//     let adj_matrix_var_2 = Boolean2DArray::new_witness(cs.clone(), || Ok(adj_matrix_2)).unwrap();

//     let hash1 = hasher(&adj_matrix_var_1).unwrap();
//     let hash2 = hasher(&adj_matrix_var_2).unwrap();

//     assert_eq!(hash1, hash2);
// }
//

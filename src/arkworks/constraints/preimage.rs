use ark_crypto_primitives::crh::sha256::constraints::{DigestVar, Sha256Gadget, UnitVar};
use ark_crypto_primitives::crh::sha256::Sha256;
use ark_crypto_primitives::crh::CRHSchemeGadget;
use ark_ff::{BigInt, Fp, PrimeField};
use ark_ff::{BigInteger, BigInteger256};
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;

use ark_r1cs_std::uint8::UInt8;
use ark_r1cs_std::{R1CSVar, ToBytesGadget};
use ark_relations::ns;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, Namespace, SynthesisError};

#[derive(Clone)]
pub struct PreImage<ConstraintF: PrimeField> {
    pub x: Option<ConstraintF>,  // preimage - private input
    pub hash_x: Option<Vec<u8>>, // digest - public input
}

fn to_byte_vars<ConstraintF: PrimeField>(
    cs: impl Into<Namespace<ConstraintF>>,
    data: &[u8],
) -> Vec<UInt8<ConstraintF>> {
    let cs = cs.into().cs();
    UInt8::new_witness_vec(cs, data).unwrap()
}

impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for PreImage<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // Create witness x:
        let x_var = FpVar::<ConstraintF>::new_witness(ark_relations::ns!(cs, "x"), || {
            self.x.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Create parameter unit:
        let unit_var = UnitVar::default();

        // Convert x to bytes
        let x_bytes = x_var.to_bytes()?;
        let x_bytes_u8 = x_bytes
            .iter()
            .map(|byte| byte.value().unwrap())
            .collect::<Vec<u8>>();
        // println!("x_bytes_u8: {:?}", x_bytes_u8);

        // Compute the hash
        let computed_hash =
            <Sha256Gadget<ConstraintF> as CRHSchemeGadget<Sha256, ConstraintF>>::evaluate(
                &unit_var,
                &to_byte_vars(ns!(cs, "input"), &x_bytes_u8),
            )
            .unwrap();
        // println!("computed_hash: {:?}", computed_hash.value().unwrap());

        // Create digest variable from hash_x:
        let hash_x_bytes = self
            .hash_x
            .clone()
            .ok_or(SynthesisError::AssignmentMissing)?;

        let hash_x_var = DigestVar::new_input(ns!(cs, "hash_x"), || Ok(hash_x_bytes))?;

        // Ensure the computed hash equals the provided hash
        computed_hash.enforce_equal(&hash_x_var)?;

        Ok(())
    }
}

// Tests:
#[cfg(test)]
mod test {
    use super::*;
    use ark_bls12_381::Fr;
    use ark_ff::BigInt;
    use ark_relations::r1cs::ConstraintSystem;

    #[test]
    fn correctness_preimage() {
        let cs = ConstraintSystem::<Fr>::new_ref();

        // Generate a known field element and its byte representation
        let x = Fr::from(20);
        let x_var = FpVar::<Fr>::new_witness(ark_relations::ns!(cs, "x"), || Ok(x)).unwrap();
        let x_bytes = x_var.to_bytes().unwrap();
        let x_bytes_u8 = x_bytes
            .iter()
            .map(|byte| byte.value().unwrap())
            .collect::<Vec<u8>>();
        // parmaeter unit
        let unit_var = UnitVar::default();

        // Compute the hash
        let computed_hash = <Sha256Gadget<Fr> as CRHSchemeGadget<Sha256, Fr>>::evaluate(
            &unit_var,
            &to_byte_vars(ns!(cs, "input"), &x_bytes_u8),
        )
        .unwrap();

        // Print the computed hash for verification
        // println!("computed_hash: {:?}", computed_hash);

        // Prepare public inputs for the circuit
        let preimage = PreImage {
            x: Some(x),
            hash_x: Some(computed_hash.value().unwrap().to_vec()), // digest
        };

        // Generate constraints and check satisfaction
        preimage.generate_constraints(cs.clone()).unwrap();
        let is_satisfied = cs.is_satisfied().unwrap();
        assert!(is_satisfied);
    }
    #[test]
    fn soundness_preimage() {
        let cs = ConstraintSystem::<Fr>::new_ref();

        // Generate a random field element and its byte representation
        let x = Fr::from(10);
        let x_var = FpVar::<Fr>::new_witness(ark_relations::ns!(cs, "x"), || Ok(x)).unwrap();
        let x_bytes = x_var.to_bytes().unwrap();
        let x_bytes_u8 = x_bytes
            .iter()
            .map(|byte| byte.value().unwrap())
            .collect::<Vec<u8>>();
        // parmaeter unit
        let unit_var = UnitVar::default();

        // Compute the hash
        let computed_hash = <Sha256Gadget<Fr> as CRHSchemeGadget<Sha256, Fr>>::evaluate(
            &unit_var,
            &to_byte_vars(ns!(cs, "input"), &x_bytes_u8),
        )
        .unwrap();

        println!("computed_hash: {:?}", computed_hash.value().unwrap());
        // print the length of the hash
        println!(
            "computed_hash length: {:?}",
            computed_hash.value().unwrap().len()
        );

        // convert the hash to a number:
        // Convert the hash to a number:
        let hash_bytes = computed_hash.value().unwrap();

        // we got 4 u64s from the hash function because the hash is 32 bytes = 256 bits
        let mut hash_u64 = [0u64; 4];

        for (i, chunk) in hash_bytes.chunks(8).enumerate() {
            let mut array = [0u8; 8];
            array.copy_from_slice(chunk);
            hash_u64[i] = u64::from_le_bytes(array);
        }

        let hash_bigint = BigInt::<4>::new(hash_u64);
        // create BigInteger for hash_bigint
        // let hash_bigint = BigInteger256::from(hash_bigint);
        println!("hash_bigint: {:?}", hash_bigint);
        // Print the computed hash for verification
        // println!("computed_hash: {:?}", computed_hash);

        // Prepare public inputs for the circuit
        let preimage = PreImage {
            x: Some(Fr::from(4)),
            hash_x: Some(computed_hash.value().unwrap().to_vec()), // digest
        };

        // Generate constraints and check satisfaction
        preimage.generate_constraints(cs.clone()).unwrap();
        let is_satisfied = cs.is_satisfied().unwrap();
        // assert equal to false
        assert!(!is_satisfied);
    }
}

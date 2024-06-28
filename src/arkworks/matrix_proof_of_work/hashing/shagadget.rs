use sha2::{Digest, Sha256};

#[cfg(test)]
mod tests {
    use ark_bls12_381::Fr as BlsFr;
    use ark_crypto_primitives::crh::{
        sha256::constraints::{Sha256Gadget, UnitVar},
        CRHScheme, CRHSchemeGadget,
    };
    use ark_r1cs_std::{alloc::AllocVar, fields::fp::FpVar, uint8::UInt8, R1CSVar, ToBytesGadget};
    use ark_relations::{
        ns,
        r1cs::{ConstraintSystem, ConstraintSystemRef, Namespace},
    };
    use rand::RngCore;
    const TEST_LENGTHS: &[usize] = &[
        0, 1, 2, 8, 20, 40, 55, 56, 57, 63, 64, 65, 90, 100, 127, 128, 129,
    ];

    use super::*;
    /// Witnesses bytes
    fn to_byte_vars(cs: impl Into<Namespace<BlsFr>>, data: &[u8]) -> Vec<UInt8<BlsFr>> {
        let cs = cs.into().cs();
        UInt8::new_witness_vec(cs, data).unwrap()
    }
    #[test]
    fn test_sha256_gadget() {
        let cs = ark_relations::r1cs::ConstraintSystem::new_ref();
        // Create a new instance of the gadget
        let mut sha256_var = Sha256Gadget::default();

        // Update the gadget with some input
        let curr_var = FpVar::<BlsFr>::new_witness(ark_relations::ns!(cs, "sha256 gadget"), || {
            Ok(BlsFr::from(1))
        })
        .unwrap();
        sha256_var
            .update(&ToBytesGadget::to_bytes(&curr_var).unwrap())
            .unwrap();

        // Finalize the gadget
        let digest_var = sha256_var.finalize();

        let res = digest_var.unwrap();
    }

    /// Tests the CRHCheme trait
    #[test]
    fn crh() {
        let mut rng = ark_std::test_rng();
        let cs = ConstraintSystem::<BlsFr>::new_ref();
        // CRH parameters are nothing
        let unit = ();
        let unit_var = UnitVar::default();

        for &len in TEST_LENGTHS {
            // Make a random string of the given length
            let mut input_str = vec![0u8; len];
            rng.fill_bytes(&mut input_str);

            // Compute the hashes and assert consistency
            let computed_output =
                <Sha256Gadget<BlsFr> as CRHSchemeGadget<Sha256, BlsFr>>::evaluate(
                    &unit_var,
                    &to_byte_vars(ns!(cs, "input"), &input_str),
                )
                .unwrap();

            println!(
                "Computed output: {:?}",
                computed_output.value().unwrap().to_vec()
            );
            let expected_output = <Sha256 as CRHScheme>::evaluate(&unit, input_str).unwrap();
            assert_eq!(
                computed_output.value().unwrap().to_vec(),
                expected_output,
                "CRH error at length {}",
                len
            )
        }
    }
}

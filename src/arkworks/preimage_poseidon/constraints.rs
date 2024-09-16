use super::hash_parm::poseidon_parameters_for_test;
use ark_bls12_381::Fr;
use ark_crypto_primitives::sponge::constraints::CryptographicSpongeVar;
use ark_crypto_primitives::sponge::poseidon::constraints::PoseidonSpongeVar;
use ark_crypto_primitives::sponge::poseidon::{PoseidonConfig, PoseidonSponge};
use ark_crypto_primitives::sponge::{Absorb, CryptographicSponge};
use ark_ff::PrimeField;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::prelude::*;
use ark_r1cs_std::uint8::UInt8;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, Namespace, SynthesisError};

#[derive(Clone)]
pub struct HashCircuit<F: PrimeField> {
    pub message: Vec<UInt8<F>>,  // Input message as bytes (Vec<u8>)
    pub expected_hash: FpVar<F>, // Expected hash output (y)
}

impl<F: PrimeField> HashCircuit<F> {
    fn new(message: Vec<UInt8<F>>, expected_hash: FpVar<F>) -> Self {
        Self {
            message,
            expected_hash,
        }
    }
}
/// Witnesses bytes
pub fn to_byte_vars(cs: impl Into<Namespace<Fr>>, data: &[u8]) -> Vec<UInt8<Fr>> {
    let cs = cs.into().cs();
    UInt8::new_witness_vec(cs, data).unwrap()
}
// now do hasher Var that get a string and return a hash:
// hasher function for string (in the circuit)
pub fn hasher_string_var<ConstraintF: PrimeField>(
    cs: ConstraintSystemRef<ConstraintF>,
    message: Vec<UInt8<ConstraintF>>,
) -> Result<Vec<FpVar<ConstraintF>>, SynthesisError> {
    let sponge_param = poseidon_parameters_for_test();
    let mut sponge = PoseidonSpongeVar::<ConstraintF>::new(cs.clone(), &sponge_param);
    // Absorb the message:
    sponge.absorb(&message)?;
    // Squeeze the field elements to get the hash
    let hash = sponge.squeeze_field_elements(1)?;
    Ok(hash)
}

// generate the constraints for the hash function:
impl<F: PrimeField> ConstraintSynthesizer<F> for HashCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let HashCircuit {
            message,
            expected_hash,
        } = self;

        // hash the message with the Poseidon sponge gadget using hasher_string_var:
        let hash = hasher_string_var(cs.clone(), message)?;
        println!("hash: {:?}", hash[0].value().unwrap());
        hash[0].enforce_equal(&expected_hash)
    }
}
// do hasher that get a string and return a hash in the native field:
pub fn hasher_string_native<ConstraintF: PrimeField + Absorb>(
    c: &Vec<u8>,
) -> Result<Vec<Fr>, SynthesisError> {
    let sponge_param: PoseidonConfig<_> = poseidon_parameters_for_test();
    let mut sponge: PoseidonSponge<Fr> = PoseidonSponge::<Fr>::new(&sponge_param);
    sponge.absorb(&c);
    let hash = sponge.squeeze_field_elements(1).to_vec();
    Ok(hash)
}

// write tests for the r1cs:
#[cfg(test)]
mod test {
    use super::*;
    use ark_bls12_381::Fr as F;

    use ark_relations::r1cs::ConstraintSystem;
    use ark_std::test_rng;
    #[test]
    fn test_hash_circuit() {
        let message = b"hello world";
        let message_bytes = message.to_vec();

        let expected_hash = hasher_string_native::<F>(&message_bytes).unwrap();

        let cs = ConstraintSystem::<F>::new_ref();
        // message witness
        let message_bytes2: Vec<UInt8<F>> = to_byte_vars(cs.clone(), message);

        // y - expected hash as public input
        let expected_hash = FpVar::<F>::new_input(cs.clone(), || Ok(expected_hash[0])).unwrap();

        let circuit = HashCircuit::new(message_bytes2, expected_hash);
        circuit.clone().generate_constraints(cs.clone()).unwrap();
        println!("Number of constraints: {}", cs.num_constraints());
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_hash_native_vs_var() {
        let rng = &mut test_rng();
        let message = b"hello world";
        let message_bytes = message.to_vec();
        let expected_hash_native = hasher_string_native::<F>(&message_bytes).unwrap();

        let cs = ConstraintSystem::<F>::new_ref();
        let message_bytes = to_byte_vars(cs.clone(), message);
        let expected_hash_var = hasher_string_var(cs.clone(), message_bytes).unwrap();

        assert_eq!(
            expected_hash_native[0],
            expected_hash_var[0].value().unwrap()
        );
    }
}

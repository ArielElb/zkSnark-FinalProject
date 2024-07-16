use ark_bls12_381::Fr;
use ark_crypto_primitives::crh::sha256::constraints::{DigestVar, Sha256Gadget};
use ark_ff::BigInteger;
use ark_ff::PrimeField;
use ark_r1cs_std::alloc::AllocVar;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::fields::FieldVar;
use ark_r1cs_std::R1CSVar;
use ark_r1cs_std::ToBytesGadget;
use ark_relations::r1cs::ConstraintSystemRef;
use num_bigint::BigUint;
use sha2::Digest;
use sha2::Sha256;
const K: usize = 10;
/// Finalizes a native SHA256 struct and gets the bytes
pub fn finalize(sha256: Sha256) -> Vec<u8> {
    sha256.finalize().to_vec()
}
// hash to bytes:
pub fn hash_to_bytes<ConstraintF: PrimeField>(
    x_plus_j: FpVar<ConstraintF>,
) -> DigestVar<ConstraintF> {
    let mut sha256_var = Sha256Gadget::default();
    // convert x_plus_j to bytes:
    let x_plus_j_bytes = x_plus_j.to_bytes().unwrap();
    // calculate the hash(x+j):
    sha256_var.update(&x_plus_j_bytes).unwrap();
    let result = sha256_var.finalize().unwrap();
    result
}
pub fn generate_bases_native(x: BigUint) -> Vec<BigUint> {
    let mut a_j_s = vec![];
    for j in 0..K {
        let mut sha256 = Sha256::default();
        let x_fr = Fr::from(x.clone());
        let j_fr: ark_ff::Fp<ark_ff::MontBackend<ark_bls12_381::FrConfig, 4>, 4> =
            Fr::from(j as u64);

        let x_bytes = x_fr.into_bigint().to_bytes_le();
        let j_bytes = j_fr.into_bigint().to_bytes_le();
        // do the hash for x || j
        sha256.update(&x_bytes);
        sha256.update(&j_bytes);
        let a_j = finalize(sha256.clone()); // hash(x || j)
                                            // convert a_j to BigUint:
        let a_j = BigUint::from_bytes_le(&a_j);

        a_j_s.push(a_j);
    }
    a_j_s
}
pub fn generate_bases_a<ConstraintF: PrimeField>(
    cs: ConstraintSystemRef<ConstraintF>,
    r: FpVar<ConstraintF>,
) -> Vec<FpVar<ConstraintF>> {
    let mut a_j_s = vec![];
    for j in 0..K {
        let mut sha256_var = Sha256Gadget::default();
        let r = r.to_bytes().unwrap();
        let j_bytes = FpVar::<ConstraintF>::constant(ConstraintF::from(j as u64))
            .to_bytes()
            .unwrap();
        sha256_var.update(&r).unwrap();
        sha256_var.update(&j_bytes).unwrap();
        let result: DigestVar<ConstraintF> = sha256_var.finalize().unwrap(); // a_i = hash(r || j)
        let a_j_fpvar =
            FpVar::<ConstraintF>::new_witness(ark_relations::ns!(cs, "r_j_fpvar"), || {
                Ok(ConstraintF::from_le_bytes_mod_order(
                    &result.to_bytes().unwrap().value().unwrap(),
                ))
            })
            .unwrap();
        a_j_s.push(a_j_fpvar);
    }
    a_j_s
}

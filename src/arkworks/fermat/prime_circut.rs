use crate::arkworks::fermat::modpow_circut::{mod_witnesses, modpow_ver_circuit};
use ark_bls12_381::{Bls12_381, Fr};
use ark_crypto_primitives::crh::sha256::constraints::DigestVar;
use ark_crypto_primitives::crh::sha256::constraints::Sha256Gadget;
use ark_ff::BigInteger;
use ark_ff::{Field, PrimeField};
use ark_r1cs_std::boolean::Boolean;
use ark_r1cs_std::eq::EqGadget;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::select::CondSelectGadget;
use ark_r1cs_std::uint8::UInt8;
use ark_r1cs_std::R1CSVar;
use ark_r1cs_std::ToBitsGadget;
use ark_r1cs_std::ToBytesGadget;
use ark_r1cs_std::{alloc::AllocVar, fields::FieldVar};
use ark_relations::ns;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use modulo::{mod_pow_generate_witnesses, mod_vals, return_struct};
use num_bigint::{BigUint, ToBigInt, ToBigUint};
use rand::rngs::StdRng;
use rand::SeedableRng;
const NUM_BITS: usize = 381;
const K: usize = 10;
use super::constraints::{fermat_circuit, fermat_constructor};
use super::modulo;
use super::shatry;
use num_bigint::RandBigInt;
// struct for Final circuit: PrimeCheck:
#[derive(Clone)]
pub struct PrimeCheck<ConstraintF: PrimeField> {
    x: ConstraintF,      // a seed for the initial hash // public input
    i: u64,              // the index i s.t we check if a_i=hash(x+i) is prime // public input
    r: ConstraintF,      // randomness // public input - r = hash(x + i || a_i = hash(x+i) || i )
    a_j_s: Vec<Vec<u8>>, // a vector of a_j = hash(x+j) for j in 0..i -1 // public input - to check that we actually calculated the hash correctly
    a_i: Vec<u8>,        // a_i = hash(x+i) // public input
    is_prime: bool,      // witness if the number is prime
    // modpow_ver_circuit: Vec<modpow_ver_circuit<ConstraintF>>, // vector of modpow circuits for each modpow.
    fermat_circuit: fermat_circuit<ConstraintF>,
}

fn hash_using_fpvar<ConstraintF: PrimeField>(
    x_var: FpVar<ConstraintF>,
    i_var: FpVar<ConstraintF>,
) -> Vec<u8> {
    let mut sha256_var = Sha256Gadget::default();
    // compute x+i:
    let x_plus_i = x_var + i_var;
    // convert x_plus_i to bytes:
    let x_plus_i_bytes = x_plus_i.to_bytes().unwrap();
    // calculate the hash(x+i):
    sha256_var.update(&x_plus_i_bytes).unwrap();
    let calculated_a_i: DigestVar<ConstraintF> = sha256_var.finalize().unwrap();
    calculated_a_i.value().unwrap().to_vec()
}
// implement the constraints for the circuit:
impl<ConstraintF: PrimeField> ConstraintSynthesizer<ConstraintF> for PrimeCheck<ConstraintF> {
    fn generate_constraints(
        self,
        cs: ConstraintSystemRef<ConstraintF>,
    ) -> Result<(), SynthesisError> {
        // create the public inputs:
        let x_var = FpVar::<ConstraintF>::new_input(ark_relations::ns!(cs, "x"), || Ok(self.x))?;
        let i_var = FpVar::<ConstraintF>::new_input(ark_relations::ns!(cs, "i"), || {
            Ok(ConstraintF::from(self.i))
        })?;
        let r_var = FpVar::<ConstraintF>::new_witness(ark_relations::ns!(cs, "r"), || Ok(self.r))?;
        // create the witness:
        let is_prime_var =
            Boolean::new_witness(ark_relations::ns!(cs, "is_prime"), || Ok(self.is_prime))?;
        // for each j in 0..i-1:
        for j in 0..self.i {
            let mut sha256_var = Sha256Gadget::default();
            // compute x+j:
            let x_plus_j = x_var.clone() + FpVar::<ConstraintF>::constant(ConstraintF::from(j)); // x+j
                                                                                                 // convert x_plus_j to bytes:
            let x_plus_j_bytes = x_plus_j.to_bytes().unwrap(); // x+j as bytes to le
                                                               // calculate the hash(x+j):
            sha256_var.update(&x_plus_j_bytes).unwrap();
            let calculated_a_j = sha256_var.clone().finalize().unwrap(); // hash(x+j)
                                                                         // enforce that a_j = hash(x+j):
            let a_j_var = DigestVar::new_input(ark_relations::ns!(cs, "a_j"), || {
                Ok(self.a_j_s[j as usize].clone())
            })?;
            a_j_var.enforce_equal(&calculated_a_j)?;
        }
        let mut sha256_var = Sha256Gadget::default();
        // compute x+i:
        let x_plus_i = x_var.clone() + FpVar::<ConstraintF>::constant(ConstraintF::from(self.i));
        // convert x_plus_i to bytes:
        let x_plus_i_bytes = x_plus_i.to_bytes().unwrap();

        // calculate the hash(x+i):
        sha256_var.update(&x_plus_i_bytes).unwrap();
        let calculated_a_i: DigestVar<ConstraintF> = sha256_var.finalize().unwrap();
        // enforce that a_i = hash(x+i):
        let a_i_var = DigestVar::new_input(ark_relations::ns!(cs, "a_i"), || Ok(self.a_i))?;
        a_i_var.enforce_equal(&calculated_a_i)?;

        // TODO: fermat primality
        // TODO : validate that what i calculated is what in fermat_circuit.

        let a_i_fpvar =
            FpVar::<ConstraintF>::new_witness(ark_relations::ns!(cs, "a_i_fpvar"), || {
                Ok(ConstraintF::from_le_bytes_mod_order(
                    &a_i_var.to_bytes().unwrap().value().unwrap(),
                ))
            })?;
        // create a witness from fermat_circuit:
        let randomness_var_fermat = FpVar::<ConstraintF>::new_witness(
            ark_relations::ns!(cs, "randomness_var_fermat"),
            || Ok(self.fermat_circuit.a),
        )?;
        let n_var_fermat =
            FpVar::<ConstraintF>::new_witness(ark_relations::ns!(cs, "n_var_fermat"), || {
                Ok(self.fermat_circuit.n)
            })?;

        let is_prime_var_fermat =
            Boolean::new_witness(ark_relations::ns!(cs, "is_prime_var_fermat"), || {
                Ok(self.fermat_circuit.is_prime)
            })?;

        // // enforce that the randomness is the same:
        randomness_var_fermat.enforce_equal(&r_var)?;
        // // enforce that the n is the same:
        n_var_fermat.enforce_equal(&a_i_fpvar)?;
        // // enforce that the is_prime is the same:
        is_prime_var_fermat.enforce_equal(&is_prime_var)?;
        // In the end create the constraints for the fermat circuit:
        self.fermat_circuit
            .generate_constraints(cs.clone())
            .unwrap();

        Ok(())
    }
}
fn is_prime(n: BigUint, r: [u8; 32]) -> bool {
    let mut rng: StdRng = rand::SeedableRng::from_seed(r);
    // now run for K times:
    for _ in 0..K {
        let a = rng.gen_biguint_range(&BigUint::from(2u64), &n);
        if a.modpow(&(&n - 1u32), &n) != BigUint::from(1u32) {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Bls12_381, Fr};
    use ark_ff::{BigInt, BigInteger};
    use ark_ff::{Field, PrimeField};

    use ark_relations::r1cs::{
        ConstraintSynthesizer, ConstraintSystem, ConstraintSystemRef, SynthesisError,
    };
    use ark_snark::SNARK;
    use ark_std::test_rng;
    use itertools::Itertools;
    use rand::RngCore;
    use sha2::{Digest, Sha256};

    use ark_groth16::Groth16;

    /// Finalizes a native SHA256 struct and gets the bytes
    fn finalize(sha256: Sha256) -> Vec<u8> {
        sha256.finalize().to_vec()
    }

    #[test]
    fn initial_procces() {
        let mut rng = ark_std::test_rng();
        let cs = ConstraintSystem::<Fr>::new_ref();
        let mut x = Fr::from(5u64);
        let mut r_bytes = [0u8; 32];
        rng.fill_bytes(&mut r_bytes);
        let a_i = [0u8; 32];
        let i: u64 = 2;
        // set it up using sha256 default:
        // create for each j in 0..i-1 the hash(x+j):
        let mut a_j_s = vec![];
        for j in 0..i {
            let mut sha256 = Sha256::default();
            let x_plus_j = x + Fr::from(j);
            let x_plus_j_bytes = x_plus_j.into_bigint().to_bytes_le();
            // do the hash for x+j:
            sha256.update(&x_plus_j_bytes);
            let a_j = finalize(sha256.clone());
            a_j_s.push(a_j);
        }
        let mut sha256 = Sha256::default();
        let x_plus_i = x + Fr::from(i);
        let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
        // do the hash for x+i:
        sha256.update(&x_plus_i_bytes);
        let a_i = finalize(sha256.clone());
        // convert a_i to biguint:
        let a_i_biguint: BigUint = BigUint::from_bytes_le(&a_i);

        // r = hash(x + i || a_i = hash(x+i) || i )
        sha256.update(&x_plus_i_bytes);
        sha256.update(&a_i);
        sha256.update(&i.to_le_bytes());
        let r = finalize(sha256.clone());

        // take the 32 u8 from r:
        for (i, byte) in r.iter().enumerate() {
            r_bytes[i] = *byte;
        }
        // convert r to Fr:
        let r = Fr::from_le_bytes_mod_order(&r_bytes);
        // create fermat circuit:
        let fermat_circuit = fermat_constructor::<Fr>(BigUint::from(r), a_i_biguint.clone());
        // create the circuit:
        let circuit = PrimeCheck {
            x,
            i,
            r,
            a_j_s: a_j_s.clone(),
            a_i,
            is_prime: is_prime(a_i_biguint, r_bytes),
            fermat_circuit,
        };
        circuit.generate_constraints(cs.clone()).unwrap();
        // check if the circuit is satisfied:
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn groth16() {
        let mut rng = ark_std::test_rng();
        let cs = ConstraintSystem::<Fr>::new_ref();
        let mut x = Fr::from(5u64);
        let mut r_bytes = [0u8; 32];
        rng.fill_bytes(&mut r_bytes);
        let a_i = [0u8; 32];
        let i: u64 = 2;
        // set it up using sha256 default:
        // create for each j in 0..i-1 the hash(x+j):
        let mut a_j_s = vec![];
        for j in 0..i {
            let mut sha256 = Sha256::default();
            let x_plus_j = x + Fr::from(j);
            let x_plus_j_bytes = x_plus_j.into_bigint().to_bytes_le();
            // do the hash for x+j:
            sha256.update(&x_plus_j_bytes);
            let a_j = finalize(sha256.clone());
            a_j_s.push(a_j);
        }
        let mut sha256 = Sha256::default();
        let x_plus_i = x + Fr::from(i);
        let x_plus_i_bytes = x_plus_i.into_bigint().to_bytes_le();
        // do the hash for x+i:
        sha256.update(&x_plus_i_bytes);
        let a_i = finalize(sha256.clone());
        // convert a_i to biguint:
        let a_i_biguint: BigUint = BigUint::from_bytes_le(&a_i);
        // r = hash(x + i || a_i = hash(x+i) || i )
        sha256.update(&x_plus_i_bytes);
        sha256.update(&a_i);
        sha256.update(&i.to_le_bytes());
        let r = finalize(sha256.clone());
        // take the 32 u8 from r:
        for (i, byte) in r.iter().enumerate() {
            r_bytes[i] = *byte;
        }
        // convert r to Fr:
        let r = Fr::from_le_bytes_mod_order(&r_bytes);
        // create fermat circuit:
        let fermat_circuit = fermat_constructor::<Fr>(BigUint::from(r), a_i_biguint.clone());
        // create the circuit:
        let circuit = PrimeCheck {
            x,
            i,
            r,
            a_j_s: a_j_s.clone(),
            a_i,
            is_prime: is_prime(a_i_biguint, r_bytes),
            fermat_circuit,
        };
        // rng:
        let mut rng = ark_std::rand::rngs::StdRng::seed_from_u64(test_rng().next_u64());
        // setup the groth16:
        let (pk, vk) =
            Groth16::<Bls12_381>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();
        // create the proof:
        let proof = Groth16::<Bls12_381>::prove(&pk, circuit.clone(), &mut rng).unwrap();
        // create the public input:
        let cs_too: ConstraintSystemRef<Fr> = ConstraintSystem::new_ref();
        circuit.generate_constraints(cs_too.clone()).unwrap();
        let public_input = ConstraintSystemRef::borrow(&cs_too)
            .unwrap()
            .instance_assignment
            .clone();
        // print the public inpus one by one nicely:
        for (i, input) in public_input.iter().enumerate() {
            println!("public_input[{}]: {:?}", i, input);
        }
        let is_correct = Groth16::<Bls12_381>::verify(&vk, &public_input[1..], &proof).unwrap();
        print!("is_correct: {:?}", is_correct);
    }
}

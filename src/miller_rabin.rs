use std::ops::Div;
use ark_ff::{PrimeField};   
use ark_r1cs_std::fields::fp::FpVar;
use ark_bls12_381::Fq as F;
use num_bigint::{BigUint, RandBigInt, ToBigUint};
use num_integer::Integer;

pub struct PrimeCircut<ConstraintF: PrimeField>{
    n: FpVar<ConstraintF>,
    k: usize,
}
//<ConstraintF: PrimeField>
pub fn miller_rabin_test2(n: u64, k: usize) -> bool{
    if n == 2{
        return true;
    }
    if n%2== 0 {
        return false;
    }
    let modulus = <F as PrimeField>::MODULUS;
    // convert n to a biguint
    let n_bigint = n.to_biguint().unwrap();
    // convert n_bigint to a biguint:
    let one: BigUint = 1.to_biguint().unwrap();

    let n_minus_one = n_bigint.clone() - one.clone();
    let mut rng = rand::thread_rng();
    let mut s = 0;
    let two = 2.to_biguint().unwrap();
    let mut zero = 0.to_biguint().unwrap();


    let mut d = n_bigint.clone() - one.clone();
    while d.is_even(){
        d = d.div(2.to_biguint().unwrap());
        s = s+1;
    }
    let mut y = 1.to_biguint().unwrap();
    let ubound = n_bigint.clone()-&2.to_biguint().unwrap();
    let lbound = &2.to_biguint().unwrap();
    for _ in 0..128{
        let mut rng = rand::thread_rng();
        let a = rng.gen_biguint_range(&lbound, &ubound);
        let mut x = a.modpow(&d, &n_bigint);
        for _j in 0..s{
            y = x.modpow(&two, &n_bigint);
            println!("x after: {:?}", x);
            println!("y after: {:?}", y);
            if one == y && x != one && x != n_minus_one{
                return false;
            }
            x = y.clone();
        }
        if y != one{
            return false;
        }
    }
    return true;
}//

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bls12_381::{Bls12_381, Fr as BlsFr};
    use ark_poly::univariate::DensePolynomial;
    use ark_poly_commit::marlin_pc::MarlinKZG10;
    use ark_std::{ops::*, UniformRand};
    use ark_relations::r1cs::ConstraintSynthesizer;
    use ark_relations::r1cs::ConstraintSystem;
    use rand::{rngs::StdRng, SeedableRng};
    use rand_chacha::ChaChaCore;
    #[test]
    fn test_prime_native() {
        assert!(!miller_rabin_test2(10, 1));
        assert!(miller_rabin_test2(7, 1));
        assert!(miller_rabin_test2(11, 1));
        assert!(!miller_rabin_test2(15, 1));
        assert!(!miller_rabin_test2(21, 1))
    }

}
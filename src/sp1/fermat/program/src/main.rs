#![no_main]
sp1_zkvm::entrypoint!(main);

type PublicValuesTuple = sol! {
    tuple(uint32, bool)
};
use alloy_sol_types::sol;
use alloy_sol_types::SolType;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sha2::{Digest, Sha256};
pub fn main() {
    let seed = sp1_zkvm::io::read::<[u8; 8]>();
    let num_of_rounds = sp1_zkvm::io::read::<u32>();
    let mut n = sp1_zkvm::io::read::<u32>();
    let mut is_primebool = false;
    let mut hashed: u32 = 0;
    let mask: u32 = 1;
    // copy the seed to a new array of 32 bytes:
    let mut seed_arr = [0u8; 32];
    for i in 0..8 {
        seed_arr[i] = seed[i];
    }
    for _ in 0..num_of_rounds {
        if is_primebool {
            break;
        }
        hashed = hash(n) | mask;

        is_primebool = fermat_test(hashed, seed_arr, 20);
        n += 1;
    }
    let bytes = PublicValuesTuple::abi_encode(&(hashed, is_primebool));
    sp1_zkvm::io::commit_slice(&bytes);
}
fn hash(n: u32) -> u32 {
    let mut hasher = Sha256::new();
    hasher.update(n.to_be_bytes());
    let result = hasher.finalize();
    let mut res = [0u8; 4];
    res.copy_from_slice(&result[..4]);
    u32::from_be_bytes(res)
}

fn fermat_test(n: u32, seed: [u8; 32], k: u32) -> bool {
    if n == 1 || n == 4 {
        return false;
    }
    if n <= 3 {
        return true;
    }

    let mut rng: StdRng = SeedableRng::from_seed(seed);

    for _ in 0..k {
        let x = rng.gen_range(2..n - 2);

        if mod_exp(x, n - 1, n) != 1 {
            return false;
        }
    }

    true
}

fn mod_exp(base: u32, exponent: u32, modulus: u32) -> u32 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;

    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }

    result
}

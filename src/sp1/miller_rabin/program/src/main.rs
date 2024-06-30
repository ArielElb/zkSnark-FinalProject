//! A simple program that takes a number n  and hash(X) ,... Hash(X+number_of_rounds) and check if any of the hashed number is prime.
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use alloy_sol_types::{sol, SolType};
use rand::Rng;
use sha2::{Digest, Sha256};
/// The public values encoded as a tuple that can be easily deserialized inside Solidity.
type PublicValuesTuple = sol! {
    tuple(uint32, uint32, uint32,bool)
};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let mut n = sp1_zkvm::io::read::<u32>();
    let num_of_rounds = sp1_zkvm::io::read::<u32>();

    if num_of_rounds > 50 {
        panic!(
            "This program is not designed to handle more than 50 rounds. You requested {} rounds.",
            num_of_rounds
        );
    }

    let mut is_primebool = false;
    let mut hashed: u32 = 0;
    let mask: u32 = 1;
    // do a for loop : hash(x),check if prime, if prime return true o.w continue to hash(x+1):
    for i in 0..num_of_rounds {
        if is_primebool {
            break;
        }
        println!("cycle-tracker-start: hashing ");
        // Genearte a random number between 0 and 2^64:
        hashed = hash(n) | mask;
        println!("hashed: {}", hashed);
        println!("cycle-tracker-start: hashing ");
        // is_primebool = probabilistic_miller_rabin(hashed, 20);
        is_primebool = is_prime1(hashed);
        n += 1;
    }

    // Encocde the public values of the program.
    let bytes = PublicValuesTuple::abi_encode(&(n, num_of_rounds, hashed, is_primebool));

    // Commit to the public values of the program.
    sp1_zkvm::io::commit_slice(&bytes);
}
// Returns if divisible via immediate checks than 6k ± 1.
// Source: https://en.wikipedia.org/wiki/Primality_test#Rust
// #[sp1_derive::cycle_tracker]
fn is_prime1(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    // Check if n is divisible by 6k ± 1 up to sqrt(n)
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}
// implement probabilistic primality test using Miller-Rabin algorithm.
// Source: https://en.wikipedia.org/wiki/Miller%E2%80%93Rabin_primality_test
#[sp1_derive::cycle_tracker]
fn probabilistic_miller_rabin(n: u32, k: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }
    let mut rng = rand::thread_rng();
    for _ in 0..k {
        let a = rng.gen_range(2..n - 1);
        let mut x = a.pow(d) % n;
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..s - 1 {
            x = x.pow(2) % n;
            if x == 1 {
                return false;
            }
            if x == n - 1 {
                break;
            }
        }
        if x != n - 1 {
            return false;
        }
    }
    true
}

// hash :
#[sp1_derive::cycle_tracker]
fn hash(seed: u32) -> u32 {
    let mut hasher = Sha256::new();
    hasher.update(seed.to_be_bytes());
    let hash = hasher.finalize();
    // Convert the hash to a u32
    let mut result = 0u32;
    for i in 0..4 {
        result |= (hash[i] as u32) << (i * 8);
    }
    result
}

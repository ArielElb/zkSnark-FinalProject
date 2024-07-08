//! A simple program that takes a number `p` and a base `a` as input, and writes whether `p` passes
//! the Fermat test for base `a`.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::{sol, SolType};
use sp1_zkvm::io::{commit_slice, read};

/// The public values encoded as a tuple that can be easily deserialized inside Solidity.
type PublicValuesTuple = sol! {
    tuple(uint32, bool)
};

pub fn main() {
    // Read inputs to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let p = read::<u32>();
    let a = read::<u32>();

    // Check if p is 2 (the smallest prime number).
    let is_prime = if p == 2 {
        true
    } else if p < 2 || p % 2 == 0 {
        false
    } else {
        // Perform Fermat's little theorem test: a^(p-1) ≡ 1 (mod p)
        let mut result = 1u32;
        let mut base = a % p;
        let mut exponent = p - 1;

        while exponent > 0 {
            if exponent % 2 == 1 {
                result = (result * base) % p;
            }
            base = (base * base) % p;
            exponent /= 2;
        }

        result == 1
    };

    // Encode the public values of the program.
    let bytes = PublicValuesTuple::abi_encode(&(p, is_prime));

    // Commit to the public values of the program.
    commit_slice(&bytes);
}

# Applications of zk-SNARKs

This project presents various applications of zk-SNARKs (Zero-Knowledge Succinct Non-Interactive Arguments of Knowledge) developed using the ArkWorks library in Rust, with a Next.js frontend for interaction. The project showcases cryptographic proof applications such as Fibonacci sequence computation, matrix multiplication verification, and primality testing with Fermat’s algorithm.

## Table of Contents
- [Overview](#overview)
- [What are zk-SNARKs?](#what-are-zk-snarks)
- [zk-SNARKs vs zk-STARKs](#zk-snarks-vs-zk-starks)
- [Implemented Applications](#implemented-applications)
  - [1. Fibonacci SNARK](#1-fibonacci-snark)
  - [2. Matrix Multiplication SNARK](#2-matrix-multiplication-snark)
  - [3. Prime SNARK (Fermat Test)](#3-prime-snark-fermat-test)
- [Next.js Frontend](#nextjs-frontend)
- [Project Setup and Usage](#project-setup-and-usage)
- [Benchmarks](#benchmarks)
- [Dependencies](#dependencies)
- [Contributors](#contributors)

## Overview
This project, supervised by Dr. Eylon Yogev, demonstrates zk-SNARK applications using the ArkWorks Rust library. It covers:
1. **zk-SNARK Properties**: Key features include succinct proofs, non-interactiveness, computational soundness, and zero-knowledge privacy.
2. **zk-SNARKs vs zk-STARKs**: A comparison between zk-SNARKs and zk-STARKs, highlighting differences in scalability and the need (or lack) for a trusted setup.
3. **Programming in ArkWorks**: Using the Rust-based ArkWorks library, which offers efficient tools for zk-SNARK implementation, such as handling finite fields and setting up constraint systems.

## What are zk-SNARKs?
zk-SNARKs are cryptographic protocols that allow one party (the prover) to prove to another (the verifier) that a certain computation was performed correctly without revealing any details of the computation itself. The main properties of zk-SNARKs include:
- **Succinctness**: The proof size and verification time are minimal, regardless of the complexity of the computation.
- **Non-interactiveness**: No back-and-forth communication is required between prover and verifier after proof generation.
- **Arguments of Knowledge**: Ensures the prover actually knows the solution, rather than producing it by chance.
- **Zero Knowledge**: The verifier learns nothing beyond the fact that the computation is correct.

## zk-SNARKs vs zk-STARKs
While zk-SNARKs are efficient, they require a trusted setup, which can introduce risks of centralization and security vulnerabilities. zk-STARKs (Zero-Knowledge Scalable Transparent Arguments of Knowledge) provide a scalable alternative that doesn't require a trusted setup and supports larger computations with minimal complexity, making them preferable for applications needing high scalability.

## Implemented Applications
### 1. Fibonacci SNARK
A Fibonacci sequence proof generator that computes the nth Fibonacci number based on initial values \( a \) and \( b \). The proof generation provides the following information:
- **Verification Time**: Constant, regardless of the position in the Fibonacci sequence.
- **Proof Size**: A compact 384 bytes when using the Groth16 proof system in ArkWorks.

### 2. Matrix Multiplication SNARK
A zk-SNARK for verifying matrix multiplication. This implementation allows a client (verifier) to verify that a company (prover) correctly multiplied matrices \( A \) and \( B \) to obtain matrix \( C \), without revealing the actual matrices.
- **Process**:
  - The prover receives matrices \( A \) and \( B \) as witness inputs and computes \( C \).
  - Poseidon hashing is applied to enforce consistency between the provided hashes and computed values.
  - Proof is generated, and the verifier can verify this proof with minimal computational effort.
- **Benchmarking**: As the matrix size grows, proving time increases, but verification time remains constant.

### 3. Prime SNARK (Fermat Test)
A zk-SNARK that uses Fermat's primality test to verify the smallest prime value derived from hashing an input value \( x \) with SHA-256.
- **Circuit Implementation**:
  - Uses SHA-256 for hashing \( x \).
  - Implements modular exponentiation (modpow) and verifies primality through Fermat's test.
  - Detects Carmichael numbers, which can occasionally pass Fermat’s test despite not being prime.
- **Benchmarking**: Proving time increases with larger bit sizes, but verification time remains unaffected by input size.

## Next.js Frontend
The Next.js frontend provides an interactive UI for input and proof verification. Users can:
- Input values for Fibonacci, matrix multiplication, and primality testing.
- Generate and verify zk-SNARK proofs directly from the interface.
- View results and benchmark data in a user-friendly format.

## Project Setup and Usage
```bash
# 1. Clone the Repository
git clone [<repository-url>](https://github.com/ArielElb/zkSnark-FinalProject.git)
cd zkSnark-FinalProject

# 2. Running the Backend
# The backend, implemented in Rust, handles zk-SNARK proof generation and verification.
cd backend
cargo run --release

# 3. Running the Frontend
# The frontend, built with Next.js, provides a user interface for interacting with the backend.
cd frontend-next
npm install
npm run dev

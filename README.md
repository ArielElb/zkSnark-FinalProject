# README

## ZKProof Implementation for Miller-Rabin and Matrix Multiplication

Welcome to our project repository, where we implement zero-knowledge proofs (ZKProofs) for the Miller-Rabin primality test and matrix multiplication using the SP1 framework and `arkworks.rs`. This repository also includes a web front-end built with Next.js to interact with our ZKProof implementations.

### Table of Contents
- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Miller-Rabin ZKProof](#miller-rabin-zkproof)
  - [Matrix Multiplication ZKProof](#matrix-multiplication-zkproof)
- [Web Front-End](#web-front-end)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Zero-Knowledge Proofs (ZKProofs) allow one party to prove to another that they know a value without revealing the value itself. This project leverages the SP1 framework and `arkworks.rs` to create ZKProofs for two specific problems:
1. **Miller-Rabin Primality Test**: A probabilistic test to determine if a number is a prime.
2. **Matrix Multiplication Verification**: Proving the correctness of the multiplication of two matrices and their resultant hash using zkSNARKs.

## Features

- **Miller-Rabin Primality Test with ZKProof**: Implemented using the SP1 framework.
- **Matrix Multiplication ZKProof**: Using `arkworks.rs`, we prove that given witness matrices \(A\) and \(B\), and a public input hash of \(A * B\), the computation is correctly executed and hashed.
- **Next.js Web Front-End**: An intuitive web interface for users to interact with the ZKProofs.

## Installation

### Prerequisites

- Rust (for the ZKProof implementations)
- Node.js and npm (for the Next.js front-end)

### Steps

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/zkproof-matrix-multiplication.git
   cd zkproof-matrix-multiplication
   ```

2. **Set up the Rust environment**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. **Build the Rust project**:
   ```bash
   cargo build --release
   ```

4. **Set up the Next.js front-end**:
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

## Usage

### Miller-Rabin ZKProof

1. **Navigate to the Miller-Rabin ZKProof directory**:
   ```bash
   cd miller-rabin
   ```

2. **Run the proof generation**:
   ```bash
   cargo run --release
   ```

### Matrix Multiplication ZKProof

1. **Navigate to the Matrix Multiplication ZKProof directory**:
   ```bash
   cd matrix-multiplication
   ```

2. **Run the proof generation**:
   ```bash
   cargo run --release
   ```

## Web Front-End

The web front-end is built using Next.js and provides an interface for users to input data, generate ZKProofs, and verify them.

1. **Start the Next.js development server**:
   ```bash
   cd frontend
   npm run dev
   ```

2. **Access the web interface**:
   Open your web browser and navigate to `http://localhost:3000`.

## Contributing

Contributions are welcome! Please follow these steps to contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-xyz`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add some feature'`).
5. Push to the branch (`git push origin feature-xyz`).
6. Create a new Pull Request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

Thank you for your interest in our project! If you have any questions or feedback, please open an issue or contact us directly.

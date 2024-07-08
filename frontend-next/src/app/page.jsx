// src/pages/index.jsx
import Head from "next/head";
import styles from "../styles/Home.module.css";
import Link from "next/link";

export default function Home() {
  return (
    <div className={styles.container}>
      <Head>
        <title>Exploring Zero-Knowledge Proofs with SP1 and arkworks.rs</title>
        <meta
          name='description'
          content='Learn about zkSNARKs and arkworks.rs in a Next.js project'
        />
        <link rel='icon' href='/favicon.ico' />
      </Head>

      <main className={styles.main}>
        <h1 className={styles.title}>Home Page</h1>
        <div className={styles.explanation}>
          <p>
            Zero-Knowledge Proofs (zkProofs) are cryptographic protocols that allow one
            party (the prover) to prove to another party (the verifier) that they know a
            value or statement without revealing any information about the value itself.
            This project leverages the SP1 framework and{" "}
            <a href='https://arkworks.rs/' target='_blank' rel='noopener noreferrer'>
              arkworks.rs
            </a>{" "}
            to implement zkProofs for various computational tasks, including:
          </p>
          <ul>
            <li>
              <strong>Miller-Rabin Primality Test:</strong> Using SP1, we verify the
              correctness of prime tests without disclosing the number itself.
            </li>
            <li>
              <strong>Matrix Multiplication Verification:</strong> Demonstrating the
              integrity of matrix computations and their results using arkworks.rs,
              without revealing the matrices themselves.
            </li>
            <li>
              <strong>Fibonacci Sequence Computation:</strong> Showing how arkworks.rs can
              ensure the accuracy of computed Fibonacci numbers while keeping the initial
              parameters private.
            </li>
          </ul>
          <p>
            The Next.js web front-end provides an intuitive interface for users to
            interact with these zkProofs, demonstrating the practical application of
            cryptographic protocols in a user-friendly environment.
          </p>
          <p>
            <b>Cryptographic Primitives: </b> Utilizing the Poseidon sponge hash function
            for matrix multiplication and SHA-256 for the primality test, ensuring robust
            security and efficiency in our proofs.
          </p>
          <h2>Key Concepts Explained:</h2>
          <h3>1. What is zkSNARK?</h3>
          <p>
            Zero-Knowledge Succinct Non-Interactive Argument of Knowledge (zkSNARK) is a
            cryptographic protocol that enables efficient verification of computations
            without revealing private data. It allows a prover to convince a verifier that
            a statement is true without divulging any additional information beyond the
            validity of the statement.
          </p>
          <h3>2. What is R1CS?</h3>
          <p>
            Rank-1 Constraint System (R1CS) is a formulation used in zkSNARKs to express
            computational problems as a set of constraints. These constraints define what
            a valid solution looks like, enabling zkSNARKs to efficiently verify complex
            computations.
          </p>
          <h3>3. Proof Systems:</h3>
          <p>
            <strong>Marlin:</strong> Marlin is a zkSNARK proof system designed for
            universal and updatable recursive proofs. It optimizes proof size and
            verification time, making it suitable for various applications requiring
            scalability and efficiency.
          </p>
          <p>
            <strong>Groth16:</strong> Groth16 is another zkSNARK proof system known for
            its succinctness and efficiency in proving knowledge of a satisfying
            assignment to a given NP statement. It is widely used in applications
            requiring compact proofs and fast verification.
          </p>

          <h3>Frameworks:</h3>
          <p>
            <a href='https://arkworks.rs/' target='_blank' rel='noopener noreferrer'>
              arkworks.rs
            </a>{" "}
            is a Rust-based library providing essential tools and primitives for building
            zkSNARK circuits and proofs. It supports various elliptic curves and
            cryptographic primitives necessary for implementing zkSNARK protocols,
            including circuit synthesizers for R1CS.
          </p>
          <p>
            <a
              href='https://github.com/succinctlabs/sp1'
              target='_blank'
              rel='noopener noreferrer'
            > 
              SP1
            </a>{" "}
            SP1 is a performant, 100% open-source, contributor-friendly zero-knowledge
            virtual machine (zkVM) that can prove the execution of arbitrary Rust (or any
            LLVM-compiled language) programs. SP1 democratizes access to ZKPs by allowing
            developers to use programmable truth with popular programming languages. SP1
            is inspired by the open-source software movement and takes a collaborative
            approach towards building the best zkVM for rollups, coprocessors and other
            ZKP applications. We envision a diversity of contributors integrating the
            latest ZK innovations, creating a zkVM that is performant, customizable and
            will stand the test of time.
          </p>
          <p>
            By combining these concepts and technologies, our project demonstrates the
            power and utility of zero-knowledge proofs in verifying computations while
            maintaining data privacy and integrity.
          </p>
        </div>
      </main> 
    </div>
  );
}

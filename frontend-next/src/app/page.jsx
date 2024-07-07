// src/pages/index.jsx
import Head from "next/head";
import styles from "../styles/Home.module.css";
import Link from "next/link";

export default function Home() {
  return (
    <div className={styles.container}>
      <Head>
        <title>Nova Code Editor</title>
      </Head>
      <main className={styles.main}>
        <h1 className={styles.title}>home page</h1>
        <div className={styles.explanation}>
          Zero-Knowledge Proofs (ZKProofs) allow one party to prove to another that they
          know a value without revealing the value itself. This project leverages the SP1
          framework and arkworks.rs to create ZKProofs for two specific problems:
          <br />
          <br />
          Miller-Rabin Primality Test: A probabilistic test to determine if a number is a
          prime.
          <br />
          Matrix Multiplication Verification: Proving the correctness of the
          multiplication of two matrices and their resultant hash using zkSNARKs.
          <br />
          <br />
          Miller-Rabin Primality Test with ZKProof: Implemented using the SP1 framework.
          <br />
          Matrix Multiplication ZKProof: Using arkworks.rs, we prove that given witness
          matrices (A) and (B), and a public input hash of (A \times B), the computation
          is correctly executed and hashed.
          <br />
          <br />
          Next.js Web Front-End: An intuitive web interface for users to interact with the
          ZKProofs.
          <br />
          Cryptographic Primitives: We use the Poseidon sponge hash function for matrix
          multiplication and SHA-256 for the primality test.
        </div>
      </main>
    </div>
  );
}

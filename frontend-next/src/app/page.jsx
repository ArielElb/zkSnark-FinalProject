import Head from "next/head";
import styles from "../styles/Home.module.css";

export default function Home() {
  return (
    <>
      <div className={styles.container}>
        <header className={styles.header}>
          <div className={styles.headerParagraph}>
            <h1>zkSnark-Final Project</h1>
            <h2>What is our project all about?</h2>
            <p>
              Our project is a Proof of Concept (PoC) for implementing zero-knowledge
              proofs (zkProofs) using the SP1 framework and arkworks.rs. We are
              demonstrating zkProofs for various computational tasks, including primality
              tests, matrix multiplication, and Fibonacci sequence computation. Our
              project leverages zkProofs to ensure the integrity and efficency of
              computations while hold the Zero-Knowledge property.
            </p>
            <h3>What are zkProofs?</h3>
            <p>
              Zero-Knowledge Proofs (zkProofs) are cryptographic protocols that allow one
              party (the prover) to prove to another party (the verifier) that they know a
              value or statement without revealing any information about the value itself.
              This project leverages the SP1 framework and{" "}
              <a href='https://arkworks.rs/' target='_blank' rel='noopener noreferrer' className={styles.link}>
                arkworks.rs
              </a>{" "}
              to implement zkProofs for various computational tasks, including:
            </p>
            <ul>
              <li>
                <strong>Fermat Primality Test:</strong> We are using arkworks.rs to prove
                that given a seed x for the randomness,and another number i, we can prove
                that we compute and check if n=hash(x+i) is prime. (i.e., the hash of the
                sum of x and i is a prime number) when the randomness for the fermat
                primality test is a witness r = hash(x+i || n || i )
              </li>
              <li>
                <strong>Matrix Multiplication Verification:</strong> Demonstrating the
                integrity of matrix computations and their results using arkworks.rs,
                without revealing the matrices themselves.
              </li>
              <li>
                <strong>Fibonacci Sequence Computation:</strong> Showing how arkworks.rs
                can ensure the accuracy of computed Fibonacci numbers while keeping the
                initial parameters private.
              </li>
            </ul>

            <h3>What is a SNARK?</h3>
            <p>
              A succinct non-interactive argument of knowledge (SNARK) allows an untrusted
              prover P to prove the knowledge of a witness w satisfying some property. For
              example, w could be a pre-image of a designated value y of a cryptographic
              hash function h, i.e., a w such that h(w) = y. A trivial proof is for P to
              send w to the verifier V, who directly checks that w satisfies the claimed
              property. A SNARK achieves the same, but with better verification costs (and
              proof sizes). Succinct means that verifying a proof is exponentially faster
              than checking the witness directly (this also implies that proofs are
              exponentially smaller than the size of the statement proven).
            </p>
            <ul>
              <strong>The key features of zk-SNARKs are:</strong>
              <li>
                <b>Zero-Knowledge:</b> The verifier learns nothing other than the fact
                that the prover knows the solution.
              </li>
              <li>
                <b>Succinctness:</b> The proof is very small and can be verified quickly.
              </li>
              <li>
                <b>Non-Interactive: </b> After an initial setup phase, no interaction is
                required between the prover and verifier to verify the proof.
              </li>
              <li>
                <b>Arguments of Knowledge:</b> The proof convinces the verifier that the
                prover actually knows the solution.
              </li>
            </ul>
          </div>
        </header>
        <main className={styles.main}>
          <div className={styles.row}>
            <div className={styles.card}>
              <h2>Frameworks</h2>
              <p>
                <a href='https://arkworks.rs/' target='_blank' rel='noopener noreferrer'>
                  arkworks.rs
                </a>{" "}
                is a Rust-based library providing essential tools and primitives for
                building zkSNARK circuits and proofs. It supports various elliptic curves
                and cryptographic primitives necessary for implementing zkSNARK protocols,
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
                virtual machine (zkVM) that can prove the execution of arbitrary Rust (or
                any LLVM-compiled language) programs. SP1 democratizes access to ZKPs by
                allowing developers to use programmable truth with popular programming
                languages. SP1 is inspired by the open-source software movement and takes
                a collaborative approach towards building the best zkVM for rollups,
                coprocessors and other ZKP applications. We envision a diversity of
                contributors integrating the latest ZK innovations, creating a zkVM that
                is performant, customizable and will stand the test of time.
              </p>
            </div>
            <div className={styles.card}>
              <h2>R1CS</h2>

              <p>
                Rank-1 Constraint System (R1CS) is a formulation used in zkSNARKs to
                express computational problems as a set of constraints. These constraints
                define what a valid solution looks like, enabling zkSNARKs to efficiently
                verify complex computations.
              </p>
              <p>
                So the R1CS protocol is a powerful way to formally capture an algebraic
                circuit and to translate into a set of matrices and vectors that can be
                fed into a downstream proof system
              </p>
            </div>
          </div>
          <div className={styles.row}>
            <div className={styles.card}>
              <h2>Cryptographic Primitives: </h2>
              <p>
                Utilizing the Poseidon sponge hash function for matrix multiplication and
                SHA-256 for the primality test, ensuring robust security and efficiency in
                our proofs.
              </p>
            </div>

            <div className={styles.card}>
              <h2>zkSNARK Proof Systems</h2>
              <p>
                <strong>Groth16:</strong> Groth16 is another zkSNARK proof system known
                for its succinctness and efficiency in proving knowledge of a satisfying
                assignment to a given NP statement. It is widely used in applications
                requiring compact proofs and fast verification.
              </p>
              <p>
                <strong>Marlin:</strong> Marlin is a zkSNARK proof system designed for
                universal and updatable recursive proofs. It optimizes proof size and
                verification time, making it suitable for various applications requiring
                scalability and efficiency.
              </p>
            </div>
          </div>
        </main>
      </div>
    </>
  );
}

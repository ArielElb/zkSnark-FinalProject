// pages/index.js
"use client";
import Head from "next/head";
import styles from "../../styles/information.module.css";
import Link from "next/link";

const InformationPage = ({ searchParams }) => {
  const handleBack = () => {
    window.history.back();
  };
  return (
    <div className={styles.container}>
      <main className={styles.main}>
        <button onClick={handleBack} className={styles.topRightButton}>
          back to verify
        </button>
        <h1 className={styles.title}>Information about {searchParams.type}</h1>

        {searchParams.type == "Prime number" && ( //explain about prime
          <p className={styles.description}>
            In cryptography, there are many scenarios where we use prime numbers. These
            prime numbers need to be mutually agreed upon by both parties involved. We do
            not want the client to compute the prime number on their own. Conversely, we
            do not want the server to have full control over the selection of the prime
            number. <br></br> <br></br>
            With zk proof we can use the server to work on checking if the number is a
            prime number and the client will need only to verify it.
          </p>
        )}
        {searchParams.type == "fibonachi number" && ( //explain about fibonachi number
          <div className={styles.description}>
            <h1>Fibonacci Sequence and zk-SNARKs</h1>
            <p>
              The Fibonacci sequence is a series of numbers where each number is the sum
              of the two preceding ones, starting from 0 and 1. Mathematically, it can be
              defined as:
            </p>
            <ul>
              <li>F(0) = 0</li>
              <li>F(1) = 1</li>
              <li>F(n) = F(n-1) + F(n-2) for n `{">"}` 1</li>
            </ul>
            <h2>Zero-Knowledge Proofs</h2>
            <p>
              Zero-Knowledge Succinct Non-Interactive Arguments of Knowledge (zk-SNARKs)
              are cryptographic protocols that allow one party (the prover) to prove to
              another party (the verifier) that they know a solution to a problem, without
              revealing the solution itself.
            </p>
            <h2>Connecting Fibonacci to zk-SNARKs</h2>
            <p>
              In this implementation, we use zk-SNARKs to prove that we know the correct
              result of a Fibonacci sequence computation without revealing the sequence
              itself. Here's how it works:
            </p>
            <h3>1. Circuit Definition</h3>
            <p>
              We define a <strong>Fibonacci Circuit</strong> that represents the
              computation of the Fibonacci sequence. The circuit takes:
            </p>
            <ul>
              <li>
                Initial values: <code>a</code> and <code>b</code>
              </li>
              <li>
                Number of steps: <code>num_of_steps</code>
              </li>
              <li>
                Expected result: <code>result</code>
              </li>
            </ul>
            <p>
              The circuit enforces the rule that each number in the sequence is the sum of
              the two preceding numbers.
            </p>
            <h3>2. Proving</h3>
            <p>
              The prover uses the defined circuit and their private input (the initial
              values and the number of steps) to generate a proof. This proof shows that
              the prover knows the correct Fibonacci sequence leading to the expected
              result.
            </p>
            <h3>3. Verification</h3>
            <p>
              The verifier uses the proof to confirm that the prover knows the correct
              result of the Fibonacci computation, without learning the actual sequence.
              This ensures that the computation was performed correctly.
            </p>
            <h3>4. Proof Systems Used</h3>
            <p>This implementation uses two proof systems:</p>
            <ul>
              <li>
                <strong>Marlin:</strong> A universal zk-SNARK proof system that supports
                efficient proving and verification for various computations without
                trusted setup.
              </li>
              <li>
                <strong>Groth16:</strong> A zk-SNARK proof system known for producing very
                short proofs and quick verification times.
              </li>
            </ul>
            <h3>Summary</h3>
            <p>
              By using zk-SNARKs, we can verify the correctness of a Fibonacci sequence
              computation without revealing the sequence itself. This provides privacy and
              efficiency in proving mathematical computations.
            </p>
          </div>
        )}
        {searchParams.type == "matrix multification" && ( //explain about matrix multification
          <div className={styles.description}>
            <h1>Matrix Operations and zk-SNARKs</h1>
            <p>
              In this implementation, we use zk-SNARKs to prove the correctness of matrix
              multiplication without revealing the matrices themselves. Here's an
              explanation of how it works:
            </p>
            <h2>Matrix Operations</h2>
            <p>
              We perform matrix multiplication, which involves multiplying two matrices to
              produce a third matrix. Specifically, if we have two matrices <code>A</code>{" "}
              and <code>B</code>, their product <code>C</code> is calculated as follows:
            </p>
            <ul>
              <li>
                Each element in <code>C</code> is the dot product of the corresponding row
                from <code>A</code> and column from <code>B</code>.
              </li>
            </ul>
            <h2>Zero-Knowledge Proofs</h2>
            <p>
              Zero-Knowledge Succinct Non-Interactive Arguments of Knowledge (zk-SNARKs)
              are cryptographic protocols that allow one party (the prover) to prove to
              another party (the verifier) that they know a solution to a problem, without
              revealing the solution itself.
            </p>
            <h2>Connecting Matrices to zk-SNARKs</h2>
            <p>
              In this implementation, we use zk-SNARKs to prove that the product of two
              matrices <code>A</code> and <code>B</code> equals a given matrix{" "}
              <code>C</code>, without revealing the matrices themselves. Here's how it
              works:
            </p>
            <h3>1. Public and Private Inputs</h3>
            <p>We use the following inputs:</p>
            <ul>
              <li>
                Public inputs: <code>hash(A)</code>, <code>hash(B)</code>,{" "}
                <code>hash(C)</code>
              </li>
              <li>
                Private inputs: <code>A</code>, <code>B</code>
              </li>
            </ul>
            <h3>2. Hashing with Poseidon</h3>
            <p>
              To ensure the integrity of the matrices, we use the{" "}
              <strong>Poseidon hash function</strong>, which is based on a cryptographic
              sponge construction. The steps are:
            </p>
            <ul>
              <li>
                Compute after flattening A,B to vec(A),vec(B) <code>hash(A)</code> and{" "}
                <code>hash(B)</code> and compare them to the public inputs.
              </li>
              <li>
                Perform matrix multiplication to get <code>C = A * B</code>.
              </li>
              <li>
                Flatten the resulting matrix <code>C</code> into a single vector.
              </li>
              <li>
                Compute <code>hash(C)</code> from the flattened vector and use it as a
                public input.
              </li>
            </ul>
            <h3>3. Circuit Definition</h3>
            <p>
              We define a <strong>Matrix Circuit</strong> that represents the computation
              of matrix multiplication. The circuit enforces the following rules:
            </p>
            <ul>
              <li>
                Each element in <code>C</code> is the dot product of the corresponding row
                from <code>A</code> and column from <code>B</code>.
              </li>
              <li>
                The hash of the flattened <code>A</code> must equal the public input{" "}
                <code>hash(A)</code>.
              </li>
              <li>
                The hash of the flattened <code>B</code> must equal the public input{" "}
                <code>hash(B)</code>.
              </li>
              <li>
                The hash of the flattened <code>C</code> must equal the public input{" "}
                <code>hash(C)</code>.
              </li>
            </ul>
            <h3>4. Proving</h3>
            <p>
              The prover uses the defined circuit and their private input (the matrices)
              to generate a proof. This proof shows that the prover knows the correct
              product of the matrices without revealing the matrices themselves.
            </p>
            <h3>5. Verification</h3>
            <p>
              The verifier uses the proof to confirm that the prover knows the correct
              product of the matrices, without learning the actual matrices. This ensures
              that the computation was performed correctly.
            </p>
            <h3>6. Proof Systems Used</h3>
            <p>
              This implementation uses the <strong>Groth16</strong> proof system, which is
              known for producing very short proofs and quick verification times.
            </p>
            <h3>Summary</h3>
            <p>
              By using zk-SNARKs and the Poseidon hash function, we can verify the
              correctness of matrix multiplication without revealing the matrices
              themselves. This provides privacy and efficiency in proving mathematical
              computations.
            </p>
          </div>
        )}
        {searchParams.type=="Prime number"&& //explain about prime
       
       <div className={styles.imageContainer}>
          
       <div className={styles.imageRow}>
         <img src="/prime3.png" alt="Image 1" className={styles.image} />
         <img src="/prime2.png" alt="Image 2" className={styles.image} />
       </div>
     </div>}
        {searchParams.type=="fibonachi number"&& //explain about fibonachi number
      
      <div className={styles.imageContainer}>
          
      <div className={styles.imageRow}>
        <img src="/fib_186_1.png" alt="Image 1" className={styles.image} />
        <img src="/fib_186_2.png" alt="Image 2" className={styles.image} />
      </div>
    </div>}
        {searchParams.type=="matrix multification"&& //explain about matrix multification
     
        <div className={styles.imageContainer}>
              
        <div className={styles.imageRow}>
          <img src="/matrix_mult_power_1.png" alt="Image 1" className={styles.image} />
          <img src="/matrix_mult_power_2.png" alt="Image 2" className={styles.image} />
        </div>
        <div className={styles.imageRow}>
          <img src="/matrix_mult_power_3.png" alt="Image 3" className={styles.image} />
        </div>
        </div>}

      </main>
    </div>
  );
};

export default InformationPage;

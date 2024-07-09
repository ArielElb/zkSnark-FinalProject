// src/app/verify/page.jsx
"use client";
import Link from "next/link";
import { useState } from "react";
import axios from "axios";
import styles from "../../styles/verify.module.css"; // תיקון נתיב לקובץ ה-CSS

// Create a custom Axios instance with the specified settings
const axiosInstance = axios.create({
  maxContentLength: 100000000,
  maxBodyLength: 1000000000,
});

const InputPrimePage = () => {
  const [n, setN] = useState(20);
  const [numOfRounds, setNumOfRounds] = useState(10);
  const [seed, setSeed] = useState(123131);
  const [proofResponse, setProofResponse] = useState(null);
  const [verifyResponse, setVerifyResponse] = useState(null);
  const [isLoadingProof, setIsLoadingProof] = useState(false);
  const [isLoadingVerify, setIsLoadingVerify] = useState(false);
  const [isPrime, setIsPrime] = useState(false);
  const [proofTime, setProofTime] = useState(0);
  const [proofSize, setProofSize] = useState(0);

  const handleProve = () => {
    setIsLoadingProof(true);
    axiosInstance
      .post("http://127.0.0.1:8080/api/prime_sp1/prove", {
        n,
        num_of_rounds: numOfRounds,
        seed,
      })
      .then((response) => {
        const { is_prime, proof_time, proof_size, proof, vkey } = response.data;
        setIsPrime(is_prime);
        setProofTime(proof_time);
        setProofSize(proof_size);
        setProofResponse({ is_prime, proof_time, proof_size });

        localStorage.setItem("proof", JSON.stringify(proof));
        localStorage.setItem("vkey", vkey);
      })
      .catch((error) => {
        console.error("Error generating proof:", error);
      })
      .finally(() => {
        setIsLoadingProof(false);
      });
  };

  const handleVerify = () => {
    const proof = JSON.parse(localStorage.getItem("proof"));
    const vkey = localStorage.getItem("vkey");

    setIsLoadingVerify(true);
    axiosInstance
      .post("http://127.0.0.1:8080/api/prime_sp1/verify", { proof, vkey })
      .then((response) => {
        setVerifyResponse(response.data);
      })
      .catch((error) => {
        console.error("Error verifying proof:", error);
      })
      .finally(() => {
        setIsLoadingVerify(false);
      });
  };

  return (
    <div className={styles.container}>
      <h1>Proof Generation and Verification</h1>
      <div>
        <h2>Generate Proof</h2>
        <label>
          n:
          <input
            type='number'
            value={n}
            onChange={(e) => setN(parseInt(e.target.value))}
          />
        </label>
        <label>
          Number of Rounds:
          <input
            type='number'
            value={numOfRounds}
            onChange={(e) => setNumOfRounds(parseInt(e.target.value))}
          />
        </label>
        <label>
          Seed:
          <input
            type='number'
            value={seed}
            onChange={(e) => setSeed(parseInt(e.target.value))}
          />
        </label>
        <button onClick={handleProve} disabled={isLoadingProof}>
          {isLoadingProof ? "Generating Proof..." : "Generate Proof"}
        </button>
        {proofResponse && (
          <div className={styles.responseContainer}>
            <h3>Proof Details</h3>
            <p>Is Prime: {isPrime ? "Yes" : "No"}</p>
            <p>Proof Time: {proofTime} ms</p>
            <p>Proof Size: {proofSize} bytes</p>
          </div>
        )}
      </div>
      <div>
        <h2>Verify Proof</h2>
        <button onClick={handleVerify} disabled={isLoadingVerify}>
          {isLoadingVerify ? "Verifying..." : "Verify Proof"}
        </button>
        {verifyResponse && (
          <div className={styles.responseContainer}>
            <h3>Verification Response</h3>
            <pre>{JSON.stringify(verifyResponse, null, 2)}</pre>
          </div>
        )}
      </div>
    </div>
  );
};

export default InputPrimePage;

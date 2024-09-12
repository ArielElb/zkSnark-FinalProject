"use client";
import { useState } from "react";
import styles from "../../styles/verify.module.css";
import Link from "next/link";
import axios from "axios";

const InputPrimePage = () => {
  const [currentOption, setCurrentOption] = useState("prove");
  const [number, setNumber] = useState(""); // Input number for proving prime
  const [rounds, setRounds] = useState(""); // Input number of rounds
  const [primeResult, setPrimeResult] = useState(null); // Store prime result from the proof
  const [isLoading, setIsLoading] = useState(false); // Loading state for proof
  const [verifyResult, setVerifyResult] = useState(""); // Verification result
  const [verifyingTime, setVerifyingTime] = useState(""); // Verification time
  const [recProof,setRecProof] = useState("");

  const handleProvePrime = () => {
    if (number && rounds) {
      const requestData = {
        x: parseInt(number), // Seed number (x)
        i: parseInt(rounds), // Number of rounds (i)
      };

      setIsLoading(true);
      axios
        .post("http://127.0.0.1:8080/api/prime_arkworks/prove", requestData)
        .then((response) => {
          console.log("Prime prove response:", response.data);
          setPrimeResult(response.data);
          const {
            proof,
            pvk,
            setup_time,
            proving_time,
            num_constraints,
            num_variables,
            j,
            found_prime,
            prime_num,
          } = response.data;
          localStorage.setItem("prime_proof", JSON.stringify(proof)); // Store proof as JSON
          localStorage.setItem("pvk", JSON.stringify(pvk)); // Store pvk as JSON
          localStorage.setItem("prime_num", prime_num); // Store prime number
          localStorage.setItem("prime_round", j); // Store the round where prime was found
          localStorage.setItem("seed_number", number); // Save the seed number (x)
          setRecProof(proof)
        })
        .catch((error) => {
          console.error("Error proving prime:", error);
        })
        .finally(() => {
          setIsLoading(false);
        });
    } else {
      alert("Please enter a number and number of rounds.");
    }
  };

  const handleVerifyPrime = () => {
    const proof = JSON.parse(localStorage.getItem("prime_proof")); // Parse proof from localStorage
    const pvk = JSON.parse(localStorage.getItem("pvk")); // Parse pvk from localStorage
    const prime_round = localStorage.getItem("prime_round"); // The round from localStorage
    const seed_number = localStorage.getItem("seed_number"); // The seed number from localStorage

    const requestData = {
      x: parseInt(seed_number), // Seed number (x)
      j: parseInt(prime_round), // The round where the prime was found (j)
      proof, // Proof
      pvk, // PVK
    };

    setIsLoading(true);
    axios
      .post("http://127.0.0.1:8080/api/prime_arkworks/verify", requestData)
      .then((response) => {
        console.log("Prime verify response:", response.data);
        const { valid, verifying_time } = response.data;
        if (valid) {
          setVerifyResult("Verification successful!");
        } else {
          setVerifyResult("Verification failed.");
        }
        setVerifyingTime(verifying_time);
      })
      .catch((error) => {
        console.error("Error verifying prime:", error);
        setVerifyResult("Verification failed.");
      })
      .finally(() => {
        setIsLoading(false);
      });
  };

  return (
    <div className={styles.container}>
      <Link href={{ pathname: "../information/", query: { type: "Prime number" } }}>
        <button className={styles.topRightButton}>More Information</button>
      </Link>
      <div className={styles.optionButtons}>
        <button
          onClick={() => setCurrentOption("prove")}
          className={currentOption === "prove" ? styles.activeButton : ""}
        >
          Prove
        </button>
        <button
          onClick={() => setCurrentOption("verify")}
          className={currentOption === "verify" ? styles.activeButton : ""}
        >
          Verify
        </button>
      </div>

      {currentOption === "prove" && (
        <>
          <h1 className={styles.title}>Prime Input</h1>
          <div className={styles.inputRows}>
            <input
              type='text'
              className={styles.inputField}
              value={number}
              onChange={(e) => setNumber(e.target.value)}
              placeholder='Enter a seed number x'
            />
            <input
              type='text'
              className={styles.inputField}
              value={rounds}
              onChange={(e) => setRounds(e.target.value)}
              placeholder='Enter the number of rounds:'
            />
          </div>
          <button className={styles.saveButton} onClick={handleProvePrime}>
            Prove
          </button>

          {isLoading && <p>Loading proof...</p>}

          {primeResult && !isLoading && (
            <div className={styles.resultContainer}>
              <h2>Prime Prove Results:</h2>
              <p>Prime Number: {primeResult.prime_num}</p>
              <p>Prime was found in the {primeResult.j}-th round</p>
              <p>Setup Time: {primeResult.setup_time} seconds</p>
              <p>Proving Time: {primeResult.proving_time} seconds</p>
              <p>Number of Constraints: {primeResult.num_constraints}</p>
              <p>Number of Variables: {primeResult.num_variables}</p>
              <p>The proof: {recProof.substring(0, 150)}</p>
              {primeResult.found_prime ? (
                <p style={{ color: "green" }}>Prime found: {primeResult.prime_num}</p>
              ) : (
                <p style={{ color: "red" }}>Prime not found in given rounds.</p>
              )}
            </div>
          )}
        </>
      )}

      {currentOption === "verify" && (
        <div className={styles.option2Container}>
          <h1 className={styles.title}>Verify Proof</h1>
          <p>Seed Number (x): {localStorage.getItem("seed_number")}</p>
          <p>Round (j): {localStorage.getItem("prime_round")}</p>
          <button className={styles.saveButton} onClick={handleVerifyPrime}>
            Verify
          </button>

          {isLoading && <p>Verifying...</p>}

          {verifyResult && !isLoading && (
            <div className={styles.verifyResult}>
              <h2>Verification Result:</h2>
              <p>{verifyResult}</p>
              <p>Verifying Time: {verifyingTime} seconds</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default InputPrimePage;

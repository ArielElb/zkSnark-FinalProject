"use client";
import { useState, useEffect } from "react";
import axios from "axios";
import Link from "next/link";

import styles from "../../styles/verify.module.css"; // תיקון נתיב לקובץ ה-CSS

const InputFibonacciPage = () => {
  const [number, setNumber] = useState("");
  const [rounds, setRounds] = useState("");
  const [a, setA] = useState("");
  const [b, setB] = useState("");
  const [currentOption, setCurrentOption] = useState("prove");
  const [isLoadingProof, setIsLoadingProof] = useState(false);
  const [provingTime, setProvingTime] = useState(null);
  const [verifyResult, setVerifyResult] = useState("");
  const [verifyingTime, setVerifyingTime] = useState("");
  const [isLoadingVerify, setIsLoadingVerify] = useState(false);
  const [recProof, setRecProof] = useState("");
  const [error, setError] = useState(""); // State for error messages

  // Separate state for verification inputs to avoid affecting the prove inputs
  const [verifyA, setVerifyA] = useState("");
  const [verifyB, setVerifyB] = useState("");
  const [verifyNumber, setVerifyNumber] = useState("");

  useEffect(() => {
    // Pre-fill the input fields with values from localStorage on page load for Prove
    const firstNumber = JSON.parse(localStorage.getItem("first_number"));
    const secondNumber = JSON.parse(localStorage.getItem("second_number"));
    const fibNumber = JSON.parse(localStorage.getItem("fib_number"));

    if (firstNumber) setA(firstNumber);
    if (secondNumber) setB(secondNumber);
    if (fibNumber) setNumber(fibNumber);

    // Optionally, pre-fill verify fields with localStorage values initially
    if (firstNumber) setVerifyA(firstNumber);
    if (secondNumber) setVerifyB(secondNumber);
    if (fibNumber) setVerifyNumber(fibNumber);
  }, []);

  const handleProve = () => {
    if (rounds && a && b) {
      if (rounds > 183) {
        setError("The maximum number of rounds is 186.");
        return; // Exit the function if the condition is met
      }
      setError("");
      // Reset previous stats
      setProvingTime(null);
      const requestData = {
        a: parseInt(a),
        b: parseInt(b),
        num_of_rounds: parseInt(rounds),
      };
      localStorage.setItem("first_number", JSON.stringify(parseInt(a)));
      localStorage.setItem("second_number", JSON.stringify(parseInt(b)));
      setIsLoadingProof(true);
      axios
        .post("http://127.0.0.1:8080/api/fibbonaci/prove", requestData)
        .then((response) => {
          const { proof, pvk, fib_number, proving_time } = response.data;
          localStorage.setItem("proof", JSON.stringify(proof));
          localStorage.setItem("pvk", JSON.stringify(pvk));
          setProvingTime(proving_time);
          setNumber(fib_number);
          console.log(proof);
          setRecProof(proof);
          localStorage.setItem("fib_number", JSON.stringify(parseInt(fib_number)));
        })
        .catch((error) => {
          console.error("Error proving fibonacci:", error);
        })
        .finally(() => {
          setIsLoadingProof(false);
        });
    } else {
      alert(
        "Please enter the number of rounds and the first and second fibonacci numbers."
      );
    }
  };

  const handleVerify = () => {
    setVerifyResult("");
    const pvk = JSON.parse(localStorage.getItem("pvk"));
    const proof = JSON.parse(localStorage.getItem("proof"));

    const requestData = {
      proof: proof,
      pvk: pvk,
      a: parseInt(verifyA), // Send the edited 'a' value from verify input
      b: parseInt(verifyB), // Send the edited 'b' value from verify input
      fib_number: parseInt(verifyNumber), // Send the edited Fibonacci number from verify input
    };

    setIsLoadingVerify(true);
    axios
      .post("http://127.0.0.1:8080/api/fibbonaci/verify", requestData)
      .then((response) => {
        const { is_res, verifying_time } = response.data;
        if (is_res) {
          setVerifyResult("Verification successful!");
        } else {
          setVerifyResult("Verification failed.");
        }
        setVerifyingTime("Verifying Time: " + verifying_time);
      })
      .catch((error) => {
        console.error("Error verifying fibonacci:", error);
        setVerifyResult("Verification failed due to an error.");
        setVerifyingTime("");
      })
      .finally(() => {
        setIsLoadingVerify(false);
      });
  };

  return (
    <div className={styles.container}>
      <Link href={{ pathname: "../information/", query: { type: "fibonachi number" } }}>
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
          <h1 className={styles.title}>Fibonacci Input</h1>
          <div className={styles.inputRows}>
            <input
              type='text'
              className={styles.inputField}
              value={rounds}
              onChange={(e) => setRounds(e.target.value)}
              placeholder='Enter the number of rounds'
            />
            <input
              type='text'
              className={styles.inputField}
              value={a} // Values from localStorage
              onChange={(e) => setA(e.target.value)}
              placeholder='Enter the first Fibonacci number'
            />
            <input
              type='text'
              className={styles.inputField}
              value={b} // Values from localStorage
              onChange={(e) => setB(e.target.value)}
              placeholder='Enter the second Fibonacci number'
            />
          </div>
          <button onClick={handleProve} className={styles.saveButton}>
            Prove
          </button>
          {error && <p className={styles.error}>{error}</p>}
          {isLoadingProof && (
            <div className={styles.loading}>
              <p>Loading proof...</p>
            </div>
          )}
          <div className={styles.additionalInfo}>
            {provingTime !== null && (
              <>
                <p>The Fibonacci number is: {number}</p>
                <p>Proving Time: {provingTime.toFixed(6)} seconds</p>
                <p>The proof: {recProof.substring(0, 120)}</p>
                <p>{recProof.substring(120, 300)}</p>
              </>
            )}
          </div>
        </>
      )}

      {currentOption === "verify" && (
        <div className={styles.option2Container}>
          <h1 className={styles.title}>Verify Proof</h1>
          <div className={styles.inputRows}>
            <input
              type='text'
              className={styles.inputField}
              value={verifyA} // Separate state for verify input
              onChange={(e) => setVerifyA(e.target.value)}
              placeholder='Enter the first Fibonacci number'
            />
            <input
              type='text'
              className={styles.inputField}
              value={verifyB} // Separate state for verify input
              onChange={(e) => setVerifyB(e.target.value)}
              placeholder='Enter the second Fibonacci number'
            />
            <input
              type='text'
              className={styles.inputField}
              value={verifyNumber} // Separate state for verify input
              onChange={(e) => setVerifyNumber(e.target.value)}
              placeholder='Enter the Fibonacci number to verify'
            />
          </div>
          <button onClick={handleVerify} className={styles.saveButton}>
            Verify
          </button>
          {isLoadingVerify && (
            <div className={styles.loading}>
              <p>Verifying...</p>
            </div>
          )}
          {verifyResult && !isLoadingVerify && (
            <div>
              <h2>Verification Result:</h2>
              <p>{verifyResult}</p>
              <p>{verifyingTime}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default InputFibonacciPage;

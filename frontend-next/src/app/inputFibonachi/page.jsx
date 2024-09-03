// src/app/verify/page.jsx
"use client";
import { useState } from "react";
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
  BigInt.prototype.toJSON = function () {
    return { $bigint: this.toString() };
  };
  const handleProve = () => {
    // Reset previous stats
    setProvingTime(null);
    const requestData = {
      a: parseInt(a),
      b: parseInt(b),
      result: number.toString(),
      num_of_rounds: parseInt(rounds),
    };
    localStorage.setItem("first_number", JSON.stringify(parseInt(a)));
    localStorage.setItem("second_number", JSON.stringify(parseInt(b)));
    setIsLoadingProof(true);
    axios
      .post("http://127.0.0.1:8080/api/fibbonaci/prove", requestData)
      .then((response) => {
        const { proof, pvk, proving_time } = response.data;
        localStorage.setItem("proof", JSON.stringify(proof));
        localStorage.setItem("pvk", JSON.stringify(pvk));
        setProvingTime(proving_time);
      })
      .catch((error) => {
        console.error("Error proving fibonacci:", error);
      })
      .finally(() => {
        setIsLoadingProof(false);
      });
  };
  const handleVerify = () => {
    const pvk = JSON.parse(localStorage.getItem("pvk"));
    const proof = JSON.parse(localStorage.getItem("proof"));
    setVerifyResult("");
    const requestData = {
      proof: proof,
      pvk: pvk,
      a: JSON.parse(localStorage.getItem("first_number")),
      b: JSON.parse(localStorage.getItem("second_number")),
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
        console.error("Error verifying matrices:", error);
        setVerifyResult("Verification failed error.");
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
          <h1 className={styles.title}>fibonacci input</h1>
          <div className={styles.inputRows}>
            <input
              type='text'
              className={styles.inputField}
              value={number}
              onChange={(e) => setNumber(e.target.value)}
              placeholder='enter the fibonacci number'
            />
            <input
              type='text'
              className={styles.inputField}
              value={rounds}
              onChange={(e) => setRounds(e.target.value)}
              placeholder='enter the number of rounds'
            />
            <input
              type='text'
              className={styles.inputField}
              value={a}
              onChange={(e) => setA(e.target.value)}
              placeholder='enter the first fibonacci number'
            />
            <input
              type='text'
              className={styles.inputField}
              value={b}
              onChange={(e) => setB(e.target.value)}
              placeholder='enter the second fibonacci number'
            />
          </div>
          <button onClick={handleProve} className={styles.saveButton}>
            Prove
          </button>
          {isLoadingProof && (
            <div className={styles.loading}>
              <p>Loading proof...</p>
            </div>
          )}
          <div className={styles.additionalInfo}>
            {provingTime !== null && (
              <p>Proving Time: {provingTime.toFixed(6)} seconds</p>
            )}
          </div>
        </>
      )}

      {currentOption === "verify" && (
        <div className={styles.option2Container}>
          <h1 className={styles.title}>Verify Proof</h1>
          <button onClick={handleVerify} className={styles.saveButton}>
            Verify
          </button>
          {isLoadingVerify && (
            <div className={styles.loading}>
              <p>Verifying...</p>
            </div>
          )}
          {verifyResult && !isLoadingVerify && (
            <div className={styles.verifyResult}>
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

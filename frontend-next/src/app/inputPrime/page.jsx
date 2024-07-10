"use client";
import { useState, useEffect } from "react";
import styles from "../../styles/verify.module.css";
import Link from "next/link";

const InputPrimePage = () => {
  const [currentOption, setCurrentOption] = useState("prove");
  const [number, setNumber] = useState("");
  const [rounds, setRounds] = useState("");





  const handleProve = () => {
    // Reset previous stats
    setSetupTime(null);
    setProvingTime(null);
    setNumConstraints(null);
    setNumVariables(null);

    const requestData = {
      n: number,
      num_of_rounds: rounds,
      matrixseed: 123,
    };

    setIsLoadingProof(true);
    axios
      .post("http://127.0.0.1:8080/api/prime_sp1/prove", requestData)
      .then((response) => {
        const {
          proof,
          pvk,
          hash_a,
          hash_b,
          hash_c,
          setup_time,
          proving_time,
          num_constraints,
          num_variables,
        } = response.data;
        localStorage.setItem("proof", JSON.stringify(proof));
        localStorage.setItem("pvk", JSON.stringify(pvk));
        setHashes({ hash_a, hash_b, hash_c });
        setSetupTime(setup_time);
        setProvingTime(proving_time);
        setNumConstraints(num_constraints);
        setNumVariables(num_variables);
      })
      .catch((error) => {
        console.error("Error proving matrices:", error);
      })
      .finally(() => {
        setIsLoadingProof(false);
      });
  };

  return (
    <div className={styles.container}>
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
          <h1 className={styles.title}>prime input</h1>
          <div className={styles.inputRows}>
          <input type="text" className={styles.inputField} value={number}
              onChange={(e) => setNumber(e.target.value)}  placeholder="enter the prime number" />
          <input type="text" className={styles.inputField} value={rounds}
              onChange={(e) => setRounds(e.target.value)} placeholder="enter the number of rounds" />
          </div>
              <button className={styles.saveButton}>Prove</button>
        </>
      )}

      {currentOption === "verify" && (
        <div className={styles.option2Container}>
          <h1 className={styles.title}>Verify Proof</h1>
          <label className={styles.label}>
            Enter Hash A:
            <input
              type="text"
              className={styles.inputBox}
            />
          </label>
                  <button className={styles.saveButton}>Verify</button>
        </div>
      )}
    </div>
  );
};

export default InputPrimePage;

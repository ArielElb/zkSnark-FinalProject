// src/app/verify/page.jsx
"use client";
import { useState } from "react";
import axios from "axios";

import styles from "../../styles/verify.module.css"; // תיקון נתיב לקובץ ה-CSS

const InputPrimePage = ({}) => {
  const [number, setNumber] = useState("");
  const [rounds, setRounds] = useState("");
  const [result, setResult] = useState(null);
  
  const handleSubmit = async (e) => {
    e.prevent.prevent();
    try {
      const response = await axios.post("http://localhost:8080/api/prime_snark", {
        x: parseInt(number),
        num_of_rounds: parseInt(rounds),
      });
      setResult(response.data);
    } catch (error) {
      console.error("Error submitting the form", error);
      setResult({ error: "Failed to compute. Please try again." });
    }
  };

  return (
    <div className={styles.background}>
    <div className={styles.container}>
      <h1 className={styles.title}>PROVE prime number</h1>
      <header className={styles.header}>
        <form onSubmit={handleSubmit} className={styles.form}>
          <div className={styles.inputGroup}>
            <input
              id="number"
              type='number'
              value={number}
              onChange={(e) => setNumber(e.target.value)}
              className={styles.input}
              placeholder="Get Number"
            />
          </div>
          <div className={styles.inputGroup}>
            <input
              id="rounds"
              type='number'
              value={rounds}
              onChange={(e) => setRounds(e.target.value)}
              className={styles.input}
              placeholder="Get Number of Rounds"
            />
          </div>
          <button type='submit' className={styles.button}>Submit</button>
        </form>
      </header>
      {result && (
        <div className={styles.result}>
          <h2>Result</h2>
          {result.error ? (
            <p>{result.error}</p>
          ) : (
            <>
              <p>Proof: {result.proof}</p>
              <p>Public Input: {result.public_input.join(", ")}</p>
              <p>Number of Constraints: {result.num_constraints}</p>
              <p>Number of Variables: {result.num_variables}</p>
              <p>Proving Time: {result.proving_time} seconds</p>
              <p>Verifying Time: {result.verifying_time} seconds</p>
            </>
          )}
        </div>
      )}
    </div>
    </div>
  );
};

export default InputPrimePage;

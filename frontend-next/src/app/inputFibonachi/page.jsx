// src/app/verify/page.jsx
"use client";
import { useState } from "react";
import axios from "axios";
import Link from "next/link";

import styles from "../../styles/verify.module.css"; // תיקון נתיב לקובץ ה-CSS

const InputPrimePage = () => {
  const [number, setNumber] = useState("");
  const [rounds, setRounds] = useState("");
  const [a, setA] = useState("");
  const [b, setB] = useState("");
  
  return (
    <div className={styles.background}>
    <div className={styles.container}>
      <h1 className={styles.title}>Prove Fibonachi Number</h1>
        <form className={styles.form}>
          <div className={styles.inputGroup}>
            <input
              id="number"
              type='number'
              value={number}
              onChange={(e) => setNumber(e.target.value)}
              className={styles.input}
              placeholder="Get fibonachi Number"
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
          <div className={styles.inputGroup}>
            <input
              id="a"
              type='number'
              value={a}
              onChange={(e) => setA(e.target.value)}
              className={styles.input}
              placeholder="Get first number of fibonachi"
            />
          </div>
          <div className={styles.inputGroup}>
            <input
              id="rounds"
              type='b'
              value={b}
              onChange={(e) => setB(e.target.value)}
              className={styles.input}
              placeholder="Get second number of fibonachi"
            />
          </div>
          <Link href={{pathname: "../verifyPrime" , query: {number: number, rounds:rounds, a: a, b: b },}}>
             <button  type='submit' className={styles.button}>Get result</button>
          </Link>
        </form>
    </div>
    </div>
  );
};

export default InputPrimePage;

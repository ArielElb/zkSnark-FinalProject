// src/app/verify/page.jsx
"use client";
import Link from "next/link";

import { useState } from "react";
import axios from "axios";
import { useRouter } from 'next/router';

import styles from "../../styles/verify.module.css"; // תיקון נתיב לקובץ ה-CSS

const InputPrimePage = ({searchParams}) => {
  const [number, setNumber] = useState("");
  const [rounds, setRounds] = useState(""); 
  return (
    <div className={styles.background}>
    <div className={styles.container}>
      <h1 className={styles.title}>Prove {searchParams.msg}</h1>
        <form className={styles.form}>
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
          <Link href={{pathname: "../verifyPrime" , query: {number: number, rounds:rounds, type: 0 },}}>
             <button  type='submit' className={styles.button}>Get result</button>
          </Link>
        </form>
    </div>
    </div>
  );
};

export default InputPrimePage;
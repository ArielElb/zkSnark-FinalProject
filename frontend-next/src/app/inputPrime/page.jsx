"use client";
import { useState, useEffect } from "react";
import styles from "../../styles/verify.module.css";
import Link from "next/link";

const InputPrimePage = () => {
  const [currentOption, setCurrentOption] = useState("prove");
  const [number, setNumber] = useState("");
  const [rounds, setRounds] = useState("");

  return (
    <div className={styles.container}>
       <Link href={{pathname: '../information/' , query: {type:"Prime number"},}}>
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
          <h1 className={styles.title}>Prime input</h1>
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

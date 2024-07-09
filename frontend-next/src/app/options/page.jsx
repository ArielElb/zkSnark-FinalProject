"use client";
import Link from "next/link";
import styles from "../../styles/options.module.css";
import { useState, useEffect } from "react";

const OptionsPage = () => {
  return (
    <div className={styles.container}>
      <section className={styles.explanation}>
        <h1>options to vetify</h1>
        <p>
          Prime number - will verify if the number you give is a prime number.<br />
          Fibonacci number - will verify if the number you give is a Fibonacci number.<br />
          Matrix multiplication - will verify a multiplication between 2 matrices.
        </p>
      </section>
      <div className={styles.modal}>
        <h2 className={styles.title}>Choose Option</h2>
        <div className={styles.options}>
          <Link href="../inputPrime" legacyBehavior>
            <button className={styles.option}>Prime Number</button>
          </Link>
          <Link href="../inputFibonachi" legacyBehavior>
            <button className={styles.option}>Fibonacci Number</button>
          </Link>
          <Link href="../inputMatrix" legacyBehavior>
            <button className={styles.option}>Matrix Multiplication</button>
          </Link>
        </div>
      </div>
    </div>
  );
};

export default OptionsPage;

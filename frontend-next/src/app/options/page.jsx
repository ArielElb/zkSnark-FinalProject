"use client";
import Link from "next/link";
import styles from "../../styles/options.module.css";
import { useState, useEffect } from "react";

const OptionsPage = () => {
  return (
    <div className={styles.container}>
      <section className={styles.explanation}>
        <h1>Verification Options</h1>
        <p>
          Choose from the following options to verify different mathematical properties:
        </p>
        <ul>
          <li>Prime number: Verify if the number you provide is a prime number.</li>
          <li>
            Fibonacci number: Verify if the number you provide is a Fibonacci number.
          </li>
          <li>Matrix multiplication: Verify the result of multiplying two matrices.</li>
        </ul>
      </section>
      <div className={styles.modal}>
        <h2 className={styles.title}>Choose Option</h2>
        <div className={styles.options}>
          <Link href='../inputPrime' legacyBehavior>
            <button className={styles.option}>Prime Number</button>
          </Link>
          <Link href='../inputFibonacci' legacyBehavior>
            <button className={styles.option}>Fibonacci Number</button>
          </Link>
          <Link href='../inputMatrix' legacyBehavior>
            <button className={styles.option}>Matrix Multiplication</button>
          </Link>
        </div>
      </div>
    </div>
  );
};

export default OptionsPage;

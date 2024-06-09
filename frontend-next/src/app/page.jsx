"use client";
import Link from "next/link";
import styles from "../styles/Home.module.css";

const HomePage = () => {
  const options = [
    "Prime number",
    "Fibonacci",
    "Factorization",
    "Perfect square",
    "quadratic equation",
    "Proof for Linear Equations",
  ];

  return (
    <div className={styles.backgroundContainer}>
      <div className={styles.container}>
        <h1 className={styles.message}>Please Select the option you want to verify</h1>
        <div className={styles.buttonContainer}>
          {options.map((option, index) => (
            <Link key={index} href={{ pathname: "/verify", query: { option } }} passHref>
              <button className={styles.optionButton}>{option}</button>
            </Link>
          ))}
        </div>
      </div>
    </div>
  );
};

export default HomePage;

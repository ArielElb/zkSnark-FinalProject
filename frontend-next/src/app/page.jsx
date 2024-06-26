"use client";
import Link from "next/link";
import styles from "../styles/Home.module.css";

const HomePage = () => {
  const names = [
    "Prime number",
    "Fibonacci",
    "Factorization",
    "Perfect square",
    "quadratic_equation",
    "Linear Equations",
  ];
  const options = [
    "/inputPrime",
    "/inputFibonachi",
    "/third",
    "/fourth",
    "/fifth",
    "/sixth",
  ];
  return (
    <div className={styles.backgroundContainer}>
      <div className={styles.container}>
        <h1 className={styles.message}>Please Select the option you want to verify</h1>
        <div className={styles.buttonContainer}>
          {names.map((name, index) => (
            <Link key={index} href={{ pathname: options[index], query: {msg : name},}}>
              <button className={styles.optionButton}>{name}</button>
            </Link>
          ))}
        </div>
      </div>
    </div>
  );
};

export default HomePage;

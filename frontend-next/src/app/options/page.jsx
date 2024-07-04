"use client";
import Link from "next/link";
import styles from "../../styles/options.module.css";

const OptionsPage = () => {
  const names = [
    "Prime number",
    "Fibonacci",
    "Matrix multiplication",
    "Perfect square",
    "quadratic_equation",
    "Linear Equations",
  ];
  const options = [
    "../inputPrime",
    "../inputFibonachi",
    "../inputMatrix",
    "../fourth",
    "../fifth",
    "../sixth",
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

export default OptionsPage;

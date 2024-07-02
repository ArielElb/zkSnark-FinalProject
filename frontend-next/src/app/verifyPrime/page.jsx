// src/app/verify/page.jsx
"use client";
import { useState, useEffect } from "react";
import axios from "axios";
import { Line } from "react-chartjs-2";
import { Chart as ChartJS, LineElement, PointElement, LinearScale, Title, CategoryScale } from 'chart.js';
import styles from "../../styles/result.module.css";

ChartJS.register(LineElement, PointElement, LinearScale, Title, CategoryScale);

const VerifyPrimePage = ({ searchParams }) => {
  const [result, setResult] = useState(null);
  const [isLoading, setIsLoading] = useState(true);
  if(searchParams.a) {
      console.log(searchParams.a);
  } else {
    console.log("yaaaaaa");
    useEffect(() => {
      const handleSubmit = async () => {
        try {
          const response = await axios.post("http://localhost:8080/api/prime_sp1", {
            n: parseInt(searchParams.number),
            num_of_rounds: parseInt(searchParams.rounds),
            evm: false
          });  
          console.log(response);
          console.log(response.data);
          setResult(response.data);
          setIsLoading(false);
        } catch (error) {
          console.error("Error submitting the form", error);
          setResult({ error: "Failed to compute. Please try again." });
          setIsLoading(false);
        }
      };
  
      handleSubmit();
    }, [searchParams.number, searchParams.rounds]);
  }
 

  return (
    <div className={styles.container}>
      <div className={styles.leftContainer}>
        <h1 className={styles.title}>Prime Number Verification Results</h1>
        <p className={styles.explanation}>
          Below are the results of your prime number verification request:
        </p>
        {isLoading ? (
          <p className={styles.loadingText}>Calculating...</p>
        ) : (
          <div className={styles.resultContainer}>
            {result.error ? (
              <p className={styles.errorText}>{result.error}</p>
            ) : (
              <>
                <p className={styles.resultText}>n: {result.n}</p>
                <p className={styles.resultText}>num of rounds: {result.num_of_rounds}</p>
                <p className={styles.resultText}>is prime:  {result.is_prime}</p>
                <p className={styles.resultText}>prime: {result.prime}</p>
              </>
            )}
          </div>
        )}
      </div>
      <div className={styles.rightContainer}>
      </div>
    </div>
  );
};

export default VerifyPrimePage;

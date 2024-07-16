// src/app/verify/page.jsx
"use client";
import { useState, useEffect } from "react";
import styles from "../../styles/result.module.css";

const VerifyPrimePage = ({ searchParams }) => {
  
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
            {provResult.error ? (
              <p className={styles.errorText}>{provResult.error}</p>
            ) : (
              <div className={styles.resultContainer}>
                {isVerifying ? (
                  <>
                    <p className={styles.resultText}>
                      proving time: {provResult.proving_time}
                    </p>
                    <p className={styles.loadingText}>verifing...</p>
                  </>
                ) : (
                  <>
                    <p className={styles.resultText}>
                      proving time: {provResult.proving_time}
                    </p>
                    <p className={styles.resultText}>
                      the answer is: {verifyResult.answer}
                    </p>
                    <p className={styles.resultText}>
                      verifying time: {verifyResult.verifying_time}
                    </p>
                  </>
                )}
              </div>
            )}
          </div>
        )}
      </div>
      <div className={styles.rightContainer}></div>
    </div>
  );
};

export default VerifyPrimePage;

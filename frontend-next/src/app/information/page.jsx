// pages/index.js
"use client";
import Head from 'next/head';
import styles from '../../styles/information.module.css';
import Link from "next/link";

const InformationPage = ({ searchParams }) => {
  const handleBack = () => {
    window.history.back();
  };
    return (
    <div className={styles.container}>
      <Head>
        <title>Information Page</title>
        <meta name="description" content="Proof of Concept for zkProofs" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className={styles.main}>
       <button onClick={handleBack} className={styles.topRightButton}>back to verify</button>
        <h1 className={styles.title}>information about {searchParams.type}</h1>

        <h2 className={styles.subtitle}>What is our project all about?</h2>

        <p className={styles.description}>
          Our project is a Proof of Concept (PoC) for implementing zero-knowledge proofs (zkProofs) using the SP1 framework and arkworks.rs. 
          We are demonstrating zkProofs for various computational tasks, including primality tests, matrix multiplication, and Fibonacci sequence computation. 
          Our project leverages zkProofs to ensure the integrity and efficiency of computations while holding the Zero-Knowledge property.
        </p>

        <div className={styles.imageContainer}>
          <div className={styles.imageRow}>
            <img src="/logo.jpg" alt="Image 1" className={styles.image} />
            <img src="/logo.jpg" alt="Image 2" className={styles.image} />
          </div>
          <div className={styles.imageRow}>
            <img src="/logo.jpg" alt="Image 3" className={styles.image} />
            <img src="/logo.jpg" alt="Image 4" className={styles.image} />
          </div>
        </div>
      </main>
    </div>
  );
};

export default InformationPage;

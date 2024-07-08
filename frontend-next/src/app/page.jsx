import Head from 'next/head';
import styles from '../styles/Home.module.css';

export default function Home() {
  return (
    <>
      <div className={styles.container}>
        <header className={styles.header}>
            
          <div className={styles.headerParagraph}>
          <h1>supabase / NEXT.js</h1>
            Zero-Knowledge Proofs (zkProofs) are cryptographic protocols that allow one
            party (the prover) to prove to another party (the verifier) that they know a
            value or statement without revealing any information about the value itself.
            This project leverages the SP1 framework and{' '}
            <a href='https://arkworks.rs/' target='_blank' rel='noopener noreferrer' className={styles.link}>
              arkworks.rs
            </a>.
            ffffffffffffffffff
            fffffffffffffffffff
            ffffffffffffffffffff
            fffffffffffffffffffff
            fffffffffffffffffffffffffffffffffffffffffffff
            ffffffffffffffffff
            fffffffffffffffffff
            ffffffffffffffffffff
            fffffffffffffffffffff
            fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
            fffffffffffffffffff
            ffffffffffffffffffff
            fffffffffffffffffffff
            fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
            fffffffffffffffffff
            ffffffffffffffffffff
            fffffffffffffffffffff
            fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
            fffffffffffffffffff
            ffffffffffffffffffff
            fffffffffffffffffffff
            fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
            fffffffffffffffffff
            ffffffffffffffffffff
            fffffffffffffffffffff
            fffffffffffffffffffffffffffffffffffffffffffff
          </div>
        </header>
        <main className={styles.main}>
          <div className={styles.row}>
            <div className={styles.card}>
              <h2>Cookie-based Auth and the Next.js App Router</h2>
              <p>
                This free course by Jon Meyers, shows you how to configure Supabase Auth to use cookies, and steps through some common patterns.
              </p>
            </div>
            <div className={styles.card}>
              <h2>Supabase Next.js App Router Example</h2>
              <p>
                Want to see a code example containing some common patterns with Next.js and Supabase? Check out this repo!
              </p>
            </div>
          </div>
          <div className={styles.row}>
            <div className={styles.card}>
              <h2>Supabase Auth Helpers Docs</h2>
              <p>
                This template has configured Supabase Auth to use cookies for you, but the docs are a great place to learn more.
              </p>
            </div>
            <div className={styles.card}>
              <h2>Next.js Front-end for zkProofs</h2>
              <p>
                The Next.js web front-end provides an intuitive interface for users to
                interact with these zkProofs, demonstrating the practical application of
                cryptographic protocols in a user-friendly environment.
              </p>
            </div>
          </div>
        </main>
      </div>
    </>
  );
}

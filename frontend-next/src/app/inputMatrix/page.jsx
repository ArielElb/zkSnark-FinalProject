"use client";
import { useState, useEffect } from "react";
import styles from "../../styles/matrix.module.css";
import Link from "next/link";

import axios from "axios";

const InputMatrixPage = () => {
  const [size, setSize] = useState(2);
  const [matrix1, setMatrix1] = useState(generateRandomMatrix(2));
  const [matrix2, setMatrix2] = useState(generateRandomMatrix(2));
  const [savedMatrix1, setSavedMatrix1] = useState(matrix1);
  const [savedMatrix2, setSavedMatrix2] = useState(matrix2);
  const [currentOption, setCurrentOption] = useState("prove");
  const [verifyHashes, setVerifyHashes] = useState({
    hash_a: "",
    hash_b: "",
    hash_c: "",
  });
  const [verifyHashA,setVerifyHashA]=useState("");
  const [verifyHashB,setVerifyHashB]=useState("");
  const [verifyHashC,setVerifyHashC]=useState("");
  const [hashes, setHashes] = useState({ hash_a: "", hash_b: "", hash_c: "" });
  const [verifyResult, setVerifyResult] = useState("");
  const [verifyingTime, setVerifyingTime] = useState("");
  const [isLoadingProof, setIsLoadingProof] = useState(false);
  const [isLoadingVerify, setIsLoadingVerify] = useState(false);
  const [setupTime, setSetupTime] = useState(null);
  const [provingTime, setProvingTime] = useState(null);
  const [numConstraints, setNumConstraints] = useState(null);
  const [numVariables, setNumVariables] = useState(null);
  const [recProof,setRecProof] = useState("");


  useEffect(() => {
    setSavedMatrix1(matrix1);
  }, [matrix1]);

  useEffect(() => {
    setSavedMatrix2(matrix2);
  }, [matrix2]);

  // Function to generate random matrix of given size
  function generateRandomMatrix(size) {
    return Array(size)
      .fill("")
      .map(
        () =>
          Array(size)
            .fill("")
            .map(() => Math.floor(Math.random() * 10)) // Adjust range as needed
      );
  }

  const handleSizeChange = (e) => {
    const newSize = parseInt(e.target.value);
    setSize(newSize);
    setMatrix1(generateRandomMatrix(newSize));
    setMatrix2(generateRandomMatrix(newSize));
  };

  const handleInputChange = (matrix, setMatrix, i, j, e) => {
    const value = e.target.value;
    if (!isNaN(value)) {
      const newMatrix = matrix.map((row, rowIndex) =>
        rowIndex === i
          ? row.map((cell, cellIndex) => (cellIndex === j ? value : cell))
          : row
      );
      setMatrix(newMatrix);
    }
  };

  const handleProve = () => {
    // Reset previous stats
    setSetupTime(null);
    setProvingTime(null);
    setNumConstraints(null);
    setNumVariables(null);

    const convertToNumbers = (matrix) =>
      matrix.map((row) => row.map((cell) => parseFloat(cell)));

    const requestData = {
      size,
      matrix_a: convertToNumbers(savedMatrix1),
      matrix_b: convertToNumbers(savedMatrix2),
    };

    setIsLoadingProof(true);
    axios
      .post("http://127.0.0.1:8080/api/matrix_prove/prove", requestData)
      .then((response) => {
        console.log("Matrix prove response:", response.data);
        const {
          proof,
          pvk,
          hash_a,
          hash_b,
          hash_c,
          setup_time,
          proving_time,
          num_constraints,
          num_variables,
        } = response.data;
        localStorage.setItem("proof", JSON.stringify(proof));
        localStorage.setItem("pvk", JSON.stringify(pvk));
        setHashes({ hash_a, hash_b, hash_c });
        setRecProof(proof)
        localStorage.setItem("hash_a", JSON.stringify(hash_a));
        localStorage.setItem("hash_b", JSON.stringify(hash_b));
        localStorage.setItem("hash_c", JSON.stringify(hash_c));

        setSetupTime(setup_time);
        setProvingTime(proving_time);
        setNumConstraints(num_constraints);
        setNumVariables(num_variables);
      })
      .catch((error) => {
        console.error("Error proving matrices:", error);
      })
      .finally(() => {
        setIsLoadingProof(false);
      });
  };

  const handleVerify = () => {
    const pvk = JSON.parse(localStorage.getItem("pvk"));
    const proof = JSON.parse(localStorage.getItem("proof"));

    const requestData = {
      pvk,
      proof,
      hash_a: verifyHashA,
      hash_b: verifyHashB,
      hash_c: verifyHashC,
    };
    console.log(requestData);
    setIsLoadingVerify(true);
    axios
      .post("http://127.0.0.1:8080/api/matrix_prove/verify", requestData)
      .then((response) => {
        console.log("Matrix verify response:", response.data);
        const { valid, verifying_time } = response.data;
        if (valid) {
          setVerifyResult("Verification successful!");
        } else {
          setVerifyResult("Verification failed.");
        }
        setVerifyingTime(verifying_time);
      })
      .catch((error) => {
        console.error("Error verifying matrices:", error);
        setVerifyResult("Verification failed.");
        setVerifyingTime("");
      })
      .finally(() => {
        setIsLoadingVerify(false);
      });
  };

  const renderMatrixInput = (matrix, setMatrix) => (
    <div className={styles.matrix}>
      {matrix.map((row, i) => (
        <div key={i} className={styles.row}>
          {row.map((cell, j) => (
            <input
              key={j}
              type='text'
              value={cell}
              onChange={(e) => handleInputChange(matrix, setMatrix, i, j, e)}
              className={styles.cell}
            />
          ))}
        </div>
      ))}
    </div>
  );

  const AdditionalInfo = () => (
    <div className={styles.additionalInfo}>
      {setupTime !== null && <p>Setup Time: {setupTime.toFixed(6)} seconds</p>}
      {provingTime !== null && <p>Proving Time: {provingTime.toFixed(6)} seconds</p>}
      {numConstraints !== null && <p>Number of Constraints: {numConstraints}</p>}
      {numVariables !== null && <p>Number of Variables: {numVariables}</p>}
    </div>
  );

  return (
    <div className={styles.container}>
       <Link href={{pathname: '../information/' , query: {type:"matrix multification"},}}>
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
          onClick={() => {setCurrentOption("verify");
          if (localStorage.getItem("hash_a")) {
          setVerifyHashA(JSON.parse(localStorage.getItem("hash_a")));
            }
            if (localStorage.getItem("hash_b")) {
              setVerifyHashB(JSON.parse(localStorage.getItem("hash_b")));
                }
                if (localStorage.getItem("hash_c")) {
                  setVerifyHashC(JSON.parse(localStorage.getItem("hash_c")));
                    }
          }}
          className={currentOption === "verify" ? styles.activeButton : ""}
        >
          Verify
        </button>
      </div>

      {currentOption === "prove" && (
        <>
          <h1 className={styles.title}>Matrix Input</h1>
          <label className={styles.label}>
            Select matrix size (2-40):
            <select value={size} onChange={handleSizeChange} className={styles.select}>
              {[...Array(39).keys()].map((s) => (
                <option key={s + 2} value={s + 2}>
                  {s + 2}
                </option>
              ))}
            </select>
          </label>
          <div className={styles.matrices}>
            {renderMatrixInput(matrix1, setMatrix1)}
            {renderMatrixInput(matrix2, setMatrix2)}
          </div>
          <button onClick={handleProve} className={styles.saveButton}>
            Prove
          </button>
          {isLoadingProof && (
            <div className={styles.loading}>
              <p>Loading proof...</p>
            </div>
          )}
          {hashes.hash_a && !isLoadingProof && (
            <div className={styles.hashContainer}>
              <h2>Proof Hashes:</h2>
              <p>Hash A: {hashes.hash_a}</p>
              <p>Hash B: {hashes.hash_b}</p>
              <p>Hash C: {hashes.hash_c}</p>
              <p>The proof: {recProof.substring(0, 120)}</p>
              <p>{recProof.substring(120, 300)}</p>


            </div>
          )}
          <AdditionalInfo /> {/* Render additional information component */}
        </>
      )}

      {currentOption === "verify" && (
        
        <div className={styles.option2Container}>
          <h1 className={styles.title}>Verify Proof</h1>
          <label className={styles.label}>
            Enter Hash A:
            <input
              type='text'
              value={JSON.parse(localStorage.getItem("hash_a"))}
              onChange={(e) =>
                setVerifyHashA( e.target.value )
              }
              className={styles.inputBox}
            />
          </label>
          <label className={styles.label}>
            Enter Hash B:
            <input
              type='text'
              value={JSON.parse(localStorage.getItem("hash_b"))}
              onChange={(e) =>
                setVerifyHashB( e.target.value )
              }
              className={styles.inputBox}
            />
          </label>
          <label className={styles.label}>
            Enter Hash C:
            <input
              type='text'
              value={JSON.parse(localStorage.getItem("hash_c"))}
              onChange={(e) =>
                setVerifyHashC( e.target.value )
              }
              className={styles.inputBox}
            />
          </label>
          <button onClick={handleVerify} className={styles.saveButton}>
            Verify
          </button>
          {isLoadingVerify && (
            <div className={styles.loading}>
              <p>Verifying...</p>
            </div>
          )}
          {verifyResult && !isLoadingVerify && (
            <div className={styles.verifyResult}>
              <h2>Verification Result:</h2>
              <p>{verifyResult}</p>
              <p>Verifying Time: {verifyingTime}</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

export default InputMatrixPage;

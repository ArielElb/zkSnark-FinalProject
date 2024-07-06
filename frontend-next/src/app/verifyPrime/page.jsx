// src/app/verify/page.jsx
"use client";
import { useState, useEffect } from "react";
import axios from "axios";
import styles from "../../styles/result.module.css";

const VerifyPrimePage = ({ searchParams }) => {
  const [provResult, setProveResult] = useState(null);
  const [verifyResult, setVerifyResult] = useState(null);

  const [isLoading, setIsLoading] = useState(true);
  const [isVerifying,setIsVerifying] = useState(true);
    if(searchParams.type==0) {//it is prime

    } else {

    if(searchParams.type==1) { //it is fibonacci
    useEffect(() => {
    const handleSubmit = async () => {
      try {
        const response = await axios.post("http://127.0.0.1:8080/api/fibbonaci/prove", {
          a: parseInt(searchParams.a),
          b: parseInt(searchParams.b),
          result: parseInt(searchParams.number),
          num_of_rounds: parseInt(searchParams.rounds),
        });  
        
        setProveResult(response.data);
        setIsLoading(false);
        console.log("print1");
        console.log(response.data.proof);
        console.log(response.data.pvk);
        console.log(response.data);

        const response2 = await axios.post("http://127.0.0.1:8080/api/fibbonaci/verify", {
          proof: response.data.proof,
          pvk: response.data.vk,
          a: parseInt(searchParams.a),
          b: parseInt(searchParams.b),
        });  
        console.log(response2);
        console.log(response2.data);
        console.log("print2")
        setVerifyResult(response2.data);
        setIsVerifying(false);
        if (response2.data.is_res==1) {
          setVerifyResult(prevData => ({
            ...prevData,
            answer: 'true'
          }));
        }else {
          setVerifyResult(prevData => ({
            ...prevData,
            answer: 'false'
          }));
        }
      } catch (error) {
        console.log("print3")
        console.error("Error submitting the form", error);
        setProveResult({ error: "Failed to compute. Please try again." });
        setIsLoading(false);
      }
    };

       handleSubmit(); 
      }, [searchParams.number, searchParams.rounds]);
  } else {
    if(searchParams.type==2){ //matrix option
      useEffect(() => {
        const handleSubmit = async () => {
          try {
            console.log(localStorage.getItem('matrix_1'))
            console.log(localStorage.getItem('size'));
            const response = await axios.post("http://127.0.0.1:8080/api/matrix_prove/prove", {
              size: parseInt(localStorage.getItem('size')),
              matrix_a: localStorage.getItem('matrix_1'),
              matrix_b: localStorage.getItem('matrix_2'),
            });  
            
            setProveResult(response.data);
            setIsLoading(false);
            console.log("print1");
            console.log(response.data.proof);
            console.log(response.data.pvk);
            console.log(response.data);
    
            const response2 = await axios.post("http://127.0.0.1:8080/api/matrix_prove/verify", {
              pvk: response.data.pvk,
              proof: response.data.proof,
              hash: response.data.hash,
            });  
            console.log(response2);
            console.log(response2.data);
            console.log("print2")
            setVerifyResult(response2.data);
            setIsVerifying(false);
            if (response2.data.is_res==1) {
              setVerifyResult(prevData => ({
                ...prevData,
                answer: 'true'
              }));
            }else {
              setVerifyResult(prevData => ({
                ...prevData,
                answer: 'false'
              }));
            }
          } catch (error) {
            console.log("print3")
            console.error("Error submitting the form", error);
            setProveResult({ error: "Failed to compute. Please try again." });
            setIsLoading(false);
          }
        };
    
           handleSubmit(); 
          }, [searchParams.number, searchParams.rounds]);
    }
  }
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
            {provResult.error ? (
              <p className={styles.errorText}>{provResult.error}</p>
            ) :  (
              <div className={styles.resultContainer}>
              {isVerifying ? (
                <>
                <p className={styles.resultText}>proving time: {provResult.proving_time}</p>
                <p className={styles.loadingText}>verifing...</p> 
                </>  ) :  (
                <>
                <p className={styles.resultText}>proving time: {provResult.proving_time}</p>
                <p className={styles.resultText}>the answer is: {verifyResult.answer}</p> 
                <p className={styles.resultText}>verifying time: {verifyResult.verifying_time}</p> 
                </>
              )}
            </div>
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

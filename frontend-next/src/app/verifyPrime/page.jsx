// pages/index.js
"use client";

import React, { useState } from "react";
import axios from "axios";

export default function Home() {
  const [x, setX] = useState("");
  const [numOfRounds, setNumOfRounds] = useState("");
  const [result, setResult] = useState(null);

  const handleSubmit = async (event) => {
    event.preventDefault();
    try {
      const response = await axios.post("http://localhost:8080/api/prime_snark", {
        x: parseInt(x),
        num_of_rounds: parseInt(numOfRounds),
      });
      setResult(response.data);
    } catch (error) {
      console.error("Error submitting the form", error);
      setResult({ error: "Failed to compute. Please try again." });
    }
  };

  return (
    <div>
      <h1>Prime Circuit</h1>
      <form onSubmit={handleSubmit}>
        <label>
          Enter x:
          <input
            type='number'
            value={x}
            onChange={(e) => setX(e.target.value)}
            required
          />
        </label>
        <br />
        <br />
        <label>
          Enter number of rounds:
          <input
            type='number'
            value={numOfRounds}
            onChange={(e) => setNumOfRounds(e.target.value)}
            required
          />
        </label>
        <br />
        <br />
        <button type='submit'>Submit</button>
      </form>
      {result && (
        <div>
          <h2>Result</h2>
          {result.error ? (
            <p>{result.error}</p>
          ) : (
            <>
              <p>Proof: {result.proof}</p>
              <p>Public Input: {result.public_input.join(", ")}</p>
              <p>Number of Constraints: {result.num_constraints}</p>
              <p>Number of Variables: {result.num_variables}</p>
              <p>Proving Time: {result.proving_time} seconds</p>
              <p>Verifying Time: {result.verifying_time} seconds</p>
            </>
          )}
        </div>
      )}
    </div>
  );
}

/*
 
*/

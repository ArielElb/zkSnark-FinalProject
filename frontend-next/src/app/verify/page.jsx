// pages/verify.jsx
"use client";
import { useState } from "react";
import axios from "axios";

const VerifyPage = () => {
  const [number, setNumber] = useState("");
  const [rounds, setRounds] = useState("");
  const [result, setResult] = useState(null);

  const handleSubmit = async (e) => {
    e.preventDefault();
    try {
      const response = await axios.post("http://localhost:8080/api/prime_snark", {
        x: parseInt(number),
        num_of_rounds: parseInt(rounds),
      });
      setResult(response.data);
    } catch (error) {
      console.error("Error submitting the form", error);
      setResult({ error: "Failed to compute. Please try again." });
    }
  };

  return (
    <div>
      <header>
        <form onSubmit={handleSubmit}>
          <div>
            <label>
              Get Number:
              <input
                type='number'
                value={number}
                onChange={(e) => setNumber(e.target.value)}
              />
            </label>
          </div>
          <div>
            <label>
              Get Number of Rounds:
              <input
                type='number'
                value={rounds}
                onChange={(e) => setRounds(e.target.value)}
              />
            </label>
          </div>
          <button type='submit'>Submit</button>
        </form>
      </header>
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
};

export default VerifyPage;

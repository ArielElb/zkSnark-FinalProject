import { useRouter } from "next/router";
import { useState } from "react";

const VerifyPage = () => {
  const router = useRouter();
  const { option } = router.query;
  const [number, setNumber] = useState("");
  const [rounds, setRounds] = useState("");

  const handleSubmit = (e) => {
    e.preventDefault();
    console.log("Number:", number);
    console.log("Number of Rounds:", rounds);
    // Add additional logic for prime number check or any other operation.
  };

  return (
    <div>
      <header>
        <h1>{option}</h1>
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
    </div>
  );
};

export default VerifyPage;

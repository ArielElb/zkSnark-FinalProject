"use client";
import { useState } from "react";
import styles from "../../styles/matrix.module.css";

const InputMatrixPage = () => {
  const [size, setSize] = useState(2);
  const [matrix1, setMatrix1] = useState(Array(2).fill('').map(() => Array(2).fill('')));
  const [matrix2, setMatrix2] = useState(Array(2).fill('').map(() => Array(2).fill('')));
  const [savedMatrix1, setSavedMatrix1] = useState(null);
  const [savedMatrix2, setSavedMatrix2] = useState(null);
  const [currentOption, setCurrentOption] = useState("option1");

  const handleSizeChange = (e) => {
    const newSize = parseInt(e.target.value);
    setSize(newSize);
    setMatrix1(Array(newSize).fill('').map(() => Array(newSize).fill('')));
    setMatrix2(Array(newSize).fill('').map(() => Array(newSize).fill('')));
  };

  const handleInputChange = (matrix, setMatrix, i, j, e) => {
    const value = e.target.value;
    if (!isNaN(value)) {
      const newMatrix = matrix.map((row, rowIndex) =>
        rowIndex === i ? row.map((cell, cellIndex) => (cellIndex === j ? value : cell)) : row
      );
      setMatrix(newMatrix);
    }
  };

  const handleSaveMatrix = () => {
    setSavedMatrix1(matrix1);
    setSavedMatrix2(matrix2);
    console.log('Matrix 1 saved:', matrix1);
    console.log('Matrix 2 saved:', matrix2);
  };

  const renderMatrixInput = (matrix, setMatrix) => (
    <div className={styles.matrix}>
      {matrix.map((row, i) => (
        <div key={i} className={styles.row}>
          {row.map((cell, j) => (
            <input
              key={j}
              type="text"
              value={cell}
              onChange={(e) => handleInputChange(matrix, setMatrix, i, j, e)}
              className={styles.cell}
            />
          ))}
        </div>
      ))}
    </div>
  );

  return (
    <div className={styles.container}>
      <div className={styles.optionButtons}>
        <button onClick={() => setCurrentOption("option1")} className={currentOption === "option1" ? styles.activeButton : ''}>Option 1</button>
        <button onClick={() => setCurrentOption("option2")} className={currentOption === "option2" ? styles.activeButton : ''}>Option 2</button>
      </div>

      {currentOption === "option1" && (
        <>
          <h1 className={styles.title}>Matrix Input</h1>
          <label className={styles.label}>
            Select matrix size (2-8):
            <select value={size} onChange={handleSizeChange} className={styles.select}>
              {[2, 3, 4, 5, 6, 7, 8].map((s) => (
                <option key={s} value={s}>
                  {s}
                </option>
              ))}
            </select>
          </label>
          <div className={styles.matrices}>
            {renderMatrixInput(matrix1, setMatrix1)}
            {renderMatrixInput(matrix2, setMatrix2)}
          </div>
          <button onClick={handleSaveMatrix} className={styles.saveButton}>
            Save Matrix
          </button>
          {savedMatrix1 && savedMatrix2 && (
            <div className={styles.savedMatrix}>
              <h2>Saved Matrices:</h2>
              <div className={styles.matrices}>
                <div>
                  <h3>Matrix 1:</h3>
                  {savedMatrix1.map((row, i) => (
                    <div key={i} className={styles.row}>
                      {row.map((cell, j) => (
                        <span key={j} className={styles.savedCell}>
                          {cell}
                        </span>
                      ))}
                    </div>
                  ))}
                </div>
                <div>
                  <h3>Matrix 2:</h3>
                  {savedMatrix2.map((row, i) => (
                    <div key={i} className={styles.row}>
                      {row.map((cell, j) => (
                        <span key={j} className={styles.savedCell}>
                          {cell}
                        </span>
                      ))}
                    </div>
                  ))}
                </div>
              </div>
            </div>
          )}
        </>
      )}

      {currentOption === "option2" && (
        <div className={styles.option2Container}>
          <h1 className={styles.title}>Option 2 Page</h1>
          <input type="text" className={styles.inputBox} />
          <button className={styles.saveButton}>Submit</button>
        </div>
      )}
    </div>
  );
};

export default InputMatrixPage;

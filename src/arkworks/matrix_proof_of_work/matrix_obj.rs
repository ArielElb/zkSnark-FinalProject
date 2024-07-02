use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    // Create a new matrix
    fn new(rows: usize, cols: usize, data: Vec<Vec<f64>>) -> Self {
        assert!(data.len() == rows && data.iter().all(|r| r.len() == cols));
        Matrix { rows, cols, data }
    }

    // Create a matrix with random values
    fn random(rows: usize, cols: usize) -> Self {
        let mut rng = rand::thread_rng();
        let data: Vec<Vec<f64>> = (0..rows)
            .map(|_| (0..cols).map(|_| rng.gen_range(0.0..10.0)).collect())
            .collect();
        Matrix { rows, cols, data }
    }

    // Add two matrices
    fn add(&self, other: &Matrix) -> Matrix {
        assert!(self.rows == other.rows && self.cols == other.cols);
        let mut result = vec![vec![0.0; self.cols]; self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Matrix::new(self.rows, self.cols, result)
    }

    // Subtract two matrices
    fn sub(&self, other: &Matrix) -> Matrix {
        assert!(self.rows == other.rows && self.cols == other.cols);
        let mut result = vec![vec![0.0; self.cols]; self.rows];
        for i in 0..self.rows {
            for j in 0..self.cols {
                result[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        Matrix::new(self.rows, self.cols, result)
    }

    // Multiply two matrices
    fn mul(&self, other: &Matrix) -> Matrix {
        assert!(self.cols == other.rows);
        let mut result = vec![vec![0.0; other.cols]; self.rows];
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    result[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        Matrix::new(self.rows, other.cols, result)
    }

    // Display the matrix
    fn display(&self) {
        for row in &self.data {
            for val in row {
                print!("{:.2} ", val);
            }
            println!();
        }
    }

    // Convert matrix to JSON string
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // Create matrix from JSON string
    fn from_json(json: &str) -> Matrix {
        serde_json::from_str(json).unwrap()
    }
}

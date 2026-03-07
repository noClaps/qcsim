use std::ops::{Add, Mul};

use crate::complex::Complex;

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<Complex>>);

impl Matrix {
    pub fn tensor_prod(&self, mat: &Self) -> Self {
        let out_size = self.0.len() * mat.0.len();
        let mut output = vec![vec![Complex::ZERO; out_size]; out_size];

        for (i, row1) in self.0.iter().enumerate() {
            for (j, &val1) in row1.iter().enumerate() {
                for (k, row2) in mat.0.iter().enumerate() {
                    for (l, &val2) in row2.iter().enumerate() {
                        output[i * mat.0.len() + k][j * row2.len() + l] = val1 * val2;
                    }
                }
            }
        }

        Self(output)
    }

    pub fn tensor_prod_list(matrices: &[Self]) -> Self {
        let mut output = matrices[0].clone();
        for mat in &matrices[1..] {
            output = output.tensor_prod(mat)
        }

        output.clone()
    }
}

impl Mul<&Vec<Complex>> for Matrix {
    type Output = Vec<Complex>;
    fn mul(self, rhs: &Vec<Complex>) -> Self::Output {
        let mut out_vec = vec![Complex::ZERO; rhs.len()];
        for (r, row) in out_vec.iter_mut().enumerate().take(self.0.len()) {
            for (c, &val) in rhs.iter().enumerate().take(self.0[r].len()) {
                *row += self.0[r][c] * val;
            }
        }
        out_vec
    }
}

impl Add for Matrix {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = vec![vec![Complex::ZERO; self.0.len()]; self.0.len()];
        for (r, row) in sum.iter_mut().enumerate().take(self.0.len()) {
            for (c, val) in row.iter_mut().enumerate().take(self.0[r].len()) {
                *val = self.0[r][c] + rhs.0[r][c];
            }
        }
        Self(sum)
    }
}

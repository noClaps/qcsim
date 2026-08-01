use std::ops::{Add, Index, IndexMut, Mul};

use num_complex::Complex64;

use crate::Matrix;

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Self::zero(self.size());
        for i in 0..self.size() {
            for j in 0..rhs.size() {
                let mut sum = Complex64::ZERO;
                for k in 0..self.size() {
                    sum += self[i][k] * rhs[k][j];
                }
                out[i][j] = sum;
            }
        }
        out
    }
}
impl Mul<&Vec<Complex64>> for Matrix {
    type Output = Vec<Complex64>;
    fn mul(self, rhs: &Vec<Complex64>) -> Self::Output {
        let mut out_vec = vec![Complex64::ZERO; rhs.len()];
        for (r, row) in out_vec.iter_mut().enumerate() {
            for (c, &val) in rhs.iter().enumerate() {
                *row += self[r][c] * val;
            }
        }
        out_vec
    }
}
impl Mul<Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut out = Matrix::zero(rhs.size());
        for (i, row) in rhs.0.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                out[i][j] = val * self;
            }
        }
        out
    }
}
impl Mul<Matrix> for Complex64 {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut out = Matrix::zero(rhs.size());
        for (i, row) in rhs.0.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                out[i][j] = val * self;
            }
        }
        out
    }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Matrix::zero(self.size());
        for (r, row) in sum.0.iter_mut().enumerate() {
            for (c, val) in row.iter_mut().enumerate() {
                *val = self[r][c] + rhs[r][c];
            }
        }
        sum
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<Complex64>;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

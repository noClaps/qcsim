use std::{
    ops::{Index, IndexMut, Mul, Sub},
    slice::Iter,
};

use crate::{Complex, StateVector};

/// A type representing a square matrix
pub struct Matrix(Vec<Vec<Complex>>);

/// Create a [`Matrix`]
#[macro_export]
macro_rules! matrix {
    ($([$($el:expr),+ $(,)?]),+ $(,)?) => {
        $crate::Matrix::from(vec![
            $(vec![$($el,)+],)+
        ])
    };
}

impl Matrix {
    /// Dot product between a [`Matrix`] and a [`StateVector`]
    pub fn dot(&self, state_vector: &StateVector) -> StateVector {
        assert_eq!(self[0].len(), state_vector.len());
        let mut out = StateVector::zeros(state_vector.len());
        for (i, row) in self.iter().enumerate() {
            for (j, &el) in row.iter().enumerate() {
                out[i] += el * state_vector[j];
            }
        }
        out
    }
    /// Initialise a `size` by `size` [`Matrix`] with [`Complex::ZERO`] in each element
    pub fn zeros(size: usize) -> Self {
        Self(vec![vec![Complex::ZERO; size]; size])
    }
    /// Initialise a [`Matrix`] with `elems` on the diagonal
    pub fn from_diag(elems: &[Complex]) -> Self {
        let mut base = Self::zeros(elems.len());
        for (i, &el) in elems.iter().enumerate() {
            base[(i, i)] = el;
        }
        base
    }
    /// Convert into an iterator over the rows
    pub fn iter(&self) -> Iter<'_, Vec<Complex>> {
        self.0.iter()
    }
    /// Get the size of the matrix
    pub const fn size(&self) -> usize {
        self.0.len()
    }
    /// Initialise a [`Matrix`] with [`Complex::ONE`] along the diagonal
    pub fn eye(size: usize) -> Self {
        Self::from_diag(&vec![Complex::ONE; size])
    }
}

impl From<Vec<Vec<Complex>>> for Matrix {
    fn from(value: Vec<Vec<Complex>>) -> Self {
        for row in &value {
            assert_eq!(value.len(), row.len())
        }
        Self(value)
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = Complex;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}
impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}
impl Index<usize> for Matrix {
    type Output = Vec<Complex>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Mul<f64> for Matrix {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut out = Self::zeros(self.size());
        for (i, row) in self.iter().enumerate() {
            for (j, &el) in row.iter().enumerate() {
                out[(i, j)] = el * rhs;
            }
        }
        out
    }
}

impl Sub for Matrix {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size(), rhs.size());
        let mut out = Matrix::zeros(self.size());
        for (i, row) in self.iter().enumerate() {
            for (j, &el) in row.iter().enumerate() {
                out[(i, j)] = el - rhs[(i, j)];
            }
        }
        out
    }
}

use std::ops::{Add, Index, IndexMut, Mul, Sub};

use crate::{
    Complex,
    matrix::{Dot, Matrix, diagonal::DiagonalMatrix},
    state_vector::StateVector,
};

#[derive(Clone, PartialEq)]
pub struct SquareMatrix {
    elems: Vec<Complex>,
    size: usize,
}

impl SquareMatrix {
    #[must_use]
    pub fn is_diagonal(&self) -> bool {
        for i in 0..self.size {
            for j in 0..self.size {
                if i != j && self[(i, j)] != Complex::ZERO {
                    return false;
                }
            }
        }
        true
    }
    #[must_use]
    pub fn into_diag(&self) -> DiagonalMatrix {
        let mut elems = vec![Complex::ZERO; self.size];
        for i in 0..self.size {
            elems[i] = self[(i, i)];
        }
        DiagonalMatrix {
            elems,
            size: self.size,
        }
    }
    #[must_use]
    pub fn kron(&self, rhs: &Self) -> Self {
        let mut out = SquareMatrix::zeros(self.size * rhs.size);
        for i in 0..self.size {
            for j in 0..self.size {
                for k in 0..rhs.size {
                    for l in 0..rhs.size {
                        out[(i * rhs.size + k, j * rhs.size + l)] = self[(i, j)] * rhs[(k, l)];
                    }
                }
            }
        }
        out
    }
}

impl Matrix for SquareMatrix {
    fn zeros(size: usize) -> Self {
        Self {
            elems: vec![Complex::ZERO; size * size],
            size,
        }
    }
    fn size(&self) -> usize {
        self.size
    }
    fn map(&self, f: impl Fn(&Complex) -> Complex) -> Self {
        Self {
            elems: self.elems.iter().map(f).collect(),
            size: self.size,
        }
    }
    fn t(&self) -> Self {
        let mut out = self.clone();
        for i in 0..self.size {
            for j in 0..self.size {
                out[(j, i)] = self[(i, j)];
            }
        }
        out
    }
    fn eye(size: usize) -> Self {
        let mut elems = vec![Complex::ZERO; size * size];
        for i in 0..size {
            elems[i * size + i] = Complex::ONE;
        }
        Self { elems, size }
    }
}

impl Index<(usize, usize)> for SquareMatrix {
    type Output = Complex;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.elems[index.0 * self.size + index.1]
    }
}

impl IndexMut<(usize, usize)> for SquareMatrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.elems[index.0 * self.size + index.1]
    }
}

impl<const D: usize> From<[[Complex; D]; D]> for SquareMatrix {
    fn from(value: [[Complex; D]; D]) -> Self {
        let size = D;
        let mut elems = vec![Complex::ZERO; size * size];
        for i in 0..size {
            for j in 0..size {
                elems[i * size + j] = value[i][j];
            }
        }
        Self { elems, size }
    }
}

impl Add for SquareMatrix {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.size, rhs.size,
            "Attempted to add matrices of different sizes"
        );
        Self {
            size: self.size,
            elems: self
                .elems
                .iter()
                .zip(rhs.elems.iter())
                .map(|(&l, &r)| l + r)
                .collect(),
        }
    }
}

impl Mul<f64> for SquareMatrix {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            size: self.size,
            elems: self.elems.iter().map(|&c| c * rhs).collect(),
        }
    }
}

impl Sub for SquareMatrix {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.size, rhs.size,
            "Attempted to subtract matrices of different sizes"
        );
        Self {
            size: self.size,
            elems: self
                .elems
                .iter()
                .zip(rhs.elems.iter())
                .map(|(&l, &r)| l - r)
                .collect(),
        }
    }
}

impl Dot for SquareMatrix {
    type Output = Self;
    fn dot(&self, rhs: &Self) -> Self::Output {
        assert_eq!(
            self.size, rhs.size,
            "Cannot dot a matrix of size {} and a matrix of size {}",
            self.size, rhs.size
        );
        let rhs = rhs.t();
        let mut out = SquareMatrix::zeros(self.size);
        for (i, row) in self.elems.chunks_exact(self.size).enumerate() {
            for (j, col) in rhs.elems.chunks_exact(rhs.size).enumerate() {
                out[(i, j)] = row.iter().zip(col.iter()).map(|(&l, &r)| l * r).sum();
            }
        }
        out
    }
}
impl Dot<StateVector> for SquareMatrix {
    type Output = StateVector;
    fn dot(&self, rhs: &StateVector) -> Self::Output {
        assert_eq!(
            self.size,
            rhs.len(),
            "Cannot dot a matrix of size {} and a vector of len {}",
            self.size,
            rhs.len()
        );
        let mut out = StateVector::zeros(rhs.len());
        for (o, row) in out.iter_mut().zip(self.elems.chunks_exact(self.size)) {
            *o = row.iter().zip(rhs.iter()).map(|(&l, &r)| l * r).sum();
        }
        out
    }
}

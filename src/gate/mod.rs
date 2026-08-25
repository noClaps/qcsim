//! <https://quantum.cloud.ibm.com/docs/en/api/qiskit/circuit_library#standard-gates>

mod many_qubit;
mod single_qubit;
mod two_qubit;

use std::ops::{Add, Index, IndexMut, Mul, Sub};

use crate::{Complex, StateVector};

/// A quantum gate
#[derive(Clone, PartialEq)]
pub struct Gate {
    elements: Vec<Complex>,
    size: usize,
}

/// Create a [`Gate`]
#[macro_export]
macro_rules! gate {
    ($([$($el:expr),+ $(,)?]),+ $(,)?) => {
        $crate::Gate::from(vec![
            $(vec![$($crate::Complex::from($el),)+],)+
        ])
    };
}

impl Gate {
    /// Initialise a `size` by `size` [`Gate`] with [`Complex::ZERO`] in each element
    pub fn zeros(size: usize) -> Self {
        Self {
            elements: vec![Complex::ZERO; size * size],
            size,
        }
    }
    /// Initialise a [`Gate`] with `elems` on the diagonal
    pub fn from_diag(elems: &[Complex]) -> Self {
        let mut base = Self::zeros(elems.len());
        for (i, &el) in elems.iter().enumerate() {
            base[(i, i)] = el;
        }
        base
    }
    /// Initialise a [`Gate`] with [`Complex::ONE`] along the diagonal
    pub fn eye(size: usize) -> Self {
        Self::from_diag(&vec![Complex::ONE; size])
    }
    /// Get the adjoint of a [`Gate`]
    #[doc(alias = "inverse", alias = "dagger")]
    pub fn adjoint(&self) -> Self {
        self.transpose().conjugate()
    }
    /// Create a unitary [`Gate`]
    pub fn unitary(value: Vec<Vec<Complex>>) -> Self {
        let gate = Self::from(value);
        assert!(gate.is_unitary(), "Gate is not unitary");
        gate
    }
    /// Find the Kronecker product between [`self`] and `rhs`.
    /// Also known as the tensor product
    #[doc(alias = "tensor")]
    pub fn kron(&self, rhs: &Self) -> Self {
        let mut out = Gate::zeros(self.size * rhs.size);
        for i in 0..self.size {
            for j in 0..self.size {
                for p in 0..rhs.size {
                    for q in 0..rhs.size {
                        out[(i * rhs.size + p, j * rhs.size + q)] = self[(i, j)] * rhs[(p, q)]
                    }
                }
            }
        }
        out
    }

    fn transpose(&self) -> Self {
        let mut out = Gate::zeros(self.size);
        for i in 0..self.size {
            for j in 0..self.size {
                out[(j, i)] = self[(i, j)];
            }
        }
        out
    }
    fn conjugate(&self) -> Self {
        let mut out = self.clone();
        out.elements.iter_mut().for_each(|el| *el = el.conj());
        out
    }
    fn is_unitary(&self) -> bool {
        let eye = Gate::eye(self.size);
        let gate = self.clone();
        gate.transpose().conjugate() * gate == eye
    }
}

impl From<Vec<Vec<Complex>>> for Gate {
    fn from(value: Vec<Vec<Complex>>) -> Self {
        assert!(
            value.iter().all(|row| value.len() == row.len()),
            "Gate matrix is not square"
        );
        let size = value.len();
        let elements = value.into_iter().flatten().collect();
        Self { elements, size }
    }
}
impl From<Vec<Complex>> for Gate {
    fn from(value: Vec<Complex>) -> Self {
        let size = value.len().isqrt();
        assert_eq!(size * size, value.len(), "Gate matrix is not square");
        Self {
            elements: value,
            size,
        }
    }
}

impl Index<(usize, usize)> for Gate {
    type Output = Complex;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.elements[index.0 * self.size + index.1]
    }
}
impl IndexMut<(usize, usize)> for Gate {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.elements[index.0 * self.size + index.1]
    }
}
impl Index<usize> for Gate {
    type Output = Complex;
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}
impl IndexMut<usize> for Gate {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl Mul<f64> for Gate {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let mut out = self.clone();
        out.elements.iter_mut().for_each(|el| *el *= rhs);
        out
    }
}
impl Mul<Complex> for Gate {
    type Output = Self;
    fn mul(self, rhs: Complex) -> Self::Output {
        let mut out = self.clone();
        out.elements.iter_mut().for_each(|el| *el *= rhs);
        out
    }
}
impl Mul for Gate {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        // https://yaikhom.com/2020-05-26-matrix-multiplication.html
        assert_eq!(self.size, rhs.size);
        let mut out = Self::zeros(self.size);
        let mut a = 0;
        let mut cc = 0;
        for _ in 0..self.size {
            let mut b = 0;
            for _ in 0..self.size {
                for c in (cc..).take(self.size) {
                    out[c] += self[a] * rhs[b];
                    b += 1;
                }
                a += 1;
            }
            cc += self.size;
        }
        out
    }
}
impl Mul<StateVector> for Gate {
    type Output = StateVector;
    fn mul(self, rhs: StateVector) -> Self::Output {
        assert_eq!(self.size, rhs.len());
        let mut out = StateVector::zeros(rhs.len());
        for i in 0..self.size {
            for j in 0..self.size {
                out[i] += self[(i, j)] * rhs[j]
            }
        }
        out
    }
}

impl Sub for Gate {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size);
        let mut out = self.clone();
        out.elements
            .iter_mut()
            .enumerate()
            .for_each(|(i, el)| *el -= rhs[i]);
        out
    }
}

impl Add for Gate {
    type Output = Gate;
    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        out.elements
            .iter_mut()
            .enumerate()
            .for_each(|(i, el)| *el += rhs[i]);
        out
    }
}

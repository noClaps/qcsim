//! <https://quantum.cloud.ibm.com/docs/en/api/qiskit/circuit_library#standard-gates>

mod many_qubit;
mod single_qubit;
mod two_qubit;

use crate::{
    Complex,
    matrix::{Dot, Matrix, diagonal::DiagonalMatrix, matrix, square::SquareMatrix},
};

macro_rules! gate {
    ($([$($el:expr),+ $(,)?]),+ $(,)?) => {
        $crate::matrix::matrix![
            $([$($crate::Complex::from($el),)+],)+
        ]
    };
    ($($el:expr),+ $(,)?) => {
        $crate::matrix::matrix![
            $($crate::Complex::from($el),)+
        ]
    };
}
pub(crate) use gate;

/// A quantum gate
pub enum Gate {
    Normal(SquareMatrix),
    Diagonal(DiagonalMatrix),
}

impl Gate {
    /// Create a [`Gate`] from a unitary matrix
    ///
    /// # Panics
    ///
    /// - Panics if `matrix` is not square.
    /// - Panics if `matrix` order is not a power of 2.
    /// - Panics if `matrix` is not unitary.
    #[must_use]
    pub fn new(matrix: SquareMatrix) -> Self {
        assert!(
            matrix.size().is_power_of_two(),
            "Gate matrix order is not a power of 2"
        );
        let gate = if matrix.is_diagonal() {
            Self::Diagonal(matrix.into_diag())
        } else {
            Self::Normal(matrix)
        };
        assert!(gate.is_unitary(), "Gate is not unitary");
        gate
    }
    /// Get the adjoint of a [`Gate`]
    ///
    /// # Panics
    ///
    /// Panics if the inverse matrix is not square.
    #[doc(alias = "adjoint", alias = "dagger")]
    #[must_use]
    pub fn inverse(&self) -> Self {
        match self {
            Self::Normal(matrix) => {
                let matrix = matrix.t().map(Complex::conj);
                Self::Normal(matrix)
            }
            Self::Diagonal(matrix) => Self::Diagonal(matrix.map(Complex::conj)),
        }
    }
    /// Add `num_qubits` control qubits to the [`Gate`]
    ///
    /// # Panics
    ///
    /// Panics if there is an error during [`Gate`] construction.
    #[must_use]
    pub fn control(&self, num_ctrl_qubits: usize) -> Self {
        match self {
            Self::Normal(matrix) => {
                let mut lhs = matrix![
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, Complex::ZERO]
                ];
                for _ in 0..num_ctrl_qubits {
                    lhs = lhs.kron(&SquareMatrix::eye(2));
                }
                let mut rhs = matrix.clone();
                for _ in 0..num_ctrl_qubits {
                    rhs = matrix![
                        [Complex::ZERO, Complex::ZERO],
                        [Complex::ZERO, Complex::ONE]
                    ]
                    .kron(&rhs);
                }
                Gate::Normal(lhs + rhs)
            }
            Self::Diagonal(matrix) => {
                let mut mat: DiagonalMatrix =
                    vec![matrix![Complex::ONE, Complex::ONE]; num_ctrl_qubits]
                        .into_iter()
                        .flatten()
                        .collect();
                mat.append(matrix);
                Self::Diagonal(mat.clone())
            }
        }
    }
    /// Create a [`Gate`] from its diagonal elements
    ///
    /// # Panics
    ///
    /// Panics if the constructed [`Gate`] is not unitary.
    pub fn from_diag<I>(elements: I) -> Self
    where
        I: IntoIterator<Item = Complex>,
    {
        let gate = Self::Diagonal(DiagonalMatrix::from_iter(elements));
        assert!(gate.is_unitary(), "Gate is not unitary");
        gate
    }

    fn is_unitary(&self) -> bool {
        match self {
            Self::Normal(matrix) => {
                let eye = SquareMatrix::eye(matrix.size());
                let Self::Normal(inv) = self.inverse() else {
                    unreachable!()
                };
                inv.dot(matrix) == eye
            }
            Self::Diagonal(matrix) => matrix.iter().all(|c| (c.norm() - 1.0).abs() < 1e-12),
        }
    }
}

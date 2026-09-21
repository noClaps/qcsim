//! <https://quantum.cloud.ibm.com/docs/en/api/qiskit/circuit_library#standard-gates>

mod many_qubit;
mod single_qubit;
mod two_qubit;

use crate::{Array2, Complex};

use ndarray::{Array1, Axis, array, linalg::kron};

macro_rules! gate {
    ($([$($el:expr),+ $(,)?]),+ $(,)?) => {
        ndarray::array![
            $([$(num_complex::Complex64::from($el),)+],)+
        ]
    };
    ($($el:expr),+ $(,)?) => {
        ndarray::array![
            $(num_complex::Complex64::from($el),)+
        ]
    };
}
pub(crate) use gate;

/// A quantum gate
pub enum Gate {
    Normal(Array2<Complex>),
    Diagonal(Array1<Complex>),
}

impl Gate {
    /// Create a [`Gate`] from a unitary matrix
    pub fn new(matrix: Array2<Complex>) -> Self {
        assert!(matrix.is_square(), "Gate matrix is not square");
        assert!(
            matrix.nrows().is_power_of_two(),
            "Gate matrix order is not a power of 2"
        );
        let gate = if is_diagonal(&matrix) {
            Self::Diagonal(matrix.into_diag())
        } else {
            Self::Normal(matrix)
        };
        assert!(gate.is_unitary(), "Gate is not unitary");
        gate
    }
    /// Get the adjoint of a [`Gate`]
    #[doc(alias = "adjoint", alias = "dagger")]
    pub fn inverse(&self) -> Self {
        match self {
            Self::Normal(matrix) => {
                let matrix = matrix.t().map(|el| el.conj());
                assert!(matrix.is_square(), "Inverse matrix is not square");
                Self::Normal(matrix)
            }
            Self::Diagonal(matrix) => Self::Diagonal(matrix.map(|el| el.conj())),
        }
    }
    /// Add `num_qubits` control qubits to the [`Gate`]
    pub fn control(&self, num_ctrl_qubits: usize) -> Self {
        match self {
            Self::Normal(matrix) => {
                let mut lhs = array![
                    [Complex::ONE, Complex::ZERO],
                    [Complex::ZERO, Complex::ZERO]
                ];
                for _ in 0..num_ctrl_qubits {
                    lhs = kron(&lhs, &Array2::eye(2));
                }
                let mut rhs = matrix.clone();
                for _ in 0..num_ctrl_qubits {
                    rhs = kron(
                        &array![
                            [Complex::ZERO, Complex::ZERO],
                            [Complex::ZERO, Complex::ONE]
                        ],
                        &rhs,
                    );
                }
                Gate::Normal(lhs + rhs)
            }
            Self::Diagonal(matrix) => {
                let mut mat = vec![array![Complex::ONE, Complex::ONE]; num_ctrl_qubits];
                let mat = mat
                    .iter_mut()
                    .reduce(|acc, a| {
                        acc.append(Axis(0), a.view()).unwrap();
                        acc
                    })
                    .unwrap();
                mat.append(Axis(0), matrix.view()).unwrap();
                Self::Diagonal(mat.clone())
            }
        }
    }
    /// Create a [`Gate`] from its diagonal elements
    pub fn from_diag<I>(elements: I) -> Self
    where
        I: IntoIterator<Item = Complex>,
    {
        let gate = Self::Diagonal(Array1::from_iter(elements));
        assert!(gate.is_unitary(), "Gate is not unitary");
        gate
    }

    fn is_unitary(&self) -> bool {
        match self {
            Self::Normal(matrix) => {
                let eye = Array2::eye(matrix.nrows());
                let Self::Normal(inv) = self.inverse() else {
                    unreachable!()
                };
                inv.dot(matrix) == eye
            }
            Self::Diagonal(matrix) => matrix.iter().all(|c| c.norm() == 1.0),
        }
    }
}

fn is_diagonal(matrix: &Array2<Complex>) -> bool {
    let size = matrix.nrows();
    for i in 0..size {
        for j in 0..size {
            if i != j && matrix[(i, j)] != Complex::ZERO {
                return false;
            }
        }
    }
    true
}

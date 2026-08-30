//! <https://quantum.cloud.ibm.com/docs/en/api/qiskit/circuit_library#standard-gates>

mod many_qubit;
mod single_qubit;
mod two_qubit;

use crate::{Array2, Complex};

use ndarray::{array, linalg::kron};

macro_rules! gate {
    ($([$($el:expr),+ $(,)?]),+ $(,)?) => {
        ndarray::array![
            $([$(num_complex::Complex64::from($el),)+],)+
        ]
    };
}
pub(crate) use gate;

/// A quantum gate
#[derive(Clone, PartialEq, Debug)]
pub struct Gate {
    pub(crate) matrix: Array2<Complex>,
    size: usize,
}

impl Gate {
    /// Create a [`Gate`] from a unitary matrix
    pub fn new(matrix: Array2<Complex>) -> Self {
        assert!(matrix.is_square(), "Gate matrix is not square");
        let gate = Self::new_unitary(matrix);
        assert!(gate.is_unitary(), "Gate is not unitary");
        gate
    }
    /// Get the adjoint of a [`Gate`]
    #[doc(alias = "adjoint", alias = "dagger")]
    pub fn inverse(&self) -> Self {
        let matrix = self.matrix.t().map(|el| el.conj());
        assert!(matrix.is_square(), "Inverse matrix is not square");
        Self::new_unitary(matrix)
    }
    /// Add `num_qubits` control qubits to the [`Gate`]
    pub fn control(&self, num_ctrl_qubits: usize) -> Self {
        let mut lhs = array![
            [Complex::ONE, Complex::ZERO],
            [Complex::ZERO, Complex::ZERO]
        ];
        for _ in 0..num_ctrl_qubits {
            lhs = kron(&lhs, &Array2::eye(2));
        }
        eprintln!("lhs:\n{lhs}");
        let mut rhs = self.matrix.clone();
        for _ in 0..num_ctrl_qubits {
            rhs = kron(
                &array![
                    [Complex::ZERO, Complex::ZERO],
                    [Complex::ZERO, Complex::ONE]
                ],
                &rhs,
            );
        }
        eprintln!("rhs:\n{rhs}");
        Gate::new_unitary(lhs + rhs)
    }

    fn is_unitary(&self) -> bool {
        let eye = Array2::eye(self.size);
        self.inverse().matrix.dot(&self.matrix) == eye
    }
    fn new_unitary(matrix: Array2<Complex>) -> Self {
        Self {
            size: matrix.nrows(),
            matrix,
        }
    }
}

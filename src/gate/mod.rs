//! <https://quantum.cloud.ibm.com/docs/en/api/qiskit/circuit_library#standard-gates>

use ndarray::{Array2, linalg::kron};
use num_complex::Complex64;

mod many_qubit;
mod single_qubit;
mod two_qubit;

macro_rules! complex {
    ($el:expr) => {
        num_complex::Complex64::from($el)
    };
}
pub(crate) use complex;

fn kron_all(matrices: &[Array2<Complex64>]) -> Array2<Complex64> {
    matrices
        .iter()
        .reduce(|acc, e| Box::leak(Box::new(kron(acc, e))))
        .unwrap()
        .clone()
}

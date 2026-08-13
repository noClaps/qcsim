//! <https://quantum.cloud.ibm.com/docs/en/api/qiskit/circuit_library#standard-gates>

mod many_qubit;
mod single_qubit;
mod two_qubit;

macro_rules! complex {
    ($el:expr) => {
        num_complex::Complex64::from($el)
    };
}

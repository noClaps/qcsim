//! <https://quantum.cloud.ibm.com/docs/en/api/qiskit/circuit_library#standard-gates>

mod many_qubit;
mod single_qubit;
mod two_qubit;

/// Create a new complex number.
///
/// # Example
///
/// ```
/// use qcsim::complex;
/// use num_complex::Complex64;
///
/// let cmplx = complex!(2.0);
/// assert_eq!(cmplx, Complex64::new(2.0, 0.0));
/// ```
#[macro_export]
macro_rules! complex {
    ($el:expr) => {
        num_complex::Complex64::from($el)
    };
}

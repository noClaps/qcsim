use num_complex::Complex64;
use qcsim_matrix::Matrix;

use crate::Gate;

impl Gate {
    pub fn identity() -> Self {
        Self::new(Matrix::diag(2))
    }
    pub fn phase(phase: f64) -> Self {
        Self::new(Complex64::cis(phase) * Matrix::diag(2))
    }
}

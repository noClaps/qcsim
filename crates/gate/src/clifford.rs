use std::f64::consts::SQRT_2;

use num_complex::Complex64;
use qcsim_matrix::matrix;

use crate::Gate;

impl Gate {
    pub fn pauli_x() -> Self {
        Self::new(matrix! {
            Complex64::ZERO, Complex64::ONE;
            Complex64::ONE, Complex64::ZERO
        })
    }
    pub fn pauli_y() -> Self {
        Self::new(matrix! {
            Complex64::ZERO, -Complex64::I;
            Complex64::I, Complex64::ZERO
        })
    }
    pub fn pauli_z() -> Self {
        Self::new(matrix! {
            Complex64::ONE, Complex64::ZERO;
            Complex64::ZERO, -Complex64::ONE
        })
    }
    pub fn phase_s() -> Self {
        Self::new(matrix! {
            Complex64::ONE, Complex64::ZERO;
            Complex64::ZERO, Complex64::I
        })
    }
    pub fn sqrt_x() -> Self {
        Self::new(
            0.5 * matrix! {
                1.0 + Complex64::I, 1.0 - Complex64::I;
                1.0 - Complex64::I, 1.0 + Complex64::I
            },
        )
    }
    pub fn hadamard() -> Self {
        Self::new(
            1.0 / SQRT_2
                * matrix! {
                    Complex64::ONE, Complex64::ONE;
                    Complex64::ONE, -Complex64::ONE
                },
        )
    }
    pub fn swap() -> Self {
        Self::new(matrix! {
            Complex64::ONE, Complex64::ZERO, Complex64::ZERO, Complex64::ZERO;
            Complex64::ZERO, Complex64::ZERO, Complex64::ONE, Complex64::ZERO;
            Complex64::ZERO, Complex64::ONE, Complex64::ZERO, Complex64::ZERO;
            Complex64::ZERO, Complex64::ZERO, Complex64::ZERO, Complex64::ONE
        })
    }
    pub fn iswap() -> Self {
        Self::new(matrix! {
            Complex64::ONE, Complex64::ZERO, Complex64::ZERO, Complex64::ZERO;
            Complex64::ZERO, Complex64::ZERO, Complex64::I, Complex64::ZERO;
            Complex64::ZERO, Complex64::I, Complex64::ZERO, Complex64::ZERO;
            Complex64::ZERO, Complex64::ZERO, Complex64::ZERO, Complex64::ONE
        })
    }
}

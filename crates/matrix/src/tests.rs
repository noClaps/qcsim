use num_complex::Complex64;

use crate::{Matrix, matrix};

#[test]
fn matrix_macro() {
    assert_eq!(
        matrix! {
            Complex64::ONE, Complex64::ZERO;
            Complex64::ZERO, Complex64::I
        },
        Matrix(vec![
            vec![Complex64::ONE, Complex64::ZERO],
            vec![Complex64::ZERO, Complex64::I]
        ])
    )
}

#[test]
fn matrix_add() {
    let mat1 = mat! {
        Complex64::ONE, Complex64::ZERO;
        Complex64::ZERO, Complex64::ONE
    };
    let mat2 = mat! {
        Complex64::ZERO, Complex64::ONE;
        Complex64::ONE, Complex64::ZERO
    };
    assert_eq!(
        mat1 + mat2,
        mat! {
            Complex64::ONE, Complex64::ONE;
            Complex64::ONE, Complex64::ONE
        }
    )
}

#[test]
fn matrix_multiply_f64() {
    let mat = mat! {
        Complex64::ONE, Complex64::ZERO;
        Complex64::ZERO, Complex64::ONE
    };
    assert_eq!(
        0.5 * mat,
        mat! {
            Complex64::new(0.5, 0.0), Complex64::ZERO;
            Complex64::ZERO, Complex64::new(0.5, 0.0)
        }
    );
}

#[test]
fn matrix_apply_vec() {
    let mat = mat! {
        Complex64::ZERO, Complex64::ONE;
        Complex64::ONE, Complex64::ZERO
    };
    let vec = vec![Complex64::ONE, Complex64::ZERO];
    assert_eq!(mat * &vec, [Complex64::ZERO, Complex64::ONE]);
}

#[test]
fn matrix_diag() {
    let mat = Matrix::diag(3);
    assert_eq!(
        mat,
        mat! {
            Complex64::ONE, Complex64::ZERO, Complex64::ZERO;
            Complex64::ZERO, Complex64::ONE, Complex64::ZERO;
            Complex64::ZERO, Complex64::ZERO, Complex64::ONE
        }
    )
}

#[test]
fn matrix_pow() {
    let mat = Matrix::diag(3);
    assert_eq!(
        mat.pow(3),
        mat! {
            Complex64::ONE, Complex64::ZERO, Complex64::ZERO;
            Complex64::ZERO, Complex64::ONE, Complex64::ZERO;
            Complex64::ZERO, Complex64::ZERO, Complex64::ONE
        }
    )
}

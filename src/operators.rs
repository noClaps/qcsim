macro_rules! gate {
    (I) => {
        crate::matrix::Matrix(vec![
            vec![crate::complex::Complex::ONE, crate::complex::Complex::ZERO],
            vec![crate::complex::Complex::ZERO, crate::complex::Complex::ONE],
        ])
    };
    (X) => {
        crate::matrix::Matrix(vec![
            vec![crate::complex::Complex::ZERO, crate::complex::Complex::ONE],
            vec![crate::complex::Complex::ONE, crate::complex::Complex::ZERO],
        ])
    };
    (Y) => {
        crate::matrix::Matrix(vec![
            vec![crate::complex::Complex::ZERO, -crate::complex::Complex::I],
            vec![crate::complex::Complex::I, crate::complex::Complex::ZERO],
        ])
    };
    (Z) => {
        crate::matrix::Matrix(vec![
            vec![crate::complex::Complex::ONE, crate::complex::Complex::ZERO],
            vec![crate::complex::Complex::ZERO, -crate::complex::Complex::ONE],
        ])
    };
    (H) => {
        crate::matrix::Matrix(vec![
            vec![
                crate::complex::Complex::ONE / crate::complex::Complex::SQRT_2,
                crate::complex::Complex::ONE / crate::complex::Complex::SQRT_2,
            ],
            vec![
                crate::complex::Complex::ONE / crate::complex::Complex::SQRT_2,
                -crate::complex::Complex::ONE / crate::complex::Complex::SQRT_2,
            ],
        ])
    };
    (PHASE, $phase:expr) => {
        crate::matrix::Matrix(vec![
            vec![crate::complex::Complex::ONE, crate::complex::Complex::ZERO],
            vec![
                crate::complex::Complex::ZERO,
                crate::complex::Complex::cis($phase),
            ],
        ])
    };
    (PI/8) => {
        crate::matrix::Matrix(vec![
            vec![crate::complex::Complex::ONE, crate::complex::Complex::ZERO],
            vec![
                crate::complex::Complex::ZERO,
                crate::complex::Complex::cis(std::f64::consts::FRAC_PI_4),
            ],
        ])
    };
    (|0><0|) => {
        crate::matrix::Matrix(vec![
            vec![crate::complex::Complex::ONE, crate::complex::Complex::ZERO],
            vec![crate::complex::Complex::ZERO, crate::complex::Complex::ZERO],
        ])
    };
    (|0><1|) => {
        crate::matrix::Matrix(vec![
            vec![crate::complex::Complex::ZERO, crate::complex::Complex::ONE],
            vec![crate::complex::Complex::ZERO, crate::complex::Complex::ZERO],
        ])
    };
    (|1><0|) => {
        crate::matrix::Matrix(vec![
            vec![crate::complex::Complex::ZERO, crate::complex::Complex::ZERO],
            vec![crate::complex::Complex::ONE, crate::complex::Complex::ZERO],
        ])
    };
    (|1><1|) => {
        crate::matrix::Matrix(vec![
            vec![crate::complex::Complex::ZERO, crate::complex::Complex::ZERO],
            vec![crate::complex::Complex::ZERO, crate::complex::Complex::ONE],
        ])
    };
}

pub(crate) use gate;

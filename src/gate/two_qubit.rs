use std::f64::consts::FRAC_1_SQRT_2;

use crate::{Complex, Gate, complex::I, gate};

/// 2 qubit gates
impl Gate {
    /// Controlled-Hadamard gate.
    pub fn ch() -> Self {
        gate![
            [1, 0, 0, 0],
            [0, FRAC_1_SQRT_2, 0, FRAC_1_SQRT_2],
            [0, 0, 1, 0],
            [0, FRAC_1_SQRT_2, 0, -FRAC_1_SQRT_2]
        ]
    }
    /// Controlled-Phase gate.
    pub fn cp(theta: f64) -> Self {
        gate![
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, Complex::cis(theta)]
        ]
    }
    /// Controlled-RX gate.
    pub fn crx(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        gate![
            [1, 0, 0, 0],
            [0, cos, 0, -I * sin],
            [0, 0, 1, 0],
            [0, -I * sin, 0, cos]
        ]
    }
    /// Controlled-RY gate.
    pub fn cry(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        gate![
            [1, 0, 0, 0],
            [0, cos, 0, -sin],
            [0, 0, 1, 0],
            [0, sin, 0, cos]
        ]
    }
    /// Controlled-RZ gate.
    pub fn crz(theta: f64) -> Self {
        gate![
            [1, 0, 0, 0],
            [0, Complex::cis(-theta / 2.0), 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, Complex::cis(theta / 2.0)]
        ]
    }
    /// Controlled-S gate.
    pub fn cs() -> Self {
        gate![[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, I]]
    }
    /// Controlled-S† gate.
    pub fn csdg() -> Self {
        gate![[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, -I]]
    }
    /// Controlled-√X gate.
    pub fn csx() -> Self {
        let p = (I + 1) / 2.0;
        let m = (-I + 1) / 2.0;
        gate![[1, 0, 0, 0], [0, p, 0, m], [0, 0, 1, 0], [0, m, 0, p]]
    }
    /// Controlled-U gate (4-parameter two-qubit gate).
    pub fn cu(theta: f64, phi: f64, lam: f64, gamma: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        gate![
            [1, 0, 0, 0],
            [
                0,
                Complex::cis(gamma) * cos,
                0,
                -Complex::cis(gamma + lam) * sin
            ],
            [0, 0, 1, 0],
            [
                0,
                Complex::cis(gamma + phi) * sin,
                0,
                Complex::cis(gamma + phi + lam) * sin
            ]
        ]
    }
    /// Controlled-X gate.
    pub fn cx() -> Self {
        gate![[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, 1], [0, 0, 1, 0]]
    }
    /// Controlled-X gate.
    pub fn cnot() -> Self {
        Self::cx()
    }
    /// Controlled-Y gate.
    pub fn cy() -> Self {
        gate![[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 0, -I], [0, 0, I, 0]]
    }
    /// Controlled-Z gate.
    pub fn cz() -> Self {
        gate![[1, 0, 0, 0], [0, 0, 0, -I], [0, 0, 1, 0], [0, I, 0, 0]]
    }
    /// Double-CX gate.
    pub fn dcx() -> Self {
        gate![[1, 0, 0, 0], [0, 0, 0, 1], [0, 1, 0, 0], [0, 0, 1, 0]]
    }
    /// An echoed cross-resonance gate.
    pub fn ecr() -> Self {
        gate![[0, 1, 0, I], [1, 0, -I, 0], [0, I, 0, 1], [-I, 0, 1, 0]] * FRAC_1_SQRT_2
    }
    /// iSWAP gate.
    pub fn iswap() -> Self {
        gate![[1, 0, 0, 0], [0, 0, I, 0], [0, I, 0, 0], [0, 0, 0, 1]]
    }
    /// A parametric 2-qubit X⊗X interaction (rotation about XX).
    pub fn rxx(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        gate![
            [cos, 0, 0, -I * sin],
            [0, cos, -I * sin, 0],
            [0, -I * sin, cos, 0],
            [-I * sin, 0, 0, cos]
        ]
    }
    /// A parametric 2-qubit Y⊗Y interaction (rotation about YY).
    pub fn ryy(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        gate![
            [cos, 0, 0, I * sin],
            [0, cos, -I * sin, 0],
            [0, -I * sin, cos, 0],
            [I * sin, 0, 0, cos]
        ]
    }
    /// A parametric 2-qubit Z⊗X interaction (rotation about ZX).
    pub fn rzx(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        gate![
            [cos, 0, -I * sin, 0],
            [0, cos, 0, I * sin],
            [-I * sin, 0, cos, 0],
            [0, I * sin, 0, cos]
        ]
    }
    /// A parametric 2-qubit Z⊗Z interaction (rotation about ZZ).
    pub fn rzz(theta: f64) -> Self {
        gate![
            [Complex::cis(-theta / 2.0), 0, 0, 0],
            [0, Complex::cis(theta / 2.0), 0, 0],
            [0, 0, Complex::cis(theta / 2.0), 0],
            [0, 0, 0, Complex::cis(-theta / 2.0)]
        ]
    }
    /// The SWAP gate.
    pub fn swap() -> Self {
        gate![[1, 0, 0, 0], [0, 0, 1, 0], [0, 1, 0, 0], [0, 0, 0, 1]]
    }
    /// XX-YY interaction gate.
    pub fn xx_minus_yy(theta: f64, beta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        gate![
            [cos, 0, 0, -I * sin * Complex::cis(-beta)],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [-I * sin * Complex::cis(beta), 0, 0, cos]
        ]
    }
    /// XX+YY interaction gate.
    pub fn xx_plus_yy(theta: f64, beta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        gate![
            [1, 0, 0, 0],
            [0, cos, -I * sin * Complex::cis(-beta), 0],
            [0, -I * sin * Complex::cis(beta), cos, 0],
            [0, 0, 0, 1]
        ]
    }
}

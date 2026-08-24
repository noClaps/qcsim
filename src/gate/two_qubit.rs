use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_4};

use crate::{Complex, Gate, complex::I, gate};

/// 2 qubit gates
impl Gate {
    /// Controlled-Hadamard gate.
    pub fn ch() -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]]) + Self::h().kron(&gate![[0, 0], [0, 1]])
    }
    /// Controlled-Phase gate.
    pub fn cp(theta: f64) -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]]) + Self::p(theta).kron(&gate![[0, 0], [0, 1]])
    }
    /// Controlled-RX gate.
    pub fn crx(theta: f64) -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]]) + Self::rx(theta).kron(&gate![[0, 0], [0, 1]])
    }
    /// Controlled-RY gate.
    pub fn cry(theta: f64) -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]]) + Self::ry(theta).kron(&gate![[0, 0], [0, 1]])
    }
    /// Controlled-RZ gate.
    pub fn crz(theta: f64) -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]]) + Self::rz(theta).kron(&gate![[0, 0], [0, 1]])
    }
    /// Controlled-S gate.
    pub fn cs() -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]]) + Self::s().kron(&gate![[0, 0], [0, 1]])
    }
    /// Controlled-S† gate.
    pub fn csdg() -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]]) + Self::sdg().kron(&gate![[0, 0], [0, 1]])
    }
    /// Controlled-√X gate.
    pub fn csx() -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]]) + Self::sx().kron(&gate![[0, 0], [0, 1]])
    }
    /// Controlled-U gate (4-parameter two-qubit gate).
    pub fn cu(theta: f64, phi: f64, lam: f64, gamma: f64) -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]])
            + (Self::u(theta, phi, lam).kron(&gate![[0, 0], [0, 1]]) * Complex::cis(gamma))
    }
    /// Controlled-X gate.
    pub fn cx() -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]]) + Self::x().kron(&gate![[0, 0], [0, 1]])
    }
    /// Controlled-X gate.
    pub fn cnot() -> Self {
        Self::cx()
    }
    /// Controlled-Y gate.
    pub fn cy() -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]]) + Self::y().kron(&gate![[0, 0], [0, 1]])
    }
    /// Controlled-Z gate.
    pub fn cz() -> Self {
        Self::i().kron(&gate![[1, 0], [0, 0]]) + Self::z().kron(&gate![[0, 0], [0, 1]])
    }
    /// Double-CX gate.
    pub fn dcx() -> Self {
        gate![[1, 0, 0, 0], [0, 0, 0, 1], [0, 1, 0, 0], [0, 0, 1, 0]]
    }
    /// An echoed cross-resonance gate.
    pub fn ecr() -> Self {
        gate![[0, 1, 0, I], [1, 0, -I, 0], [0, I, 0, 1], [-I, 0, 1, 0],] * FRAC_1_SQRT_2
    }
    /// iSWAP gate.
    pub fn iswap() -> Self {
        (Self::x().kron(&Self::x()) + Self::y().kron(&Self::y())) * Complex::cis(FRAC_PI_4)
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
        Gate::from_diag(&[
            Complex::cis(-theta / 2.0),
            Complex::cis(theta / 2.0),
            Complex::cis(theta / 2.0),
            Complex::cis(-theta / 2.0),
        ])
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
        gate![
            [1, 0, 0, 0],
            [
                0,
                (theta / 2.0).cos(),
                -I * (theta / 2.0).sin() * Complex::cis(-beta),
                0
            ],
            [
                0,
                -I * (theta / 2.0).sin() * Complex::cis(beta),
                (theta / 2.0).cos(),
                0
            ],
            [0, 0, 0, 1]
        ]
    }
}

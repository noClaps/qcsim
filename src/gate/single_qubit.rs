use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use crate::{Complex, Gate, gate};

/// Single qubit gates
impl Gate {
    /// Single-qubit Hadamard gate.
    pub fn h() -> Self {
        Self::u(FRAC_PI_2, 0.0, PI)
    }
    /// Identity gate.
    pub fn i() -> Self {
        gate![[1, 0], [0, 1]]
    }
    /// Identity gate.
    pub fn id() -> Self {
        Self::i()
    }
    /// Single-qubit rotation around the Z axis.
    pub fn p(theta: f64) -> Self {
        Self::u(0.0, 0.0, theta)
    }
    /// Rotation θ around the cos(ϕ)x+sin(ϕ)y axis.
    pub fn r(theta: f64, phi: f64) -> Self {
        Self::u(theta, -FRAC_PI_2 + phi, FRAC_PI_2 - phi)
    }
    /// Single-qubit rotation about the X axis.
    pub fn rx(theta: f64) -> Self {
        Self::r(theta, 0.0)
    }
    /// Single-qubit rotation about the Y axis.
    pub fn ry(theta: f64) -> Self {
        Self::r(theta, FRAC_PI_2)
    }
    /// Single-qubit rotation about the Z axis.
    pub fn rz(phi: f64) -> Self {
        Self::p(phi)
    }
    /// Single qubit S gate (√Z).
    pub fn s() -> Self {
        Self::p(FRAC_PI_2)
    }
    /// Single qubit S-adjoint gate (S†).
    pub fn sdg() -> Self {
        Self::p(-FRAC_PI_2)
    }
    /// The single-qubit Sqrt(X) gate (√X).
    pub fn sx() -> Self {
        Self::sdg() * Self::h() * Self::sdg()
    }
    /// The inverse single-qubit Sqrt(X) gate.
    pub fn sxdg() -> Self {
        Self::s() * Self::h() * Self::s()
    }
    /// Single qubit T gate (∜Z).
    pub fn t() -> Self {
        Self::p(FRAC_PI_4)
    }
    /// Single qubit T-adjoint gate (T†).
    pub fn tdg() -> Self {
        Self::p(-FRAC_PI_4)
    }
    /// Generic single-qubit rotation in terms of ZYZ Euler angles.
    pub fn u(theta: f64, phi: f64, lam: f64) -> Self {
        gate![
            [
                (theta / 2.0).cos(),
                -Complex::cis(lam) * (theta / 2.0).sin()
            ],
            [
                Complex::cis(phi) * (theta / 2.0).sin(),
                Complex::cis(phi + lam) * (theta / 2.0).cos()
            ]
        ]
    }
    /// The single-qubit Pauli-X gate (σ_x).
    pub fn x() -> Self {
        Self::u(PI, 0.0, PI)
    }
    /// The single-qubit Pauli-Y gate (σ_y).
    pub fn y() -> Self {
        Self::u(PI, FRAC_PI_2, FRAC_PI_2)
    }
    /// The single-qubit Pauli-Z gate (σ_z).
    pub fn z() -> Self {
        Self::p(PI)
    }
}

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_4};

use crate::{Complex, Gate, gate::gate};

const I: Complex = Complex::I;
fn cis(phase: f64) -> Complex {
    Complex::cis(phase)
}

/// Single qubit gates
impl Gate {
    /// Single-qubit Hadamard gate.
    pub fn h() -> Self {
        Self::Normal(gate![[1., 1.], [1., -1.]] * FRAC_1_SQRT_2)
    }
    /// Identity gate.
    pub fn i() -> Self {
        Self::Diagonal(gate![1., 1.])
    }
    /// Identity gate.
    #[inline]
    pub fn id() -> Self {
        Self::i()
    }
    /// Single-qubit rotation around the Z axis.
    pub fn p(theta: f64) -> Self {
        Self::Diagonal(gate![1., cis(theta)])
    }
    /// Rotation θ around the cos(ϕ)x+sin(ϕ)y axis.
    pub fn r(theta: f64, phi: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::Normal(gate![
            [cos, -I * cis(-phi) * sin],
            [-I * cis(phi) * sin, cos]
        ])
    }
    /// Single-qubit rotation about the X axis.
    pub fn rx(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::Normal(gate![[cos, -I * sin], [-I * sin, cos]])
    }
    /// Single-qubit rotation about the Y axis.
    pub fn ry(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::Normal(gate![[cos, -sin], [sin, cos]])
    }
    /// Single-qubit rotation about the Z axis.
    pub fn rz(phi: f64) -> Self {
        Self::Diagonal(gate![cis(-phi / 2.0), cis(phi / 2.0)])
    }
    /// Single qubit S gate (√Z).
    pub fn s() -> Self {
        Self::Diagonal(gate![1., I])
    }
    /// Single qubit S-adjoint gate (S†).
    pub fn sdg() -> Self {
        Self::s().inverse()
    }
    /// The single-qubit Sqrt(X) gate (√X).
    pub fn sx() -> Self {
        Self::Normal(gate![[I + 1., -I + 1.], [-I + 1., I + 1.]] * 0.5)
    }
    /// The inverse single-qubit Sqrt(X) gate.
    pub fn sxdg() -> Self {
        Self::sx().inverse()
    }
    /// Single qubit T gate (∜Z).
    pub fn t() -> Self {
        Self::Diagonal(gate![1., cis(FRAC_PI_4)])
    }
    /// Single qubit T-adjoint gate (T†).
    pub fn tdg() -> Self {
        Self::t().inverse()
    }
    /// Generic single-qubit rotation in terms of ZYZ Euler angles.
    pub fn u(theta: f64, phi: f64, lam: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::Normal(gate![
            [cos, -cis(lam) * sin],
            [cis(phi) * sin, cis(phi + lam) * cos]
        ])
    }
    /// The single-qubit Pauli-X gate (σ_x).
    pub fn x() -> Self {
        Self::Normal(gate![[0., 1.], [1., 0.]])
    }
    /// The single-qubit Pauli-Y gate (σ_y).
    pub fn y() -> Self {
        Self::Normal(gate![[0., -I], [I, 0.]])
    }
    /// The single-qubit Pauli-Z gate (σ_z).
    pub fn z() -> Self {
        Self::Diagonal(gate![1., -1.])
    }
}

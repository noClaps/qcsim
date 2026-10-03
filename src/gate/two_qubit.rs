use std::f64::consts::FRAC_1_SQRT_2;

use crate::{Complex, Gate, gate::gate};

const I: Complex = Complex::I;
fn cis(phase: f64) -> Complex {
    Complex::cis(phase)
}

/// 2 qubit gates
impl Gate {
    /// Controlled-Hadamard gate.
    #[must_use]
    pub fn ch() -> Self {
        Self::h().control(1)
    }
    /// Controlled-Phase gate.
    #[must_use]
    pub fn cp(theta: f64) -> Self {
        Self::p(theta).control(1)
    }
    /// Controlled-RX gate.
    #[must_use]
    pub fn crx(theta: f64) -> Self {
        Self::rx(theta).control(1)
    }
    /// Controlled-RY gate.
    #[must_use]
    pub fn cry(theta: f64) -> Self {
        Self::ry(theta).control(1)
    }
    /// Controlled-RZ gate.
    #[must_use]
    pub fn crz(theta: f64) -> Self {
        Self::rz(theta).control(1)
    }
    /// Controlled-S gate.
    #[must_use]
    pub fn cs() -> Self {
        Self::s().control(1)
    }
    /// Controlled-S† gate.
    #[must_use]
    pub fn csdg() -> Self {
        Self::sdg().control(1)
    }
    /// Controlled-√X gate.
    #[must_use]
    pub fn csx() -> Self {
        Self::sx().control(1)
    }
    /// Controlled-U gate (4-parameter two-qubit gate).
    #[must_use]
    pub fn cu(theta: f64, phi: f64, lam: f64, gamma: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::Normal(gate![
            [1., 0., 0., 0.],
            [0., cis(gamma) * cos, 0., -cis(gamma + lam) * sin],
            [0., 0., 1., 0.],
            [0., cis(gamma + phi) * sin, 0., cis(gamma + phi + lam) * cos]
        ])
    }
    /// Controlled-X gate.
    #[must_use]
    pub fn cx() -> Self {
        Self::x().control(1)
    }
    /// Controlled-X gate.
    #[must_use]
    pub fn cnot() -> Self {
        Self::cx()
    }
    /// Controlled-Y gate.
    #[must_use]
    pub fn cy() -> Self {
        Self::y().control(1)
    }
    /// Controlled-Z gate.
    #[must_use]
    pub fn cz() -> Self {
        Self::z().control(1)
    }
    /// Double-CX gate.
    #[must_use]
    pub fn dcx() -> Self {
        Self::Normal(gate![
            [1., 0., 0., 0.],
            [0., 0., 0., 1.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.]
        ])
    }
    /// An echoed cross-resonance gate.
    #[must_use]
    pub fn ecr() -> Self {
        Self::Normal(
            gate![
                [0., 1., 0., I],
                [1., 0., -I, 0.],
                [0., I, 0., 1.],
                [-I, 0., 1., 0.]
            ] * FRAC_1_SQRT_2,
        )
    }
    /// iSWAP gate.
    #[must_use]
    pub fn iswap() -> Self {
        Self::Normal(gate![
            [1., 0., 0., 0.],
            [0., 0., I, 0.],
            [0., I, 0., 0.],
            [0., 0., 0., 1.]
        ])
    }
    /// A parametric 2-qubit X⊗X interaction (rotation about XX).
    #[must_use]
    pub fn rxx(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::Normal(gate![
            [cos, 0., 0., -I * sin],
            [0., cos, -I * sin, 0.],
            [0., -I * sin, cos, 0.],
            [-I * sin, 0., 0., cos]
        ])
    }
    /// A parametric 2-qubit Y⊗Y interaction (rotation about YY).
    #[must_use]
    pub fn ryy(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::Normal(gate![
            [cos, 0., 0., I * sin],
            [0., cos, -I * sin, 0.],
            [0., -I * sin, cos, 0.],
            [I * sin, 0., 0., cos]
        ])
    }
    /// A parametric 2-qubit Z⊗X interaction (rotation about ZX).
    #[must_use]
    pub fn rzx(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::Normal(gate![
            [cos, 0., -I * sin, 0.],
            [0., cos, 0., I * sin],
            [-I * sin, 0., cos, 0.],
            [0., I * sin, 0., cos]
        ])
    }
    /// A parametric 2-qubit Z⊗Z interaction (rotation about ZZ).
    #[must_use]
    pub fn rzz(theta: f64) -> Self {
        Self::Diagonal(gate![
            cis(-theta / 2.0),
            cis(theta / 2.0),
            cis(theta / 2.0),
            cis(-theta / 2.0)
        ])
    }
    /// The SWAP gate.
    #[must_use]
    pub fn swap() -> Self {
        Self::Normal(gate![
            [1., 0., 0., 0.],
            [0., 0., 1., 0.],
            [0., 1., 0., 0.],
            [0., 0., 0., 1.]
        ])
    }
    /// XX-YY interaction gate.
    #[must_use]
    pub fn xx_minus_yy(theta: f64, beta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::Normal(gate![
            [cos, 0., 0., -I * sin * cis(-beta)],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [-I * sin * cis(beta), 0., 0., cos]
        ])
    }
    /// XX+YY interaction gate.
    #[must_use]
    pub fn xx_plus_yy(theta: f64, beta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::Normal(gate![
            [1., 0., 0., 0.],
            [0., cos, -I * sin * cis(-beta), 0.],
            [0., -I * sin * cis(beta), cos, 0.],
            [0., 0., 0., 1.]
        ])
    }
}

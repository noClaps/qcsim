use std::f64::consts::FRAC_1_SQRT_2;

use crate::{Complex, Gate, gate::gate};

const I: Complex = Complex::I;
fn cis(phase: f64) -> Complex {
    Complex::cis(phase)
}

/// 2 qubit gates
impl Gate {
    /// Controlled-Hadamard gate.
    pub fn ch() -> Self {
        Self::h().control(1)
    }
    /// Controlled-Phase gate.
    pub fn cp(theta: f64) -> Self {
        Self::p(theta).control(1)
    }
    /// Controlled-RX gate.
    pub fn crx(theta: f64) -> Self {
        Self::rx(theta).control(1)
    }
    /// Controlled-RY gate.
    pub fn cry(theta: f64) -> Self {
        Self::ry(theta).control(1)
    }
    /// Controlled-RZ gate.
    pub fn crz(theta: f64) -> Self {
        Self::rz(theta).control(1)
    }
    /// Controlled-S gate.
    pub fn cs() -> Self {
        Self::s().control(1)
    }
    /// Controlled-S† gate.
    pub fn csdg() -> Self {
        Self::s().control(1)
    }
    /// Controlled-√X gate.
    pub fn csx() -> Self {
        Self::sx().control(1)
    }
    /// Controlled-U gate (4-parameter two-qubit gate).
    pub fn cu(theta: f64, phi: f64, lam: f64, gamma: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::new_unitary(gate![
            [1., 0., 0., 0.],
            [0., cis(gamma) * cos, 0., -cis(gamma + lam) * sin],
            [0., 0., 1., 0.],
            [0., cis(gamma + phi) * sin, 0., cis(gamma + phi + lam) * sin]
        ])
    }
    /// Controlled-X gate.
    pub fn cx() -> Self {
        Self::x().control(1)
    }
    /// Controlled-X gate.
    pub fn cnot() -> Self {
        Self::cx()
    }
    /// Controlled-Y gate.
    pub fn cy() -> Self {
        Self::y().control(1)
    }
    /// Controlled-Z gate.
    pub fn cz() -> Self {
        Self::z().control(1)
    }
    /// Double-CX gate.
    pub fn dcx() -> Self {
        Self::new_unitary(gate![
            [1., 0., 0., 0.],
            [0., 0., 0., 1.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.]
        ])
    }
    /// An echoed cross-resonance gate.
    pub fn ecr() -> Self {
        Self::new_unitary(
            gate![
                [0., 1., 0., I],
                [1., 0., -I, 0.],
                [0., I, 0., 1.],
                [-I, 0., 1., 0.]
            ] * FRAC_1_SQRT_2,
        )
    }
    /// iSWAP gate.
    pub fn iswap() -> Self {
        Self::new_unitary(gate![
            [1., 0., 0., 0.],
            [0., 0., I, 0.],
            [0., I, 0., 0.],
            [0., 0., 0., 1.]
        ])
    }
    /// A parametric 2-qubit X⊗X interaction (rotation about XX).
    pub fn rxx(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::new_unitary(gate![
            [cos, 0., 0., -I * sin],
            [0., cos, -I * sin, 0.],
            [0., -I * sin, cos, 0.],
            [-I * sin, 0., 0., cos]
        ])
    }
    /// A parametric 2-qubit Y⊗Y interaction (rotation about YY).
    pub fn ryy(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::new_unitary(gate![
            [cos, 0., 0., I * sin],
            [0., cos, -I * sin, 0.],
            [0., -I * sin, cos, 0.],
            [I * sin, 0., 0., cos]
        ])
    }
    /// A parametric 2-qubit Z⊗X interaction (rotation about ZX).
    pub fn rzx(theta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::new_unitary(gate![
            [cos, 0., -I * sin, 0.],
            [0., cos, 0., I * sin],
            [-I * sin, 0., cos, 0.],
            [0., I * sin, 0., cos]
        ])
    }
    /// A parametric 2-qubit Z⊗Z interaction (rotation about ZZ).
    pub fn rzz(theta: f64) -> Self {
        Self::new_unitary(gate![
            [cis(-theta / 2.0), 0., 0., 0.],
            [0., cis(theta / 2.0), 0., 0.],
            [0., 0., cis(theta / 2.0), 0.],
            [0., 0., 0., cis(-theta / 2.0)]
        ])
    }
    /// The SWAP gate.
    pub fn swap() -> Self {
        Self::new_unitary(gate![
            [1., 0., 0., 0.],
            [0., 0., 1., 0.],
            [0., 1., 0., 0.],
            [0., 0., 0., 1.]
        ])
    }
    /// XX-YY interaction gate.
    pub fn xx_minus_yy(theta: f64, beta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::new_unitary(gate![
            [cos, 0., 0., -I * sin * cis(-beta)],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [-I * sin * cis(beta), 0., 0., cos]
        ])
    }
    /// XX+YY interaction gate.
    pub fn xx_plus_yy(theta: f64, beta: f64) -> Self {
        let (sin, cos) = (theta / 2.0).sin_cos();
        Self::new_unitary(gate![
            [1., 0., 0., 0.],
            [0., cos, -I * sin * cis(-beta), 0.],
            [0., -I * sin * cis(beta), cos, 0.],
            [0., 0., 0., 1.]
        ])
    }
}

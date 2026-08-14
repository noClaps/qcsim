use ndarray::array;
use num_complex::c64;

use crate::QuantumCircuit;

/// 2 qubit gates
impl QuantumCircuit {
    /// Controlled-Hadamard gate.
    pub fn ch(&mut self, q0: usize, q1: usize) {
        self.s(q1);
        self.h(q1);
        self.t(q1);
        self.cx(q0, q1);
        self.tdg(q1);
        self.h(q1);
        self.sdg(q1);
    }
    /// Controlled-Phase gate.
    pub fn cp(&mut self, theta: f64, q0: usize, q1: usize) {
        self.p(theta / 2.0, q0);
        self.cx(q0, q1);
        self.p(-theta / 2.0, q1);
        self.cx(q0, q1);
        self.p(theta / 2.0, q1);
    }
    /// Controlled-RX gate.
    pub fn crx(&mut self, theta: f64, q0: usize, q1: usize) {
        self.s(q1);
        self.cx(q0, q1);
        self.ry(-0.5 * theta, q1);
        self.cx(q0, q1);
        self.ry(0.5 * theta, q1);
        self.sdg(q1);
    }
    /// Controlled-RY gate.
    pub fn cry(&mut self, theta: f64, q0: usize, q1: usize) {
        self.ry(theta / 2.0, q1);
        self.cx(q0, q1);
        self.ry(-theta / 2.0, q1);
        self.cx(q0, q1);
    }
    /// Controlled-RZ gate.
    pub fn crz(&mut self, theta: f64, q0: usize, q1: usize) {
        self.rz(theta / 2.0, q1);
        self.cx(q0, q1);
        self.rz(-theta / 2.0, q1);
        self.cx(q0, q1);
    }
    /// Controlled-S gate.
    pub fn cs(&mut self, q0: usize, q1: usize) {
        self.t(q0);
        self.cx(q0, q1);
        self.tdg(q1);
        self.cx(q0, q1);
        self.t(q1);
    }
    /// Controlled-S† gate.
    pub fn csdg(&mut self, q0: usize, q1: usize) {
        self.tdg(q0);
        self.cx(q0, q1);
        self.t(q1);
        self.cx(q0, q1);
        self.tdg(q1);
    }
    /// Controlled-√X gate.
    pub fn csx(&mut self, q0: usize, q1: usize) {
        self.h(q1);
        self.cs(q0, q1);
        self.h(q1);
    }
    /// Controlled-U gate (4-parameter two-qubit gate).
    pub fn cu(&mut self, theta: f64, phi: f64, lam: f64, gamma: f64, q0: usize, q1: usize) {
        self.p(gamma, q0);
        self.p(lam / 2.0 - phi / 2.0, q1);
        self.p(lam / 2.0 + phi / 2.0, q1);
        self.cx(q0, q1);
        self.u(-theta / 2.0, theta, -lam / 2.0 - phi / 2.0, q1);
        self.cx(q0, q1);
        self.u(theta / 2.0, phi, theta, q1);
    }
    /// Controlled-X gate.
    pub fn cx(&mut self, q0: usize, q1: usize) {
        let mat = array![
            [c64(1, 0), c64(0, 0), c64(0, 0), c64(0, 0)],
            [c64(0, 0), c64(1, 0), c64(0, 0), c64(0, 0)],
            [c64(0, 0), c64(0, 0), c64(0, 0), c64(1, 0)],
            [c64(0, 0), c64(0, 0), c64(1, 0), c64(0, 0)],
        ];
        self.unitary(&mat, &[q0, q1]);
    }
    /// Controlled-X gate.
    pub fn cnot(&mut self, q0: usize, q1: usize) {
        self.cx(q0, q1);
    }
    /// Controlled-Y gate.
    pub fn cy(&mut self, q0: usize, q1: usize) {
        self.sdg(q1);
        self.cx(q0, q1);
        self.s(q1);
    }
    /// Controlled-Z gate.
    pub fn cz(&mut self, q0: usize, q1: usize) {
        self.h(q1);
        self.cx(q0, q1);
        self.h(q1);
    }
    /// Double-CX gate.
    pub fn dcx(&mut self, q0: usize, q1: usize) {
        self.cx(q0, q1);
        self.cx(q1, q0);
    }
    /// An echoed cross-resonance gate.
    pub fn ecr(&mut self, q0: usize, q1: usize) {
        self.s(q1);
        self.sx(q0);
        self.cx(q1, q0);
        self.x(q1);
    }
    /// iSWAP gate.
    pub fn iswap(&mut self, q0: usize, q1: usize) {
        self.s(q0);
        self.s(q1);
        self.h(q0);
        self.cx(q0, q1);
        self.cx(q1, q0);
        self.h(q1);
    }
    /// A parametric 2-qubit X⊗X interaction (rotation about XX).
    pub fn rxx(&mut self, theta: f64, q0: usize, q1: usize) {
        self.h(q0);
        self.h(q1);
        self.cx(q0, q1);
        self.rz(theta, q1);
        self.cx(q0, q1);
        self.h(q0);
        self.h(q1);
    }
    /// A parametric 2-qubit Y⊗Y interaction (rotation about YY).
    pub fn ryy(&mut self, theta: f64, q0: usize, q1: usize) {
        self.sxdg(q0);
        self.sxdg(q1);
        self.cx(q0, q1);
        self.rz(theta, q1);
        self.cx(q0, q1);
        self.sx(q0);
        self.sx(q1);
    }
    /// A parametric 2-qubit Z⊗X interaction (rotation about ZX).
    pub fn rzx(&mut self, theta: f64, q0: usize, q1: usize) {
        self.h(q0);
        self.cx(q1, q0);
        self.rz(theta, q0);
        self.cx(q1, q0);
        self.h(q0);
    }
    /// A parametric 2-qubit Z⊗Z interaction (rotation about ZZ).
    pub fn rzz(&mut self, theta: f64, q0: usize, q1: usize) {
        self.cx(q0, q1);
        self.rz(theta, q1);
        self.cx(q0, q1);
    }
    /// The SWAP gate.
    pub fn swap(&mut self, q0: usize, q1: usize) {
        self.cx(q0, q1);
        self.cx(q1, q0);
        self.cx(q0, q1);
    }
    /// XX-YY interaction gate.
    pub fn xx_minus_yy(&mut self, theta: f64, beta: f64, q0: usize, q1: usize) {
        self.sdg(q1);
        self.rz(-beta, q0);
        self.sx(q1);
        self.s(q0);
        self.s(q1);
        self.cx(q1, q0);
        self.ry(theta / 2.0, q1);
        self.ry(-theta / 2.0, q0);
        self.cx(q1, q0);
        self.sdg(q1);
        self.sdg(q0);
        self.sxdg(q1);
        self.rz(beta, q0);
        self.s(q1);
    }
    /// XX+YY interaction gate.
    pub fn xx_plus_yy(&mut self, theta: f64, beta: f64, q0: usize, q1: usize) {
        self.rz(beta, q1);
        self.sdg(q0);
        self.s(q1);
        self.sx(q0);
        self.s(q0);
        self.cx(q0, q1);
        self.ry(-theta / 2.0, q1);
        self.ry(-theta / 2.0, q0);
        self.cx(q0, q1);
        self.sdg(q1);
        self.sdg(q0);
        self.rz(-beta, q1);
        self.sxdg(q0);
        self.s(q0);
    }
}

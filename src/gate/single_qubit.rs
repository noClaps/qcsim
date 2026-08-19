use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

use ndarray::array;

use crate::{Complex, QuantumCircuit, cmplx, qubits::Qubits};

/// Single qubit gates
impl QuantumCircuit {
    /// Single-qubit Hadamard gate.
    pub fn h(&mut self, idx: impl Into<Qubits>) {
        self.u(FRAC_PI_2, 0.0, PI, idx);
    }
    /// Identity gate.
    pub fn i(&mut self, _idx: usize) {}
    /// Identity gate.
    pub fn id(&mut self, _idx: usize) {}
    /// Single-qubit rotation around the Z axis.
    pub fn p(&mut self, theta: f64, idx: impl Into<Qubits>) {
        self.u(0.0, 0.0, theta, idx);
    }
    /// Rotation θ around the cos(ϕ)x+sin(ϕ)y axis.
    pub fn r(&mut self, theta: f64, phi: f64, idx: impl Into<Qubits>) {
        self.u(theta, -FRAC_PI_2 + phi, FRAC_PI_2 - phi, idx);
    }
    /// Single-qubit rotation about the X axis.
    pub fn rx(&mut self, theta: f64, idx: impl Into<Qubits>) {
        self.r(theta, 0.0, idx);
    }
    /// Single-qubit rotation about the Y axis.
    pub fn ry(&mut self, theta: f64, idx: impl Into<Qubits>) {
        self.r(theta, FRAC_PI_2, idx);
    }
    /// Single-qubit rotation about the Z axis.
    pub fn rz(&mut self, phi: f64, idx: impl Into<Qubits>) {
        self.p(phi, idx);
    }
    /// Single qubit S gate (√Z).
    pub fn s(&mut self, idx: impl Into<Qubits>) {
        self.p(FRAC_PI_2, idx);
    }
    /// Single qubit S-adjoint gate (S†).
    pub fn sdg(&mut self, idx: impl Into<Qubits>) {
        self.p(-FRAC_PI_2, idx);
    }
    /// The single-qubit Sqrt(X) gate (√X).
    pub fn sx(&mut self, idx: impl Into<Qubits>) {
        let idx = idx.into();
        self.sdg(idx.clone());
        self.h(idx.clone());
        self.sdg(idx);
    }
    /// The inverse single-qubit Sqrt(X) gate.
    pub fn sxdg(&mut self, idx: impl Into<Qubits>) {
        let idx = idx.into();
        self.s(idx.clone());
        self.h(idx.clone());
        self.s(idx);
    }
    /// Single qubit T gate (∜Z).
    pub fn t(&mut self, idx: impl Into<Qubits>) {
        self.p(FRAC_PI_4, idx);
    }
    /// Single qubit T-adjoint gate (T†).
    pub fn tdg(&mut self, idx: impl Into<Qubits>) {
        self.p(-FRAC_PI_4, idx);
    }
    /// Generic single-qubit rotation in terms of ZYZ Euler angles.
    pub fn u(&mut self, theta: f64, phi: f64, lam: f64, idx: impl Into<Qubits>) {
        let mat = array![
            [
                cmplx!(theta / 2.0).cos(),
                -Complex::cis(lam) * (theta / 2.0).sin()
            ],
            [
                Complex::cis(phi) * (theta / 2.0).sin(),
                Complex::cis(phi + lam) * (theta / 2.0).cos()
            ]
        ];
        match idx.into() {
            Qubits::Single(idx) => self.unitary(&mat, idx),
            Qubits::Array(idxs) => {
                for idx in idxs {
                    self.unitary(&mat, idx);
                }
            }
        }
    }
    /// The single-qubit Pauli-X gate (σ_x).
    pub fn x(&mut self, idx: impl Into<Qubits>) {
        self.u(PI, 0.0, PI, idx);
    }
    /// The single-qubit Pauli-Y gate (σ_y).
    pub fn y(&mut self, idx: impl Into<Qubits>) {
        self.u(PI, FRAC_PI_2, FRAC_PI_2, idx);
    }
    /// The single-qubit Pauli-Z gate (σ_z).
    pub fn z(&mut self, idx: impl Into<Qubits>) {
        self.p(PI, idx);
    }
}

use std::f64::consts::{FRAC_PI_2, FRAC_PI_8};

use crate::QuantumCircuit;

impl QuantumCircuit {
    /// The 3-qubit controlled sqrt-X gate.
    pub fn c3sx(&mut self, q0: usize, q1: usize, q2: usize, q3: usize) {
        self.h(q3);
        self.cp(FRAC_PI_8, q0, q3);
        self.cx(q0, q1);
        self.h(q3);
        self.h(q3);
        self.cp(-FRAC_PI_8, q1, q3);
        self.cx(q0, q1);
        self.h(q3);
        self.h(q3);
        self.cp(FRAC_PI_8, q1, q3);
        self.cx(q1, q2);
        self.h(q3);
        self.h(q3);
        self.cp(-FRAC_PI_8, q2, q3);
        self.cx(q0, q2);
        self.h(q3);
        self.h(q3);
        self.cp(FRAC_PI_8, q2, q3);
        self.cx(q1, q2);
        self.h(q3);
        self.h(q3);
        self.cp(-FRAC_PI_8, q2, q3);
        self.cx(q0, q2);
        self.h(q3);
        self.h(q3);
        self.cp(FRAC_PI_8, q2, q3);
        self.h(q3);
    }
    /// The X gate controlled on 3 qubits.
    pub fn c3x(&mut self, q0: usize, q1: usize, q2: usize, q3: usize) {
        self.p(FRAC_PI_8, q0);
        self.p(FRAC_PI_8, q1);
        self.p(FRAC_PI_8, q2);
        self.h(q3);
        self.cx(q0, q1);
        self.p(FRAC_PI_8, q3);
        self.p(-FRAC_PI_8, q1);
        self.cx(q0, q1);
        self.cx(q1, q2);
        self.p(-FRAC_PI_8, q2);
        self.cx(q0, q2);
        self.p(FRAC_PI_8, q2);
        self.cx(q1, q2);
        self.p(-FRAC_PI_8, q2);
        self.cx(q0, q2);
        self.cx(q2, q3);
        self.p(-FRAC_PI_8, q3);
        self.cx(q1, q3);
        self.p(FRAC_PI_8, q3);
        self.cx(q2, q3);
        self.p(-FRAC_PI_8, q3);
        self.cx(q0, q3);
        self.p(FRAC_PI_8, q3);
        self.cx(q2, q3);
        self.p(-FRAC_PI_8, q3);
        self.cx(q1, q3);
        self.p(FRAC_PI_8, q3);
        self.cx(q2, q3);
        self.p(-FRAC_PI_8, q3);
        self.cx(q0, q3);
        self.h(q3);
    }
    /// The 4-qubit controlled X gate.
    pub fn c4x(&mut self, q0: usize, q1: usize, q2: usize, q3: usize, q4: usize) {
        self.h(q4);
        self.cp(FRAC_PI_2, q3, q4);
        self.rcccx(q0, q1, q2, q3);
        self.h(q4);
        self.h(q4);
        self.cp(-FRAC_PI_2, q3, q4);
        self.rcccx_dg(q0, q1, q2, q3);
        self.h(q4);
        self.c3sx(q0, q1, q2, q4);
    }
    /// CCX gate, also known as Toffoli gate.
    pub fn ccx(&mut self, q0: usize, q1: usize, q2: usize) {
        self.h(q2);
        self.cx(q1, q2);
        self.tdg(q2);
        self.cx(q0, q2);
        self.t(q2);
        self.cx(q1, q2);
        self.t(q1);
        self.tdg(q2);
        self.cx(q0, q2);
        self.cx(q0, q1);
        self.t(q2);
        self.t(q0);
        self.tdg(q1);
        self.h(q2);
        self.cx(q0, q1);
    }
    /// CCZ gate.
    pub fn ccz(&mut self, q0: usize, q1: usize, q2: usize) {
        self.h(q2);
        self.ccx(q0, q1, q2);
        self.h(q2);
    }
    /// Controlled-SWAP gate, also known as the Fredkin gate.
    pub fn cswap(&mut self, q0: usize, q1: usize, q2: usize) {
        self.cx(q2, q1);
        self.ccx(q0, q1, q2);
        self.cx(q2, q1);
    }
    /// Controlled-SWAP gate, also known as the Fredkin gate.
    pub fn fredkin(&mut self, q0: usize, q1: usize, q2: usize) {
        self.cswap(q0, q1, q2);
    }
    /// The simplified Toffoli gate, also referred to as Margolus gate.
    pub fn rccx(&mut self, q0: usize, q1: usize, q2: usize) {
        self.h(q2);
        self.t(q2);
        self.cx(q1, q2);
        self.tdg(q2);
        self.cx(q0, q2);
        self.t(q2);
        self.cx(q1, q2);
        self.tdg(q2);
        self.h(q2);
    }
    /// The simplified 3-controlled Toffoli gate.
    pub fn rcccx(&mut self, q0: usize, q1: usize, q2: usize, q3: usize) {
        self.h(q3);
        self.t(q3);
        self.cx(q2, q3);
        self.tdg(q3);
        self.h(q3);
        self.cx(q0, q3);
        self.t(q3);
        self.cx(q1, q3);
        self.tdg(q3);
        self.cx(q0, q3);
        self.t(q3);
        self.cx(q1, q3);
        self.tdg(q3);
        self.h(q3);
        self.t(q3);
        self.cx(q2, q3);
        self.tdg(q3);
        self.h(q3);
    }
    fn rcccx_dg(&mut self, q0: usize, q1: usize, q2: usize, q3: usize) {
        self.h(q3);
        self.t(q3);
        self.cx(q2, q3);
        self.tdg(q3);
        self.h(q3);
        self.t(q3);
        self.cx(q1, q3);
        self.tdg(q3);
        self.cx(q0, q3);
        self.t(q3);
        self.cx(q1, q3);
        self.tdg(q3);
        self.cx(q0, q3);
        self.h(q3);
        self.t(q3);
        self.cx(q2, q3);
        self.tdg(q3);
        self.h(q3);
    }
}

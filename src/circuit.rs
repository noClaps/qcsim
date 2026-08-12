use ndarray::Array1;
use num_complex::Complex64;

use crate::gate::complex;

#[derive(Clone)]
pub struct QuantumCircuit {
    pub qubits: Array1<Complex64>,
    pub num_qubits: usize,
}

impl QuantumCircuit {
    /// Initialise a quantum circuit with state |00...0>
    pub fn new(num_qubits: usize) -> Self {
        let mut qubits = Array1::zeros(1 << num_qubits);
        qubits[0] = complex!(1.0);
        Self { qubits, num_qubits }
    }

    /// Measure the state of the qubits. Consumes the circuit instance.
    pub fn measure(self) -> String {
        let rand = fastrand::f64_inclusive();
        let mut total = 0.0;
        for (i, state) in self.qubits.iter().enumerate() {
            total += state.norm_sqr();
            if total >= rand {
                return format!("{:0>1$}", format!("{i:b}"), self.num_qubits);
            }
        }
        format!("{:b}", self.qubits.len() - 1)
    }

    /// Reset the qubits to the |00...0> state
    pub fn reset(&mut self) {
        self.qubits = Array1::zeros(1 << self.num_qubits);
        self.qubits[0] = complex!(1.0);
    }
}

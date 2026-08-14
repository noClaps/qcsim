use ndarray::{Array1, Array2};
use num_complex::Complex64;

use crate::complex;

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

    /// Apply unitary `gate` to `qubits`
    pub fn unitary(&mut self, gate: Array2<Complex64>, qubits: &[usize]) {
        let state_len = self.qubits.len();
        let n = state_len.trailing_zeros() as usize;
        let k = qubits.len();
        let local_dim = 1 << k;

        let bit_position = |q: usize| -> usize { n - 1 - q };
        let target_mask = qubits
            .iter()
            .fold(0, |acc, &q| acc | (1 << bit_position(q)));
        let mut result = self.qubits.clone();

        for base in 0..state_len {
            if base & target_mask != 0 {
                continue;
            }

            let mut indices = Vec::with_capacity(local_dim);
            for local_index in 0..local_dim {
                let mut global_index = base;
                for (j, &qubit) in qubits.iter().enumerate() {
                    let local_bit = (local_index >> (k - 1 - j)) & 1;
                    let global_bit = bit_position(qubit);
                    global_index |= local_bit << global_bit;
                }
                indices.push(global_index);
            }

            let input_block = Array1::from_iter(indices.iter().map(|&i| self.qubits[i]));
            let output_block = gate.dot(&input_block);
            for (local_index, &global_index) in indices.iter().enumerate() {
                result[global_index] = output_block[local_index];
            }
        }

        self.qubits = result;
    }
}

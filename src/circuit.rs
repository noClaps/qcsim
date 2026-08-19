use ndarray::{Array1, Array2};

use crate::{Complex, cmplx, qubits::Qubits};

#[derive(Clone)]
pub struct QuantumCircuit {
    pub qubits: Array1<Complex>,
    pub num_qubits: usize,
}

impl QuantumCircuit {
    /// Initialise a quantum circuit with state |00...0>
    pub fn new(num_qubits: usize) -> Self {
        let mut qubits = Array1::zeros(1 << num_qubits);
        qubits[0] = cmplx!(1);
        Self { qubits, num_qubits }
    }

    /// Measure the state of the qubits. Consumes the circuit instance.
    pub fn measure(self) -> usize {
        let rand = fastrand::f64_inclusive();
        let mut total = 0.0;
        for (i, state) in self.qubits.iter().enumerate() {
            total += state.norm_sqr();
            if total >= rand {
                return i;
            }
        }
        self.qubits.len() - 1
    }

    /// Reset the qubits to the |00...0> state
    pub fn reset(&mut self) {
        self.qubits = Array1::zeros(1 << self.num_qubits);
        self.qubits[0] = cmplx!(1);
    }

    /// Apply unitary `gate` to `qubits`
    pub fn unitary(&mut self, gate: &Array2<Complex>, qubits: impl Into<Qubits>) {
        let qubits = match qubits.into() {
            Qubits::Single(q) => vec![q],
            Qubits::Array(qs) => qs,
        };
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
            // isolate all of the qubits that will remain unchanged.
            // e.g. CNOT(0,2) -> qubit 1 is unchanged
            // this filter will only allow through 0q0 where q can be 0 or 1
            if base & target_mask != 0 {
                continue;
            }

            // all of the possible combinations for the qubits being targeted
            // e.g. 0q0, 0q1, 1q0, 1q1
            let indices: Vec<_> = (0..local_dim)
                .map(|local_index| {
                    let mut global_index = base;
                    for (j, &qubit) in qubits.iter().enumerate() {
                        let local_bit = (local_index >> (k - 1 - j)) & 1;
                        let global_bit = bit_position(qubit);
                        global_index |= local_bit << global_bit;
                    }
                    global_index
                })
                .collect();

            // create input from qubits at `indices`
            // indices = [000, 001, 100, 101] -> input = [A_000, A_001, A_100, A_101]
            // where A_000 is the amplitude for qubit |000>
            let input = Array1::from_iter(indices.iter().map(|&i| self.qubits[i]));
            // apply gate
            let output = gate.dot(&input);
            // write output back into qubits
            // indices = [000, 001, 100, 101]
            // qubits[indices[n]] = output[n]
            for (local_index, &global_index) in indices.iter().enumerate() {
                result[global_index] = output[local_index];
            }
        }

        self.qubits = result;
    }
}

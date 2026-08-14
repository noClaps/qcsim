use std::collections::HashMap;

use crate::circuit::QuantumCircuit;

pub fn sample(qc: QuantumCircuit, samples: usize) -> HashMap<usize, usize> {
    let mut outputs = HashMap::new();
    for _ in 0..samples {
        let measurement = qc.clone().measure();
        match outputs.get_mut(&measurement) {
            Some(c) => *c += 1,
            None => {
                outputs.insert(measurement, 1);
            }
        }
    }
    outputs
}

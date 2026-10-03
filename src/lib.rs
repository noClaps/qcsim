mod circuit;
mod complex;
mod gate;
mod qubits;
#[cfg(test)]
pub mod tests;

use std::collections::HashMap;

pub use circuit::QuantumCircuit;
pub use complex::Complex;
pub use gate::Gate;

pub use ndarray::{Array1, Array2};

pub(crate) type StateVector = Array1<Complex>;

/// Sample the outcome of the [`QuantumCircuit`] `sample` times. Returns the output as a
/// [`HashMap`] where the key is the measurement and the value is the number of times it appeared.
/// Consumes the [`QuantumCircuit`] so it cannot be used afterwards.
///
/// # Example
///
/// ```
/// use qcsim::{QuantumCircuit, sample};
///
/// let mut qc = QuantumCircuit::new(2); // |00>
/// qc.h(0); // 1/sqrt(2) (|00> + |10>)
/// let output = sample(qc, 10000);
/// // output[0b00] = ~5000
/// // output[0b10] = ~5000
/// ```
///
/// # Panics
///
/// Panics if the total sum of the squared amplitudes of the qubits in `qc` do not equal 1.
#[must_use]
pub fn sample(qc: QuantumCircuit, samples: usize) -> HashMap<usize, usize> {
    assert!(
        (qc.qubits.fold(0.0, |acc, q| acc + q.norm_sqr()) - 1.0).abs() < 1e-12,
        "Total sum of squared amplitudes does not equal 1"
    );
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
    drop(qc);
    outputs
}

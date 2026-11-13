mod circuit;
mod complex;
mod gate;
mod matrix;
mod qubits;
mod state_vector;
#[cfg(test)]
mod tests;

use std::collections::HashMap;

pub use circuit::QuantumCircuit;
pub use complex::Complex;
pub use gate::Gate;
pub use matrix::{Matrix, diagonal::DiagonalMatrix, square::SquareMatrix};

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
    let mut cdf = Vec::with_capacity(qc.qubits.len());
    let mut total = 0.0;
    for amp in qc.qubits.iter() {
        total += amp.norm_sqr();
        cdf.push(total);
    }
    assert!(
        (total - 1.0).abs() < 1e-11,
        "Total sum of squared amplitudes does not equal 1"
    );
    let mut outputs = HashMap::new();
    for _ in 0..samples {
        let r = fastrand::f64();
        let idx = cdf.partition_point(|&c| c <= r).min(cdf.len() - 1);
        *outputs.entry(idx).or_insert(0) += 1;
    }
    drop(qc);
    outputs
}

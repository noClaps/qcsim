mod many_qubit;
mod single_qubit;
mod two_qubit;

/// Samples the quantum circuit 10,000 times and checks if the output contains only the provided
/// keys. Panics if any other keys are present.
macro_rules! assert_qc {
    ($qc:expr, $($el:literal),* $(,)?) => {
        let output = crate::sample($qc, 10000);
        assert!(output.keys().all(|k| [$($el,)*].contains(k)), "Expected: {:?}\nFound: {:?}", [$($el,)*], output)
    };
}
pub(crate) use assert_qc;

use crate::{QuantumCircuit, sample};

#[test]
fn readme_example() {
    let mut qc = QuantumCircuit::new(3);
    qc.h(0);
    qc.cx(0, 1);
    qc.ccx(0, 1, 2);
    println!("{}", qc.clone().measure());
    let output = sample(qc, 10000);
    println!("{:?}", output);
}

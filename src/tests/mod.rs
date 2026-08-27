mod many_qubit;
mod single_qubit;
mod two_qubit;

/// Checks if the output contains only the provided keys. Panics if any other keys are present.
macro_rules! assert_output {
    ($output:expr, $($el:literal),* $(,)?) => {
        assert!($output.keys().all(|k| [$($el,)*].contains(k)), "Expected: {:?}\nFound: {:?}", [$($el,)*], $output)
    };
}
pub(crate) use assert_output;

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

mod many_qubit;
mod single_qubit;
mod two_qubit;

/// Samples the quantum circuit 10,000 times and checks if the output contains only the provided
/// keys. Panics if any other keys are present.
macro_rules! assert_qc {
    ($qc:expr, {$($bit:expr => $val:expr),* $(,)?}) => {{
        fn almost_eq(lhs: crate::Complex, rhs: crate::Complex) -> bool {
            for (l, r) in [(lhs.re, rhs.re), (lhs.im, rhs.im)] {
                match true {
                   true if (l.abs() - r.abs()).abs() > 1e-12 => return false,
                   true if l != 0.0 && r != 0.0 && l.signum() != r.signum() => return false,
                   _ => {}
                }
            }
            true
        }
        $(assert!($bit < $qc.qubits.len(), "Bit {} outside qubit range", $bit);)*
        let mut expected = vec![crate::Complex::ZERO; $qc.qubits.len()];
        $(expected[$bit] = $val.into();)*
        let norm = expected.iter().fold(0.0, |acc, q| acc + q.norm_sqr());
        assert!((norm - 1.0).abs() <= 1e-12, "Expected state is not normalised: norm^2 = {norm}");
        for (i, &q) in $qc.qubits.iter().enumerate() {
            if !almost_eq(expected[i], q) {
                panic!("Expected amplitude {} for |{:b}>, found {}", expected[i], i, q);
            }
        }
    }};
}
pub(crate) use assert_qc;

use crate::{QuantumCircuit, sample};

#[test]
fn readme_example() {
    let mut qc = QuantumCircuit::new(3);
    qc.h(0);
    qc.cx(0, 1);
    qc.ccx(0, 1, 2);
    let output = sample(qc, 10000);
    println!("{:?}", output);
}

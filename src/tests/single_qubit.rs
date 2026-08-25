use crate::{QuantumCircuit, sample, tests::assert_output};

#[test]
fn h() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.h(0); // 1/sqrt(2) (|0> + |1>)
    let output = sample(qc, 10000);
    assert_output!(output, 0b0, 0b1);
}

#[test]
fn x() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // |1>
    let output = sample(qc, 10000);
    assert_output!(output, 0b1);
}

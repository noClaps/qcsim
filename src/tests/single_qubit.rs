use std::f64::consts::FRAC_PI_2;

use crate::{QuantumCircuit, sample, tests::assert_output};

#[test]
fn h() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.h(0); // 1/sqrt(2) (|0> + |1>)
    let output = sample(qc, 10000);
    assert_output!(output, 0b0, 0b1);

    let mut qc = QuantumCircuit::new(1); // |1>
    qc.h(0); // 1/sqrt(2) (|0> - |1>)
    let output = sample(qc, 10000);
    assert_output!(output, 0b0, 0b1);
}

#[test]
fn i() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.i(0); // |0>
    let output = sample(qc, 10000);
    assert_output!(output, 0b0);

    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // |1>
    qc.id(0); // |1>
    let output = sample(qc, 10000);
    assert_output!(output, 0b1);
}

#[test]
fn p() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.p(FRAC_PI_2, 0); // |0>
    let output = sample(qc, 10000);
    assert_output!(output, 0b0);

    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // 1>
    qc.p(FRAC_PI_2, 0); // i|1>
    let output = sample(qc, 10000);
    assert_output!(output, 0b1);
}

#[test]
fn x() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // |1>
    let output = sample(qc, 10000);
    assert_output!(output, 0b1);

    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // |1>
    qc.x(0); // |0>
    let output = sample(qc, 10000);
    assert_output!(output, 0b0);
}

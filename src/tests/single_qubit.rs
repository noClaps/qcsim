use std::f64::consts::FRAC_PI_2;

use crate::{QuantumCircuit, tests::assert_qc};

#[test]
fn h() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.h(0); // 1/sqrt(2) (|0> + |1>)
    assert_qc!(qc, 0b0, 0b1);

    let mut qc = QuantumCircuit::new(1); // |1>
    qc.h(0); // 1/sqrt(2) (|0> - |1>)
    assert_qc!(qc, 0b0, 0b1);
}

#[test]
fn i() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.i(0); // |0>
    assert_qc!(qc, 0b0);

    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // |1>
    qc.id(0); // |1>
    assert_qc!(qc, 0b1);
}

#[test]
fn p() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.p(FRAC_PI_2, 0); // |0>
    assert_qc!(qc, 0b0);

    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // 1>
    qc.p(FRAC_PI_2, 0); // i|1>
    assert_qc!(qc, 0b1);
}

#[test]
fn x() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // |1>
    assert_qc!(qc, 0b1);

    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // |1>
    qc.x(0); // |0>
    assert_qc!(qc, 0b0);
}

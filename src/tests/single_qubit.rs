use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2};

use crate::{Complex, QuantumCircuit, tests::assert_qc};

#[test]
fn h() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.h(0); // 1/sqrt(2) (|0> + |1>)
    assert_qc!(qc, {0b0 => FRAC_1_SQRT_2, 0b1 => FRAC_1_SQRT_2});

    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // |1>
    qc.h(0); // 1/sqrt(2) (|0> - |1>)
    assert_qc!(qc, {0b0 => FRAC_1_SQRT_2, 0b1 => -FRAC_1_SQRT_2});
}

#[test]
fn i() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.i(0); // |0>
    assert_qc!(qc, {0b0 => 1.0});

    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // |1>
    qc.id(0); // |1>
    assert_qc!(qc, {0b1 => 1.0});
}

#[test]
fn p() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.p(FRAC_PI_2, 0); // |0>
    assert_qc!(qc, {0b0 => 1.0});

    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // 1>
    qc.p(FRAC_PI_2, 0); // i|1>
    assert_qc!(qc, {0b1 => Complex::I});
}

#[test]
fn x() {
    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // |1>
    assert_qc!(qc, {0b1 => 1.0});

    let mut qc = QuantumCircuit::new(1); // |0>
    qc.x(0); // |1>
    qc.x(0); // |0>
    assert_qc!(qc, {0b0 => 1.0});
}

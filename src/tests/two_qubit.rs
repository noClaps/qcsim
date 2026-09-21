use std::f64::consts::{FRAC_PI_2, PI};

use crate::{QuantumCircuit, tests::assert_qc};

#[test]
fn ch() {
    let mut qc = QuantumCircuit::new(2); // |00>
    qc.x(0); // |10>
    qc.ch(0, 1); // 1/sqrt(2) (|10> + |11>)
    assert_qc!(qc, 0b10, 0b11);
}

#[test]
fn cx() {
    let mut qc = QuantumCircuit::new(2); // |00>
    qc.x(0); // |10>
    qc.cx(0, 1); // |11>
    assert_qc!(qc, 0b11);
}

#[test]
fn ecr_gate() {
    let mut qc = QuantumCircuit::new(5); // |00000>
    qc.ecr(1, 3); // 1/sqrt(2) (|00010> - i|01010>)
    assert_qc!(qc, 0b00010, 0b01010);
}

#[test]
fn iswap() {
    let mut qc = QuantumCircuit::new(2); // |00>
    qc.x(0); // |10>
    qc.iswap(0, 1); // |01>
    assert_qc!(qc, 0b01);
}

#[test]
fn rxx() {
    let mut qc = QuantumCircuit::new(2); // |00>
    qc.rxx(0.0, 0, 1); // |00>
    assert_qc!(qc, 0b00);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.rxx(PI, 0, 1); // |11>
    assert_qc!(qc, 0b11);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.rxx(FRAC_PI_2, 0, 1); // 1/sqrt(2) (|00> - i|11>)
    assert_qc!(qc, 0b00, 0b11);
}

#[test]
fn ryy() {
    let mut qc = QuantumCircuit::new(2); // |00>
    qc.ryy(0.0, 0, 1); // |00>
    assert_qc!(qc, 0b00);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.ryy(PI, 0, 1); // i|11>
    assert_qc!(qc, 0b11);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.ryy(FRAC_PI_2, 0, 1); // 1/sqrt(2) (|00> + i|11>)
    assert_qc!(qc, 0b00, 0b11);
}

#[test]
fn rzx() {
    let mut qc = QuantumCircuit::new(2); // |00>
    qc.rzx(0.0, 0, 1); // |00>
    assert_qc!(qc, 0b00);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.rzx(2.0 * PI, 0, 1); // -|00>
    assert_qc!(qc, 0b00);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.rzx(PI, 0, 1); // -i|10>
    assert_qc!(qc, 0b10);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.rzx(FRAC_PI_2, 0, 1); // 1/sqrt(2) (|00> - i|10>)
    assert_qc!(qc, 0b00, 0b10);
}

#[test]
fn rzz() {
    let mut qc = QuantumCircuit::new(2); // |00>
    qc.rzz(0.0, 0, 1); // |00>
    assert_qc!(qc, 0b00);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.rzz(2.0 * PI, 0, 1); // -|00>
    assert_qc!(qc, 0b00);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.rzz(PI, 0, 1); // -i|00>
    assert_qc!(qc, 0b00);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.rzz(FRAC_PI_2, 0, 1); // 1/sqrt(2) (1-i)|00>
    assert_qc!(qc, 0b00);
}

#[test]
fn swap() {
    let mut qc = QuantumCircuit::new(2); // |00>
    qc.x(0); // |10>
    qc.swap(0, 1); // |01>
    assert_qc!(qc, 0b01);
}

#[test]
fn cu_identity() {
    let mut qc = QuantumCircuit::new(2); // |00>
    qc.cu(0.0, 0.0, 0.0, 0.0, 0, 1); // |00>
    assert_qc!(qc, 0b00);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.x(1); // |01>
    qc.cu(0.0, 0.0, 0.0, 0.0, 0, 1); // |01>
    assert_qc!(qc, 0b01);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.x(0); // |10>
    qc.cu(0.0, 0.0, 0.0, 0.0, 0, 1); // |10>
    assert_qc!(qc, 0b10);

    let mut qc = QuantumCircuit::new(2); // |00>
    qc.x(0); // |10>
    qc.x(1); // |11>
    qc.cu(0.0, 0.0, 0.0, 0.0, 0, 1); // |11>
    assert_qc!(qc, 0b11);
}

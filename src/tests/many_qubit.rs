use std::f64::consts::FRAC_1_SQRT_2;

use crate::{Complex, QuantumCircuit, tests::assert_qc};

#[test]
fn ccx() {
    let mut qc = QuantumCircuit::new(3); // |000>
    qc.h(0); // 1/sqrt(2) (|000> + |100>)
    qc.x(1); // 1/sqrt(2) (|010> + |110>)
    qc.ccx(0, 1, 2); // 1/sqrt(2) (|010> + |111>)
    assert_qc!(qc, {0b010 => FRAC_1_SQRT_2, 0b111 => FRAC_1_SQRT_2});
}

#[test]
fn rc3x() {
    let mut qc = QuantumCircuit::new(4); // |0000>
    qc.x(0..3); // |1110>
    qc.rcccx(0, 1, 2, 3); // -|1111>
    assert_qc!(qc, {0b1111 => -1.0});

    let mut qc = QuantumCircuit::new(4); // |0000>
    qc.x(0..2); // |1100>
    qc.rcccx(0, 1, 2, 3); // i|1100>
    assert_qc!(qc, {0b1100 => Complex::I});
}

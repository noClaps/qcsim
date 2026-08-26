use crate::{QuantumCircuit, sample, tests::assert_output};

#[test]
fn ccx() {
    let mut qc = QuantumCircuit::new(3); // |000>
    qc.h(0); // 1/sqrt(2) (|000> + |100>)
    qc.x(1); // 1/sqrt(2) (|010> + |110>)
    qc.ccx(0, 1, 2); // 1/sqrt(2) (|010> + |111>)
    let output = sample(qc, 10000);
    assert_output!(output, 0b010, 0b111);
}

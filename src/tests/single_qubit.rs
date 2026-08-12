use crate::{QuantumCircuit, sample, tests::assert_output};

#[test]
fn ch() {
    let mut qc = QuantumCircuit::new(2); // |00>
    qc.x(0); // |10>
    qc.ch(0, 1); // 1/sqrt(2) (|10> + |11>)

    let outputs = sample(qc, 10000);
    assert_output!(outputs, "10", "11");
}

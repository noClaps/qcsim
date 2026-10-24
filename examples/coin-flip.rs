use qcsim::{QuantumCircuit, sample};

fn main() {
    let mut qc = QuantumCircuit::new(1);
    qc.h(0);

    println!("{:?}", sample(qc, 10000))
}

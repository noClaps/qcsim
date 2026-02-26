use qcsim::{Computer, Qubit};

fn main() {
    let q0 = Qubit::new();
    let q1 = Qubit::new();

    let mut qc = Computer::new([q0, q1]);

    qc.hadamard(0);
    qc.controlled_not(0, 1);

    let measure_count = 100_000;
    println!("{}", qc.measure(measure_count));
}

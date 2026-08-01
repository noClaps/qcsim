use qcsim::{Gate, computer::Computer};

fn main() {
    let mut qc = Computer::new(2); // |00>
    qc.apply_gate(&Gate::pauli_x(), 0); // |10>
    qc.apply_gate(&Gate::pauli_x().with_ctrl(0), 1); // |11>
    println!("{}", qc.measure())
}

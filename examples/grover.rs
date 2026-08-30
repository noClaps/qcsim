use std::{f64::consts::PI, io::stdout};

use qcsim::{Array2, Complex, Gate, QuantumCircuit, sample};
use unicode_plot::barplot;

fn main() {
    let secret_val = (1 << 15) - 1;
    let check = |x: usize| secret_val == x;

    let num_qubits = (secret_val.ilog2() + 1) as usize;
    println!("num qubits: {num_qubits}");
    let iterations = (PI / 4.0 * (num_qubits as f64).sqrt()).ceil() as usize;
    println!("iterations: {iterations}");

    let u_omega = Gate::from_diag(
        (0..1 << num_qubits).map(|i| Complex::new(-1.0, 0.0).powi(check(i) as i32)),
    );
    let mut u_s = Array2::zeros((1 << num_qubits, 1 << num_qubits));
    u_s[(0, 0)] = Complex::new(1.0, 0.0);
    u_s = u_s * 2.0 - Array2::<Complex>::eye(1 << num_qubits);
    let u_s = Gate::new(u_s);

    let mut qc = QuantumCircuit::new(num_qubits);
    qc.h(0..num_qubits);
    for _ in 0..iterations {
        qc.unitary(&u_omega, 0..num_qubits);
        qc.h(0..num_qubits);
        qc.unitary(&u_s, 0..num_qubits);
        qc.h(0..num_qubits);
    }

    let data = sample(qc, 10000);
    let &val = data.iter().max_by_key(|el| el.1).unwrap().0;
    assert!(
        check(val),
        "Grover's algorithm failed! Expected: {secret_val}, received: {val}",
    );

    let plot = barplot(
        &data.keys().collect::<Vec<_>>(),
        &data.values().collect::<Vec<_>>(),
        Default::default(),
    )
    .unwrap();
    plot.render_with_mode_auto(&mut stdout(), Default::default())
        .unwrap();
    println!("val: {val}");
}

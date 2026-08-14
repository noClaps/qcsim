use std::{f64::consts::PI, io::stdout};

use ndarray::{Array1, Array2};
use num_complex::{Complex64, c64};
use qcsim::{QuantumCircuit, sample};
use unicode_plot::barplot;

fn main() {
    let secret_val = b'a';

    let num_qubits = size_of_val(&secret_val) * 8;
    let iterations = (PI / 4.0 * (num_qubits as f64).sqrt()).ceil() as usize;

    let check = |x: u32| x == secret_val as u32;

    let u_omega = Array2::from_diag(
        &Array1::from_iter(0..1 << num_qubits).map(|&i| c64(-1, 0).powu(check(i) as u32)),
    );
    let mut u_s = Array2::zeros((1 << num_qubits, 1 << num_qubits));
    u_s[(0, 0)] = c64(1, 0);
    u_s = u_s * 2.0 - Array2::<Complex64>::eye(1 << num_qubits);

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
    let val = val as u32;
    assert_eq!(
        val, secret_val as u32,
        "Grover's algorithm failed! Expected: {secret_val}, received: {val}"
    );

    let plot = barplot(
        &data.keys().collect::<Vec<_>>(),
        &data.values().collect::<Vec<_>>(),
        Default::default(),
    )
    .unwrap();
    plot.render_with_mode_auto(&mut stdout(), Default::default())
        .unwrap();
    println!("val: {}", char::from_u32(val).unwrap());
}

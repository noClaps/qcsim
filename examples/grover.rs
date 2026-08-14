use ndarray::{Array1, Array2};
use num_complex::{Complex64, c64};
use qcsim::{QuantumCircuit, sample};

fn main() {
    const NUM_QUBITS: usize = 15;
    const SECRET_VAL: i64 = 7;
    const ITERATIONS: i64 = 4; // ceil(pi/4 * sqrt(NUM_QUBITS))

    let mut secret_db: Vec<_> = (0..1 << NUM_QUBITS).map(|_| fastrand::i64(..)).collect();
    let secret_index = fastrand::usize(..secret_db.len());
    secret_db[secret_index] = SECRET_VAL;
    let check = |x: usize| u32::from(secret_db[x] == SECRET_VAL);

    let u_omega = Array2::from_diag(
        &Array1::from_iter(0..(1usize << NUM_QUBITS)).map(|&i| c64(-1, 0).powu(check(i))),
    );
    let mut u_s = Array2::zeros((1 << NUM_QUBITS, 1 << NUM_QUBITS));
    u_s[(0, 0)] = c64(1, 0);
    u_s = u_s * 2.0 - Array2::<Complex64>::eye(1 << NUM_QUBITS);

    let mut qc = QuantumCircuit::new(NUM_QUBITS);
    qc.h(0..NUM_QUBITS);
    for _ in 0..ITERATIONS {
        qc.unitary(&u_omega, 0..NUM_QUBITS);
        qc.h(0..NUM_QUBITS);
        qc.unitary(&u_s, 0..NUM_QUBITS);
        qc.h(0..NUM_QUBITS);
    }

    let data = sample(qc, 10000);
    let &index = data.iter().max_by_key(|el| el.1).unwrap().0;
    assert_eq!(
        index, secret_index,
        "Grover's algorithm failed! Expected: {secret_index}, received: {index}"
    );
    println!("index: {index}");
}

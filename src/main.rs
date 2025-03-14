mod computer;
mod qubit;

use std::{collections::HashMap, f64::consts::SQRT_2};

use computer::Computer;
use num_complex::c64;
use qubit::Qubit;

fn main() {
    let qubit1 = Qubit::new_normal(c64(1. / SQRT_2, 0.)); // 1/sqrt(2) |0> + 1/sqrt(2) |1>
    let qubit2 = Qubit::new_normal(c64(1. / SQRT_2, 0.)); // 1/sqrt(2) |0> + 1/sqrt(2) |1>
    let qubit3 = Qubit::new_normal(c64(1. / SQRT_2, 0.)); // 1/sqrt(2) |0> + 1/sqrt(2) |1>

    let mut computer = Computer::new(vec![qubit1, qubit2, qubit3]);
    computer.toffoli(0, 1, 2);

    let mut outputs = HashMap::new();
    for _ in 0..100000 {
        let measured_state = computer.measure();
        match outputs.get_mut(&measured_state) {
            None => {
                outputs.insert(measured_state, 1);
            }
            Some(state) => *state += 1,
        };
    }
    println!("{:#?}", outputs);
}

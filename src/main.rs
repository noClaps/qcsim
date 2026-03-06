use std::{collections::HashMap, ops::AddAssign};

use qcsim::Computer;

fn main() {
    let mut qc = Computer::new(3);

    qc.hadamard(0);
    qc.controlled_not(0, 1);
    qc.toffoli(0, 1, 2);

    let mut outcomes: HashMap<String, usize> = HashMap::new();
    let measure_count = 100_000;
    for _ in 0..measure_count {
        let outcome = qc.clone().measure();
        match outcomes.get_mut(&outcome) {
            Some(o) => o.add_assign(1),
            None => {
                outcomes.insert(outcome, 1);
            }
        }
    }

    println!("{:?}", outcomes);
}

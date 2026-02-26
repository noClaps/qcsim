# Quantum Computer Simulation

This is a quantum computer simulation research project. It's probably not accurate to a real quantum computer.

## Build instructions

```sh
git clone https://github.com/noClaps/qcsim.git && cd qcsim
cargo run
```

## Usage

You should start by creating new qubits:

```rust
use qcsim::Qubit;

fn main() {
  let q0 = Qubit::new();
  let q1 = Qubit::new();
  let q2 = Qubit::new();
}
```

Then you can initialise a computer with those qubits:

```rust
use qcsim::Computer;

fn main() {
    // ...

    let mut qc = Computer::new([q0, q1, q2])
}
```

This will add your qubits to the computer in the order that you put them in the array. After this, you can use array indexing to apply operations to the qubits in the computer, using the gates you have available:

- `pauli_x`
- `pauli_y`
- `pauli_z`
- `hadamard`
- `phase`
- `pi_by_8`
- `controlled_not`
- `controlled_z`
- `swap`
- `toffoli`

For example:

```rust
fn main() {
    // ...

    qc.hadamard(0);
    qc.pauli_x(0);
    qc.pauli_x(1);
    qc.toffoli(0, 1, 2);
}
```

Once you're done, you can output the state of the system using `measure`:

```rust
fn main() {
    // ...

    let measure_count = 100_000;
    println!("{}", qc.measure(measure_count));
}
```

If the measure count is 1, then simply the state of the system will be returned, but if it is greater than 1, then a table with the state and its corresponding count will be returned.

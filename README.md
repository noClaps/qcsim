# Quantum Computer Simulation

This is a quantum computer simulation research project. It's probably not accurate to a real quantum computer.

## Build instructions

```sh
git clone https://codefloe.com/noClaps/qcsim.git && cd qcsim
cargo run
```

## Usage

You should start by initialising the quantum computer with a specified number of qubits:

```rust
use qcsim::Computer;

fn main() {
  let qc = Computer::new(3);
}
```

This will initialise them in the $\ket{00\dots0}$ state.

After this, you can use array indexing to apply operations to the qubits in the computer, using the gates you have available:

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
qc.hadamard(0);
qc.controlled_not(0, 1);
qc.toffoli(0, 1, 2);
```

Once you're done, you can output the state of the system using `measure`:

```rust
println!("{}", qc.measure());
```

This will return the measured state of the entire computer as a string, along with taking ownership of the computer instance so that no more operations can happen, since the state should only be measured at the very end.

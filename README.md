# Quantum Computer Simulation

This is a quantum computer simulation research project.

## Usage

You should start by initialising the quantum computer with a specified number of qubits:

```rust
use qcsim::QuantumCircuit;

fn main() {
  let mut qc = QuantumCircuit::new(3);
}
```

This will initialise them in the $\ket{00\dots0}$ state.

After this, you can use array indexing to apply operations to the qubits in the computer, using the gates you have available.

For example:

```rust
qc.h(0);
qc.cx(0, 1);
qc.ccx(0, 1, 2);
```

Once you're done, you can output the state of the system using `measure`:

```rust
println!("{}", qc.measure());
```

This will return the measured state of the entire computer as a string, along with taking ownership of the computer instance so that no more operations can happen, since the state should only be measured at the very end.

You can also get an entire sample of points using the `sample` function:

```rust
use qcsim::sample;

let output = sample(qc, 10000); // take 10,000 samples
println!("{:?}", output); // example output: {"000": 5024, "111": 4976}
```

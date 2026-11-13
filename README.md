# Quantum Computer Simulation

This is a quantum computer simulation research project.

## Usage

Add QCSim to your project:

```sh
cargo add --git https://github.com/noClaps/qcsim
```

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

## Benchmarks

Machine specifications:

- MacBook Air M2, 2022
- 16 GB RAM
- macOS 27.2.1 Golden Gate

Run details:

- 10000 shots with the sampler
- Python version 3.15.2
- Rust version 1.99.1
- Dependency versions are listed in [`uv.lock`](./qiskit/uv.lock) and [`Cargo.lock`](./Cargo.lock).

### Grover search

- [QCSim](./examples/grover.rs)
- [Qiskit](./qiskit/grover.py), uses Qiskit Aer `SamplerV2`

Compiled with:

```sh
cargo build --release --example grover
```

Ran with:

```sh
hyperfine ./target/release/examples/grover 'uv run qiskit/grover.py' --warmup=10
```

|           | QCSim              | Qiskit                |
| --------- | ------------------ | --------------------- |
| 12 qubits | 56.9 ms (2.1 MiB)  | 424.5 ms (112.8 MiB)  |
| 13 qubits | 134.3 ms (2.4 MiB) | 742.0 ms (162.6 MiB)  |
| 14 qubits | 274.3 ms (2.9 MiB) | 1558.8 ms (299.0 MiB) |
| 15 qubits | 614.5 ms (3.9 MiB) | 3784.5 ms (673.7 MiB) |
| 16 qubits | 1.442 s (6.1 MiB)  | 10.117 s (1923.5 MiB) |
| 17 qubits | 3.584 s (10.3 MiB) | 28.451 s (4489.6 MiB) |
| 18 qubits | 9.313 s (17.9 MiB) | 82.591 s (5425.7 MiB) |

> [!note]
> Each additional qubit doubles the size of the state vector and diagonal matrices, and quadruples the size of dense matrices ($2^1 \times 2^1 = 4 \to $2^2 \times 2^2 = 16 \to $2^3 \times 2^3 = 64$).

### Coin flip

- [QCSim](./examples/coin-flip.rs)
- [Qiskit](./qiskit/coin-flip.py), uses Qiskit `StatevectorSampler`

Compiled with:

```sh
cargo build --release --example coin-flip
```

Ran with:

```sh
hyperfine ./target/release/examples/coin-flip 'uv run qiskit/coin-flip.py' --warmup=20
```

| QCSim            | Qiskit              |
| ---------------- | ------------------- |
| 1.9 ms (1.8 MiB) | 221.5 ms (75.8 MiB) |

### Analysis

I have done my best to make the implementations of both algorithms identical in both Qiskit and QCSim. This is helped by the fact that QCSim's API is designed to be somewhat similar to Qiskit. However, there is always going to be a difference in the performance of a compiled program in a language like Rust and an interpreted program in a language like Python. It is entirely possible that many of the performance and memory differences may be due to the overhead of the Python runtime and other libraries like NumPy, rather than purely the Qiskit core implementation (which is also in Rust).

That said, I still believe these benchmarks are fair as, from my understanding, Python and NumPy are the intended way to use Qiskit, just as Rust is the intended way to use QCSim, so the runtime and library overhead are a cost you would pay in real usage of Qiskit, and a differentiating factor for QCSim and Rust. There are [official Rust bindings](https://github.com/Qiskit/qiskit-rs) for Qiskit's C API that may have higher performance, but I did not test them as they required cloning the entire [Qiskit/qiskit](https://github.com/Qiskit/qiskit) repository, which did not seem as friendly to use compared to the Python package.

Aside from the benefits of using Rust, QCSim also has a few other optimisations over Qiskit:

- The unitarity check is only run once on gate creation, for `Gate::new`, instead of at every call of `QuantumCircuit::unitary`. This is only possible because `Gate::new` enforces unitarity, meaning a non-unitary matrix can never become a gate in the first place. `QuantumCircuit::unitary` only accepts `Gate` values. In Qiskit, the unitarity check runs on every call to `QuantumCircuit.unitary` and in `UnitaryGate` construction.
- Gate matrices are automatically further optimised if diagonal. This is possible with Qiskit as well with [`DiagonalGate`](https://quantum.cloud.ibm.com/docs/en/api/qiskit/qiskit.circuit.library.DiagonalGate), and has been done in the Grover example, but is opt-in and requires specifically constructing an iterable of the diagonal elements.
  - In diagonal matrices, only the diagonal elements are stored. This massively reduces the amount of memory and computation that is required. For instance, a full dense square matrix that acts upon an 8-qubit state would need to store $2^8 \times 2^8$, or 65536, elements. Each of those is a 128-bit complex number (2 64-bit floating point numbers, one for the real part, one for the imaginary), resulting in a total memory usage of 1 MiB. If the matrix is diagonal, however, it is possible to store only the diagonal elements, of which there are $2^8$, or 256. This brings the memory usage down to 4 KiB.

    This scales incredibly well as the size of the matrix increases. There is a condition when running `Gate::new` that checks if the matrix is diagonal, and optimises it to this form if so. However, you still pay the cost once of constructing the dense matrix. If your matrix is known to be diagonal, it's usually a better idea to construct the gate using `Gate::from_diag` to avoid that memory cost. This has been done in the QCSim Grover example.

  - Diagonal matrices also have their own optimised branches in the code. For instance, multiplying a diagonal matrix with a vector is a significantly easier operation that can be done in $O(n)$ time as opposed to the $O(n^2)$ time that a full square matrix would require, where $n$ is the number of elements in the vector. This further improves the performance and memory usage of diagonal matrices.

The implementations have been optimised to the best of my ability and knowledge. There may be more performance left on the table for each one.

# Quantum Computer Simulation

This is a quantum computer simulation research project. It's probably not accurate to a real quantum computer.

## Build instructions

```sh
git clone https://gitlab.com/noClaps/qcsim.git && cd qcsim
mise install # https://mise.jdx.dev
mise build
```

You can then run it using `./qcsim`.

## Usage

You can tweak the qubits and algorithm in `src/main.rs` before building. Running the output binary (or simply running `cargo run` to run the debug version) will then return a map of outputs and the number of times they occurred in 100000 runs.

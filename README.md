# Quantum Computer Simulation

This is a quantum computer simulation research project. It's probably not accurate to a real quantum computer.

## Build instructions

```sh
git clone https://github.com/noClaps/qcsim.git && cd qcsim
mise install # https://mise.jdx.dev
mise build
```

## Usage

```
USAGE: qcsim <input>

ARGUMENTS:
  <input>     QC Instructions file to be run

OPTIONS:
  -h, --help  Display this help message and exit
```

You can run a QC instructions file with `qcsim path/to/file.qc`. The syntax for it is described in the [QC instructions syntax](#qc-instructions-syntax) section.

## QC instructions syntax

You have to start by defining your qubits. The syntax for this is:

```
qubit = |0>
```

You can also add values in front of your qubits, and have a superposition of both `|0>` and `|1>` units, like so:

```
qubit = 0.707 |0>, 0.707 |1>
```

You can add (`+`), subtract (`-`), multiply (`*`), divide (`/`) or exponent (`^`) values together.

You also have access to some functions:

- `sin(val)`: The sine function. The argument can be a complex number.
- `cos(val)`: The cosine function. The argument can be a complex number.
- `tan(val)`: The tangent function. The argument can be a complex number.
- `root(val, power)`: The root function. Both arguments can be complex numbers.

and some constants:

- `PI`: Archimedes' constant, $\pi$.
- `E`: Euler's number, $e$.
- `I`: Imaginary unit, $i$.

You can use these to create more complex qubits, such as:

```
q1 = cos(PI/2) |0>, I * sin(PI/2) |1>
q2 = (1 / root(2, 2)) |0>, (1 / root(2, 2)) |1>
```

Variable names can only contain letters and numbers. Qubits must also be normalised, and the code will tell you if the qubits you've defined are normalised or not.

Once you've defined your qubits, you can start applying them to quantum logic gates. Here are the gates you have available:

- `x(q1)`: The Pauli X gate.
- `y(q1)`: The Pauli Y gate.
- `z(q1)`: The Pauli Z gate.
- `hadamard(q1)`: The Hadamard gate.
- `phase(q1)`: The phase gate.
- `pi_8(q1)`: The π/8 gate.
- `cnot(q1, q2)`: The controlled NOT gate.
- `cz(q1, q2)`: The controlled Z gate.
- `swap(q1, q2)`: The SWAP gate.
- `toffoli(q1, q2, q3)`: The Toffoli gate.

You can also call the `measure()` function to measure the qubit at any point, with an optional argument like `measure(1000)` to specify how many times to repeat the measurement.

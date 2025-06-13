# Quantum Computer Simulation

This is a quantum computer simulation research project. It's probably not accurate to a real quantum computer.

## Build instructions

You'll need to have [Go](https://go.dev) installed.

```sh
git clone https://github.com/qcsim/qcsim.git && cd qcsim
go run .
```

## Usage

You should start by creating new qubits:

```go
import (
	"math/cmplx"

	"github.com/qcsim/qcsim/qubit"
)

func main() {
	q0 := qubit.New(1/cmplx.Sqrt(2), 1/cmplx.Sqrt(2))
	q1 := qubit.New(1, 0)
	q2 := qubit.New(0, 1)
}
```

Then you can initialise a computer with those qubits:

```go
import (
	// ...

	"github.com/qcsim/qcsim/computer"
)

func main() {
	// ...

	qc := computer.New([]qubit.Qubit{q0, q1, q2})
}
```

This will add your qubits to the computer in the order that you put them in the array. After this, you can use array indexing to apply operations to the qubits in the computer, using the gates you have available:

- `PauliX`
- `PauliY`
- `PauliZ`
- `Hadamard`
- `Phase`
- `PiBy8`
- `ControlledNot`
- `ControlledZ`
- `Swap`
- `Toffoli`

For example:

```go
func main() {
	// ...

	if err := qc.Hadamard(0); err != nil {
		slog.Error("Error performing Hadamard", "error", err)
	}
	if err := qc.PauliX(0); err != nil {
		slog.Error("Error performing Pauli X", "error", err)
	}
	if err := qc.PauliX(1); err != nil {
		slog.Error("Error performing Pauli X", "error", err)
	}
	if err := qc.Toffoli(0, 1, 2); err != nil {
		slog.Error("Error performing Toffoli", "error", err)
	}
}
```

Once you're done, you can output the state of the system using `Measure`:

```go
func main() {
	// ...

	var measureCount uint = 100000
	fmt.Println(qc.Measure(measureCount))
}
```

If the measure count is 1, then simply the state of the system will be returned, but if it is greater than 1, then a table with the state and its corresponding count will be returned.

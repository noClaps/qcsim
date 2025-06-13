package computer

import (
	"encoding/json"
	"fmt"
	"log/slog"
	"math"
	"math/cmplx"
	"strings"

	"github.com/qcsim/qcsim/qubit"
)

type computer struct {
	state []qubit.Qubit
}

const i = complex(0, 1)

// Creates a new quantum computer.
func New(qubits []qubit.Qubit) computer {
	return computer{state: qubits}
}

// Returns the measured states as a string. The returned string will have the
// measured value of the first qubit as the first character, the second qubit
// as the second character, and so on for as many qubits as were input into the
// computer. The count input determines how many times to measure. If the count
// is 1, then only the measured state is returned, but if it is greater than 1,
// then a table of the states and the number of times they appeared is
// returned.
func (c *computer) Measure(count uint) string {
	outputs := make(map[string]uint)
	for range count {
		measuredState := ""
		for _, q := range c.state {
			measuredState += fmt.Sprint(q.Measure())
		}

		if count == 1 {
			return measuredState
		}

		if val, ok := outputs[measuredState]; ok {
			outputs[measuredState] = val + 1
		} else {
			outputs[measuredState] = 1
		}
	}

	b, err := json.MarshalIndent(outputs, "", "")
	if err != nil {
		slog.Error("Error formatting output", "error", err)
	}
	output := strings.ReplaceAll(string(b), ",", "")
	return strings.Trim(output, "{\n}")
}

// Applies the Pauli X operator on the qubit. The index starts at 0 for the
// first index.
func (c *computer) PauliX(index uint) error {
	matrix := [2][2]complex128{
		{0, 1},
		{1, 0},
	}
	return c.apply1(index, matrix)
}

// Applies the Pauli Y operator on the qubit. The index starts at 0 for the
// first index.
func (c *computer) PauliY(index uint) error {
	matrix := [2][2]complex128{
		{0, -i},
		{i, 0},
	}
	return c.apply1(index, matrix)
}

// Applies the Pauli Z operator on the qubit. The index starts at 0 for the
// first index.
func (c *computer) PauliZ(index uint) error {
	matrix := [2][2]complex128{
		{1, 0},
		{0, -1},
	}
	return c.apply1(index, matrix)
}

// Applies the Hadamard operator on the qubit. The index starts at 0 for the
// first index.
func (c *computer) Hadamard(index uint) error {
	matrix := [2][2]complex128{
		{1 / math.Sqrt2, 1 / math.Sqrt2},
		{1 / math.Sqrt2, -1 / math.Sqrt2},
	}
	return c.apply1(index, matrix)
}

// Applies the Phase operator on the qubit. The index starts at 0 for the first
// index.
func (c *computer) Phase(index uint) error {
	matrix := [2][2]complex128{
		{1, 0},
		{0, i},
	}
	return c.apply1(index, matrix)
}

// Applies the π/8 operator on the qubit. The index starts at 0 for the first
// index.
func (c *computer) PiBy8(index uint) error {
	matrix := [2][2]complex128{
		{1, 0},
		{0, cmplx.Rect(1, math.Exp(math.Pi/4))},
	}
	return c.apply1(index, matrix)
}

// Applies the Controlled Not operator on the qubits. The index starts at 0 for
// the first index.
func (c *computer) ControlledNot(index1 uint, index2 uint) error {
	matrix := [4][4]complex128{
		{1, 0, 0, 0},
		{0, 1, 0, 0},
		{0, 0, 0, 1},
		{0, 0, 1, 0},
	}
	return c.apply2(index1, index2, matrix)
}

// Applies the Controlled Z operator on the qubits. The index starts at 0 for
// the first index.
func (c *computer) ControlledZ(index1 uint, index2 uint) error {
	matrix := [4][4]complex128{
		{1, 0, 0, 0},
		{0, 1, 0, 0},
		{0, 0, 1, 0},
		{0, 0, 0, -1},
	}
	return c.apply2(index1, index2, matrix)
}

// Applies the Swap operator on the qubits. The index starts at 0 for the first
// index.
func (c *computer) Swap(index1 uint, index2 uint) error {
	matrix := [4][4]complex128{
		{1, 0, 0, 0},
		{0, 0, 1, 0},
		{0, 1, 0, 0},
		{0, 0, 0, 1},
	}
	return c.apply2(index1, index2, matrix)
}

// Applies the Toffoli operator on the qubits. The index starts at 0 for the
// first index.
func (c *computer) Toffoli(index1 uint, index2 uint, index3 uint) error {
	matrix := [8][8]complex128{
		{1, 0, 0, 0, 0, 0, 0, 0},
		{0, 1, 0, 0, 0, 0, 0, 0},
		{0, 0, 1, 0, 0, 0, 0, 0},
		{0, 0, 0, 1, 0, 0, 0, 0},
		{0, 0, 0, 0, 1, 0, 0, 0},
		{0, 0, 0, 0, 0, 1, 0, 0},
		{0, 0, 0, 0, 0, 0, 0, 1},
		{0, 0, 0, 0, 0, 0, 1, 0},
	}
	return c.apply3(index1, index2, index3, matrix)
}

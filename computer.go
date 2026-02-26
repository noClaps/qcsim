package main

import (
	"fmt"
	"math"
	"math/cmplx"
	"strings"
)

func matmul(matrix [][]complex128, vector []complex128) []complex128 {
	if len(matrix) != len(matrix[0]) {
		panic("matrix should be square")
	}
	if len(matrix) != len(vector) {
		panic("matrix size should match vector size")
	}

	outVec := make([]complex128, len(vector))
	for r, row := range matrix {
		var rowSum complex128 = 0
		for j := range row {
			rowSum += row[j] * vector[j]
		}
		outVec[r] = rowSum
	}
	return outVec
}

type Computer struct {
	qubits []Qubit
}

const i complex128 = complex(0, 1)

// Creates a new quantum computer.
func NewComputer(qubits ...Qubit) Computer {
	return Computer{qubits: qubits}
}

func (c *Computer) apply(matrix [][]complex128, index ...uint) {
	switch len(index) {
	case 1:
		q := c.qubits[index[0]]

		inputAmps := []complex128{q.Zero, q.One}
		outputAmps := matmul(matrix, inputAmps)

		c.qubits[index[0]] = Qubit{Zero: outputAmps[0], One: outputAmps[1]}
	case 2:
		q0 := c.qubits[index[0]]
		q1 := c.qubits[index[1]]

		inputAmps := []complex128{
			q0.Zero * q1.Zero, // 00
			q0.Zero * q1.One,  // 01
			q0.One * q1.Zero,  // 10
			q0.One * q1.One,   // 11
		}
		outputAmps := matmul(matrix, inputAmps)

		c.qubits[index[0]] = Qubit{
			Zero: outputAmps[0] + outputAmps[1],
			One:  outputAmps[2] + outputAmps[3],
		}
		c.qubits[index[1]] = Qubit{
			Zero: outputAmps[0] + outputAmps[2],
			One:  outputAmps[1] + outputAmps[3],
		}
	case 3:
		q0 := c.qubits[index[0]]
		q1 := c.qubits[index[1]]
		q2 := c.qubits[index[2]]

		inputAmps := []complex128{
			q0.Zero * q1.Zero * q2.Zero, // 000
			q0.Zero * q1.Zero * q2.One,  // 001
			q0.Zero * q1.One * q2.Zero,  // 010
			q0.Zero * q1.One * q2.One,   // 011
			q0.One * q1.Zero * q2.Zero,  // 100
			q0.One * q1.Zero * q2.One,   // 101
			q0.One * q1.One * q2.Zero,   // 110
			q0.One * q1.One * q2.One,    // 111
		}
		outputAmps := matmul(matrix, inputAmps)

		c.qubits[index[0]] = Qubit{
			Zero: outputAmps[0] + outputAmps[1] + outputAmps[2] + outputAmps[3],
			One:  outputAmps[4] + outputAmps[5] + outputAmps[6] + outputAmps[7],
		}
		c.qubits[index[1]] = Qubit{
			Zero: outputAmps[0] + outputAmps[1] + outputAmps[4] + outputAmps[5],
			One:  outputAmps[2] + outputAmps[3] + outputAmps[6] + outputAmps[7],
		}
		c.qubits[index[2]] = Qubit{
			Zero: outputAmps[0] + outputAmps[2] + outputAmps[4] + outputAmps[6],
			One:  outputAmps[1] + outputAmps[3] + outputAmps[5] + outputAmps[7],
		}
	default:
		panic("Cannot have more than 3 indices")
	}
}

// Returns the measured states as a string. The returned string will have the measured value of the
// first qubit as the first character, the second qubit as the second character, and so on for as
// many qubits as were input into the computer. The count input determines how many times to
// measure. If the count is 1, then only the measured state is returned, but if it is greater than
// 1, then a table of the states and the number of times they appeared is returned.
func (c *Computer) Measure(count uint) string {
	states := make(map[string]uint)
	for range count {
		measuredState := ""
		for _, q := range c.qubits {
			measuredState += fmt.Sprint(q.Measure())
		}

		if count == 1 {
			return measuredState
		}

		states[measuredState] += 1
	}

	var output strings.Builder
	for k, v := range states {
		fmt.Fprintf(&output, "%s: %v\n", k, v)
	}
	return strings.TrimSpace(output.String())
}

// Applies the Pauli X operator on the qubit. The index starts at 0 for the first index.
func (c *Computer) PauliX(index uint) {
	matrix := [][]complex128{
		{0, 1},
		{1, 0},
	}
	c.apply(matrix, index)
}

// Applies the Pauli Y operator on the qubit. The index starts at 0 for the first index.
func (c *Computer) PauliY(index uint) {
	matrix := [][]complex128{
		{0, -i},
		{i, 0},
	}
	c.apply(matrix, index)
}

// Applies the Pauli Z operator on the qubit. The index starts at 0 for the first index.
func (c *Computer) PauliZ(index uint) {
	matrix := [][]complex128{
		{1, 0},
		{0, -1},
	}
	c.apply(matrix, index)
}

// Applies the Hadamard operator on the qubit. The index starts at 0 for the first index.
func (c *Computer) Hadamard(index uint) {
	matrix := [][]complex128{
		{1 / math.Sqrt2, 1 / math.Sqrt2},
		{1 / math.Sqrt2, -1 / math.Sqrt2},
	}
	c.apply(matrix, index)
}

// Applies the Phase operator on the qubit. The index starts at 0 for the first index.
func (c *Computer) Phase(index uint) {
	matrix := [][]complex128{
		{1, 0},
		{0, i},
	}
	c.apply(matrix, index)
}

// Applies the π/8 operator on the qubit. The index starts at 0 for the first index.
func (c *Computer) PiBy8(index uint) {
	matrix := [][]complex128{
		{1, 0},
		{0, cmplx.Rect(1, math.Exp(math.Pi/4))},
	}
	c.apply(matrix, index)
}

// Applies the Controlled Not operator on the qubits. The index starts at 0 for the first index.
func (c *Computer) ControlledNot(index1 uint, index2 uint) {
	matrix := [][]complex128{
		{1, 0, 0, 0},
		{0, 1, 0, 0},
		{0, 0, 0, 1},
		{0, 0, 1, 0},
	}
	c.apply(matrix, index1, index2)
}

// Applies the Controlled Z operator on the qubits. The index starts at 0 for the first index.
func (c *Computer) ControlledZ(index1 uint, index2 uint) {
	matrix := [][]complex128{
		{1, 0, 0, 0},
		{0, 1, 0, 0},
		{0, 0, 1, 0},
		{0, 0, 0, -1},
	}
	c.apply(matrix, index1, index2)
}

// Applies the Swap operator on the qubits. The index starts at 0 for the first index.
func (c *Computer) Swap(index1 uint, index2 uint) {
	matrix := [][]complex128{
		{1, 0, 0, 0},
		{0, 0, 1, 0},
		{0, 1, 0, 0},
		{0, 0, 0, 1},
	}
	c.apply(matrix, index1, index2)
}

// Applies the Toffoli operator on the qubits. The index starts at 0 for the first index.
func (c *Computer) Toffoli(index1 uint, index2 uint, index3 uint) {
	matrix := [][]complex128{
		{1, 0, 0, 0, 0, 0, 0, 0},
		{0, 1, 0, 0, 0, 0, 0, 0},
		{0, 0, 1, 0, 0, 0, 0, 0},
		{0, 0, 0, 1, 0, 0, 0, 0},
		{0, 0, 0, 0, 1, 0, 0, 0},
		{0, 0, 0, 0, 0, 1, 0, 0},
		{0, 0, 0, 0, 0, 0, 0, 1},
		{0, 0, 0, 0, 0, 0, 1, 0},
	}
	c.apply(matrix, index1, index2, index3)
}

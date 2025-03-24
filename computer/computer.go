package computer

import (
	"fmt"
	"math"
	"math/cmplx"
	"math/rand"

	"github.com/noclaps/qcsim/qubit"
)

type Computer struct {
	state []qubit.Qubit
}

const I = complex(0, 1)

// Creates a new quantum computer.
func New(qubits []qubit.Qubit) Computer {
	return Computer{state: qubits}
}

// Returns the measured states as a string. The returned string will have the
// measured value of the first qubit as the first character, the second qubit
// as the second character, and so on for as many qubits as were input into the
// computer.
func (c *Computer) Measure() string {
	output := ""
	for _, q := range c.state {
		randomVal := rand.Float64()
		zeroProb := q.ProbabilityZero()
		if randomVal <= zeroProb {
			output += fmt.Sprint(0)
		} else {
			output += fmt.Sprint(1)
		}
	}
	return output
}

func (c *Computer) PauliX(index uint) error {
	matrix := [2][2]complex128{
		{0, 1},
		{1, 0},
	}
	return c.apply1(index, matrix)
}

func (c *Computer) PauliY(index uint) error {
	matrix := [2][2]complex128{
		{0, -I},
		{I, 0},
	}
	return c.apply1(index, matrix)
}

func (c *Computer) PauliZ(index uint) error {
	matrix := [2][2]complex128{
		{1, 0},
		{0, -1},
	}
	return c.apply1(index, matrix)
}

func (c *Computer) Hadamard(index uint) error {
	matrix := [2][2]complex128{
		{1 / math.Sqrt2, 1 / math.Sqrt2},
		{1 / math.Sqrt2, -1 / math.Sqrt2},
	}
	return c.apply1(index, matrix)
}

func (c *Computer) Phase(index uint) error {
	matrix := [2][2]complex128{
		{1, 0},
		{0, I},
	}
	return c.apply1(index, matrix)
}

func (c *Computer) PiBy8(index uint) error {
	matrix := [2][2]complex128{
		{1, 0},
		{0, cmplx.Rect(1, math.Exp(math.Pi/4))},
	}
	return c.apply1(index, matrix)
}

func (c *Computer) ControlledNot(index1 uint, index2 uint) error {
	matrix := [4][4]complex128{
		{1, 0, 0, 0},
		{0, 1, 0, 0},
		{0, 0, 0, 1},
		{0, 0, 1, 0},
	}
	return c.apply2(index1, index2, matrix)
}

func (c *Computer) ControlledZ(index1 uint, index2 uint) error {
	matrix := [4][4]complex128{
		{1, 0, 0, 0},
		{0, 1, 0, 0},
		{0, 0, 1, 0},
		{0, 0, 0, -1},
	}
	return c.apply2(index1, index2, matrix)
}

func (c *Computer) Swap(index1 uint, index2 uint) error {
	matrix := [4][4]complex128{
		{1, 0, 0, 0},
		{0, 0, 1, 0},
		{0, 1, 0, 0},
		{0, 0, 0, 1},
	}
	return c.apply2(index1, index2, matrix)
}

func (c *Computer) Toffoli(index1 uint, index2 uint, index3 uint) error {
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

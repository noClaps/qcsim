package computer

import (
	"fmt"

	"github.com/qcsim/qcsim/qubit"
)

func (c *computer) apply1(index uint, matrix [2][2]complex128) error {
	if len(c.state) < 1 {
		return fmt.Errorf("Not enough qubits in computer")
	}
	if index >= uint(len(c.state)) {
		return fmt.Errorf("Index greater than number of qubits")
	}

	q := c.state[index]
	zero := q.Zero*matrix[0][0] + q.One*matrix[0][1]
	one := q.Zero*matrix[1][0] + q.One*matrix[1][1]

	newQubit := qubit.New(zero, one)
	c.state[index] = newQubit
	return nil
}

func (c *computer) apply2(index1 uint, index2 uint, matrix [4][4]complex128) error {
	if len(c.state) < 2 {
		return fmt.Errorf("Not enough qubits in computer")
	}
	if index1 >= uint(len(c.state)) || index2 >= uint(len(c.state)) {
		return fmt.Errorf("Index greater than number of qubits")
	}

	q1 := c.state[index1]
	q2 := c.state[index2]
	inputAmps := []complex128{
		q1.Zero * q2.Zero, // 00
		q1.Zero * q2.One,  // 01
		q1.One * q2.Zero,  // 10
		q1.One * q2.One,   // 11
	}

	outputAmps := [4]complex128{}
	for i, amp := range inputAmps {
		for o := range outputAmps {
			outputAmps[o] += matrix[o][i] * amp
		}
	}

	newQ1 := qubit.New(outputAmps[0]+outputAmps[1], outputAmps[2]+outputAmps[3])
	newQ2 := qubit.New(outputAmps[0]+outputAmps[2], outputAmps[1]+outputAmps[3])

	c.state[index1] = newQ1
	c.state[index2] = newQ2

	return nil
}

func (c *computer) apply3(index1 uint, index2 uint, index3 uint, matrix [8][8]complex128) error {
	if len(c.state) < 3 {
		return fmt.Errorf("Not enough qubits in computer")
	}
	if index1 >= uint(len(c.state)) || index2 >= uint(len(c.state)) || index3 >= uint(len(c.state)) {
		return fmt.Errorf("Index greater than number of qubits")
	}

	q1 := c.state[index1]
	q2 := c.state[index2]
	q3 := c.state[index3]

	inputAmps := []complex128{
		q1.Zero * q2.Zero * q3.Zero, // 000
		q1.Zero * q2.Zero * q3.One,  // 001
		q1.Zero * q2.One * q3.Zero,  // 010
		q1.Zero * q2.One * q3.One,   // 011
		q1.One * q2.Zero * q3.Zero,  // 100
		q1.One * q2.Zero * q3.One,   // 101
		q1.One * q2.One * q3.Zero,   // 110
		q1.One * q2.One * q3.One,    // 111
	}

	outputAmps := [8]complex128{}
	for i, amp := range inputAmps {
		for o := range outputAmps {
			outputAmps[o] += matrix[o][i] * amp
		}
	}

	newQ1 := qubit.New(outputAmps[0]+outputAmps[1]+outputAmps[2]+outputAmps[3], outputAmps[4]+outputAmps[5]+outputAmps[6]+outputAmps[7])
	newQ2 := qubit.New(outputAmps[0]+outputAmps[1]+outputAmps[4]+outputAmps[5], outputAmps[2]+outputAmps[3]+outputAmps[6]+outputAmps[7])
	newQ3 := qubit.New(outputAmps[0]+outputAmps[2]+outputAmps[4]+outputAmps[6], outputAmps[1]+outputAmps[3]+outputAmps[5]+outputAmps[7])

	c.state[index1] = newQ1
	c.state[index2] = newQ2
	c.state[index3] = newQ3

	return nil
}

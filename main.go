package main

import (
	"fmt"
)

func main() {
	q0 := NewQubit()
	q1 := NewQubit()
	qc := NewComputer(q0, q1)

	qc.Hadamard(0)
	qc.ControlledNot(0, 1)

	var measureCount uint = 100000
	fmt.Println(qc.Measure(measureCount))
}

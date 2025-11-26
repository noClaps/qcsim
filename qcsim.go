package main

import (
	"fmt"
	"log/slog"
	"math/cmplx"

	"github.com/noclaps/qcsim/computer"
	"github.com/noclaps/qcsim/qubit"
)

func main() {
	q0 := qubit.New(1/cmplx.Sqrt(2), 1/cmplx.Sqrt(2))
	q1 := qubit.New(1, 0)
	q2 := qubit.New(0, 1)
	q3 := qubit.New(1, 1)

	qc := computer.New([]qubit.Qubit{q0, q1, q2, q3})
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

	var measureCount uint = 100000
	fmt.Println(qc.Measure(measureCount)) // returns "110"
}

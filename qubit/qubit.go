package qubit

import (
	"fmt"
	"math"
	"math/cmplx"
	"math/rand/v2"
)

type Qubit struct {
	Zero complex128
	One  complex128
}

// Creates a new qubit with the coefficients of |0> and |1> as inputs.
func New(zero complex128, one complex128) Qubit {
	return Qubit{zero, one}
}

func (q *Qubit) IsNormalised() bool {
	return math.Abs(1-(cmplx.Abs(q.Zero)*cmplx.Abs(q.Zero)+cmplx.Abs(q.One)*cmplx.Abs(q.One))) < 0.0001
}

// Measures the qubit using a random number generator and returns either a 0 or
// a 1 to signify the state of the qubit. Note that this function does
// not check if the qubit is normalised.
func (q *Qubit) Measure() uint {
	randomVal := rand.Float64()
	zeroProb := cmplx.Abs(q.Zero) * cmplx.Abs(q.Zero)
	if randomVal <= zeroProb {
		return 0
	} else {
		return 1
	}
}

func (q Qubit) String() string {
	return fmt.Sprintf("%v |0> + %v |1>", q.Zero, q.One)
}

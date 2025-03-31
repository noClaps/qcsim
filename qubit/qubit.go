package qubit

import (
	"fmt"
	"math"
	"math/cmplx"
)

type Qubit struct {
	Zero complex128
	One  complex128
}

// Creates a new qubit with the coefficients of |0> and |1> as inputs. If it is
// not normalised, an error will be returned.
func New(zero complex128, one complex128) Qubit {
	return Qubit{zero, one}
}

func (q *Qubit) IsNormalised() bool {
	return math.Abs(1-(cmplx.Abs(q.Zero)*cmplx.Abs(q.Zero)+cmplx.Abs(q.One)*cmplx.Abs(q.One))) < 0.0001
}

// Returns the probability of the qubit returning zero when measured. This
// should be used with a random number generator between 0.0 and 1.0 to set the
// threshold. Below that threshold, 0 is returned, and above it, 1.
func (q *Qubit) ProbabilityZero() float64 {
	return cmplx.Abs(q.Zero) * cmplx.Abs(q.Zero)
}

func (q Qubit) String() string {
	return fmt.Sprintf("%v |0> + %v |1>", q.Zero, q.One)
}

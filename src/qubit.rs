use std::fmt::Display;

use crate::complex::Complex;

#[derive(Clone, Copy)]
pub struct Qubit(pub(crate) Complex, pub(crate) Complex);

impl Qubit {
    /// Creates a new qubit initialised to |0>
    pub fn new() -> Self {
        Self(Complex::ONE, Complex::ZERO)
    }

    /// Measures the qubit using a random number generator and returns either a 0 or a 1 to signify
    /// the state of the qubit. Note that this function does not check if the qubit is normalised.
    pub fn measure(&self) -> u8 {
        let random_val = fastrand::f64();
        let zero_prob = self.0.abs2();
        u8::from(random_val > zero_prob)
    }
}

impl Display for Qubit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} |0> + {} |1>", self.0, self.1)
    }
}

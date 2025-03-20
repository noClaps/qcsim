use num_complex::Complex64;
use std::process::exit;

#[derive(Debug, Clone, Copy)]
pub struct Qubit {
    pub zero: Complex64,
    pub one: Complex64,
}

impl Qubit {
    /// Creates a new qubit with the coefficients of |0> and |1> as inputs. If
    /// it is not already normalised, it will be normalised.
    pub fn new(zero: Complex64, one: Complex64) -> Self {
        let new_qubit = Self { zero, one };
        if !new_qubit.is_normalised() {
            eprintln!("[ERROR] Qubit is not normalised: {:?}", new_qubit);
            exit(1);
        }

        new_qubit
    }

    /// Returns the probability of the qubit returning zero when measured. This
    /// should be used with a random number generator between 0.0 and 1.0 to
    /// set the threshold. Below that threshold, 0 is returned, and above it,
    /// 1.
    pub fn probability_zero(self) -> f64 {
        self.zero.norm_sqr()
    }

    fn is_normalised(&self) -> bool {
        (1.0 - (self.zero.norm_sqr() + self.one.norm_sqr())).abs() < 0.0001
    }
}

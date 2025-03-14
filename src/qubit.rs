use num_complex::Complex64;

#[derive(Debug, Clone, Copy)]
pub struct Qubit {
    pub zero: Complex64,
    pub one: Complex64,
}

impl Qubit {
    /// Creates a new qubit with the coefficients of |0> and |1> as inputs. If
    /// it is not already normalised, it will be normalised.
    pub fn new(zero: Complex64, one: Complex64) -> Self {
        let mut new_qubit = Self { zero, one };
        if !new_qubit.is_normalised() {
            return new_qubit.normalise();
        }

        new_qubit
    }

    /// Creates a new normalised qubit with the coefficient of |0> as an input.
    pub fn new_normal(zero: Complex64) -> Self {
        // |b|^2 = 1 - |a|^2
        let b_sqr = 1. - zero.norm_sqr();
        let one = Complex64::from_polar(b_sqr.sqrt(), 0.);
        let new_qubit = Self { zero, one };

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

    fn normalise(&mut self) -> Self {
        let norm = (self.zero.norm_sqr() + self.one.norm_sqr()).sqrt();
        Self {
            zero: self.zero / norm,
            one: self.one / norm,
        }
    }
}

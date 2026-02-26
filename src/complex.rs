use std::{
    f64::consts::SQRT_2,
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Neg},
};

/// Complex(real: f64, imag: f64)
#[derive(Clone, Copy)]
pub struct Complex(f64, f64);

impl Complex {
    pub const ZERO: Self = Complex(0.0, 0.0);
    pub const ONE: Self = Complex(1.0, 0.0);
    pub const I: Self = Complex(0.0, 1.0);
    pub const SQRT_2: Self = Complex(SQRT_2, 0.0);

    /// Creates a complex number of the form `r*exp(i*theta)`
    pub fn rect(r: f64, theta: f64) -> Self {
        let (s, c) = theta.sin_cos();
        Self(r * c, r * s)
    }

    pub fn conj(&self) -> Self {
        Self(self.0, -self.1)
    }

    /// Returns |z|^2
    pub fn abs2(&self) -> f64 {
        self.0.powi(2) + self.1.powi(2)
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}{}{}i)",
            self.0,
            if self.1.is_sign_positive() { "+" } else { "" },
            self.1
        )
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        // a+bi * c+di = ac + adi + bci -bd
        Self(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Complex(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        // a+bi / c+di = (a+bi)(c-di) / (c+di)(c-di) = (a+bi)(c-di)/|c+di|^2
        self * rhs.conj() / rhs.abs2()
    }
}

impl Div<f64> for Complex {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

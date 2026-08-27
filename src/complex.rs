use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub(crate) const I: Complex = Complex::I;
pub(crate) fn cis(phase: f64) -> Complex {
    Complex::cis(phase)
}

/// A complex number
#[derive(Clone, Copy, Debug)]
pub struct Complex {
    /// The real part of the [`Complex`] number
    pub real: f64,
    /// The imaginary part of the [`Complex`] number
    pub imag: f64,
}

impl Complex {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 0.0);
    pub const I: Self = Self::new(0.0, 1.0);

    /// Create a new [`Complex`]
    pub const fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    /// Create a new [`Complex`] with a given phase: `exp(i * phase)`
    pub fn cis(phase: f64) -> Self {
        Self::new(phase.cos(), phase.sin())
    }

    /// Returns the square of the norm `real^2 + imag^2`
    pub const fn norm_sqr(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    /// Computes the cosine of [`Complex`]
    pub fn cos(&self) -> Self {
        // z = x + iy
        // cos(z) = cos(x) cosh(y) - i sin(x) sinh(y)
        // re: cos(re) cosh(im)
        // im: -sin(re) sinh(im)
        Self::new(
            self.real.cos() * self.imag.cosh(),
            -self.real.sin() * self.imag.sinh(),
        )
    }

    /// Returns the complex conjugate, `real - i imag`
    pub const fn conj(&self) -> Self {
        Self::new(self.real, -self.imag)
    }

    pub fn powi(&self, exp: i32) -> Self {
        (0..exp).fold(Self::ONE, |acc, _| acc * *self)
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.real, -self.imag)
    }
}

impl<T: Into<Complex>> Add<T> for Complex {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.real + rhs.real, self.imag + rhs.imag)
    }
}

impl<T: Into<Complex>> AddAssign<T> for Complex {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.real += rhs.real;
        self.imag += rhs.imag
    }
}

impl<T: Into<Complex>> Mul<T> for Complex {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        // z1 = x1 + i y1
        // z2 = x2 + i y2
        // z1 * z2 = (x1 + i y1)(x2 + i y2)
        //         = x1 x2 + i x1 y2 + i y1 x2 - y1 y2
        //         = (x1 x2 - y1 y2) + i (x1 y2 + y1 x2)
        Self::new(
            self.real * rhs.real - self.imag * rhs.imag,
            self.real * rhs.imag + self.imag * rhs.real,
        )
    }
}

impl<T: Into<Complex>> MulAssign<T> for Complex {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.real = self.real * rhs.real - self.imag * rhs.imag;
        self.imag = self.real * rhs.imag + self.imag * rhs.real;
    }
}

impl<T: Into<Complex>> Sub<T> for Complex {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self::new(self.real - rhs.real, self.imag - rhs.imag)
    }
}
impl<T: Into<Complex>> SubAssign<T> for Complex {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}

impl<T: Into<Complex>> Div<T> for Complex {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        // z1 = x1 + i y1
        // z2 = x2 + i y2
        // z1 / z2 = (x1 + i y1)/(x2 + i y2)
        //         = (x1 + i y1)(x2 - i y2)/(x2 + i y2)(x2 - i y2)
        //         = z1 z2^* / |z2|^2
        let num = self * rhs.conj();
        let den = rhs.norm_sqr();
        Self::new(num.real / den, num.imag / den)
    }
}

impl<T: Into<f64>> From<T> for Complex {
    fn from(value: T) -> Self {
        Self::new(value.into(), 0.0)
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        // derived from `numpy.isclose`
        const RTOL: f64 = 1e-5;
        const ATOL: f64 = 1e-8;
        let real_eq = (self.real - other.real).abs() <= (ATOL + RTOL * other.real.abs());
        let imag_eq = (self.imag - other.imag).abs() <= (ATOL + RTOL * other.imag.abs());
        real_eq && imag_eq
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Complex { real, imag: 0.0 } => write!(f, "{real}"),
            Complex { real: 0.0, imag } => write!(f, "{imag}i"),
            Complex { real, imag } if imag < &0.0 => write!(f, "{real}-{}i", -imag),
            Complex { real, imag } => write!(f, "{real}+{imag}i"),
        }
    }
}

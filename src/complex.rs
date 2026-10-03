use std::{
    fmt::Display,
    ops::{Add, Div, Mul, MulAssign, Neg, Sub},
};

#[derive(Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    pub const ZERO: Self = Self::new(0.0, 0.0);
    pub const ONE: Self = Self::new(1.0, 0.0);
    pub const I: Self = Self::new(0.0, 1.0);

    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }
    pub fn cis(phase: f64) -> Self {
        // e^it = cos(t) + i sin(t)
        let (sin, cos) = phase.sin_cos();
        Self::new(cos, sin)
    }

    pub const fn norm_sqr(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    pub fn norm(&self) -> f64 {
        self.norm_sqr().sqrt()
    }
    pub const fn conj(&self) -> Self {
        Self::new(self.re, -self.im)
    }
    pub fn powi(&self, n: i32) -> Self {
        // z = a+bi = r(cos(t) + i sin(t))
        // a = r cos(t), b = r sin(t)
        // b/a = tan(t) => t = atan(b/a)
        // a^2 + b^2 = (r cos(t))^2 + (r sin(t))^2
        //           = r^2 cos^2(t) + r^2 sin^2(t)
        //           = r^2 (cos^2(t) + sin^2(t))
        //           = r^2
        // r = sqrt(a^2 + b^2) = norm(z)
        // z = r cis(it)
        // z^n = r cis(i nt)
        let r = self.norm();
        let t = self.im.atan2(self.re);
        Self::cis(n as f64 * t) * r
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}
impl Add<f64> for Complex {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.re + rhs, self.im)
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}
impl Sub<Complex> for f64 {
    type Output = Complex;
    fn sub(self, rhs: Complex) -> Self::Output {
        Complex::new(self - rhs.re, rhs.im)
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        // (a+bi) * (c+di)
        // = ac + adi + bci - bd
        // = (ac-bd) + (ad+bc)i
        Self::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}
impl Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.re * rhs, self.im * rhs)
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs
    }
}
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        // (a+bi)/(c+di)
        // = (a+bi)(c-di)/(c+di)(c-di)
        // = (a+bi)(c-di)/|c+di|^2
        (self * rhs.conj()) / rhs.norm_sqr()
    }
}
impl Div<f64> for Complex {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.re / rhs, self.im / rhs)
    }
}
impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.re, -self.im)
    }
}
impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Self::new(value, 0.0)
    }
}
impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.re, self.im) {
            (r, i) if i < 0.0 => write!(f, "{r}{i}i"),
            (r, i) => write!(f, "{r}+{i}i"),
        }
    }
}

impl num_traits::identities::Zero for Complex {
    fn zero() -> Self {
        Self::ZERO
    }
    fn is_zero(&self) -> bool {
        self.re.abs() < 1e-12 && self.im.abs() < 1e-12
    }
}
impl num_traits::identities::One for Complex {
    fn one() -> Self {
        Self::ONE
    }
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        (self.re.abs() - 1.0) < 1e-12 && self.im.abs() < 1e-12
    }
}

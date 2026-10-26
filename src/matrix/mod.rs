use std::ops::{Add, Index, IndexMut, Mul, Sub};

use crate::Complex;

pub mod diagonal;
pub mod square;

pub trait Matrix<T = Self>
where
    T: Index<(usize, usize)>,
    T: IndexMut<(usize, usize)>,
    T: Clone,
    T: Add,
    T: Sub,
    T: PartialEq,
    T: Mul<f64>,
{
    fn zeros(size: usize) -> Self;
    fn size(&self) -> usize;
    #[must_use]
    fn map(&self, f: impl Fn(&Complex) -> Complex) -> Self;
    #[must_use]
    fn t(&self) -> Self;
    fn eye(size: usize) -> Self;
}

pub trait Dot<T = Self> {
    type Output;
    fn dot(&self, rhs: &T) -> Self::Output;
}

macro_rules! matrix {
    ($([$($el:expr),+ $(,)?]),+ $(,)?) => {
        $crate::matrix::square::SquareMatrix::from([$([$($el,)+],)+])
    };
    ($($el:expr),+ $(,)?) => {
        $crate::matrix::diagonal::DiagonalMatrix::from([$($crate::Complex::from($el),)+])
    }
}
pub(crate) use matrix;

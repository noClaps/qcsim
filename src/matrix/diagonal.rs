use std::{
    ops::{Add, Index, IndexMut, Mul, Sub},
    slice::Iter,
    vec::IntoIter,
};

use crate::{Complex, matrix::Matrix};

#[derive(Clone, PartialEq)]
pub struct DiagonalMatrix {
    pub(super) elems: Vec<Complex>,
    pub(super) size: usize,
}

impl DiagonalMatrix {
    pub fn append(&mut self, rhs: &Self) {
        let elems: Vec<_> = self.elems.iter().chain(rhs.elems.iter()).copied().collect();
        self.elems = elems;
    }
    pub fn iter(&self) -> Iter<'_, Complex> {
        self.elems.iter()
    }
}

impl Matrix for DiagonalMatrix {
    fn zeros(size: usize) -> Self {
        Self {
            elems: vec![Complex::ZERO; size],
            size,
        }
    }
    fn size(&self) -> usize {
        self.size
    }
    fn map(&self, f: impl Fn(&Complex) -> Complex) -> Self {
        Self {
            elems: self.elems.iter().map(f).collect(),
            size: self.size,
        }
    }
    fn t(&self) -> Self {
        self.clone()
    }
    fn eye(size: usize) -> Self {
        Self {
            elems: vec![Complex::ONE; size],
            size,
        }
    }
}

impl Index<(usize, usize)> for DiagonalMatrix {
    type Output = Complex;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 != index.1 {
            return &Complex::ZERO;
        }
        &self.elems[index.0]
    }
}
impl Index<usize> for DiagonalMatrix {
    type Output = Complex;
    fn index(&self, index: usize) -> &Self::Output {
        &self.elems[index]
    }
}

impl IndexMut<(usize, usize)> for DiagonalMatrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert_eq!(
            index.0, index.1,
            "Attempted to mutate non-diagonal value in diagonal matrix"
        );
        &mut self.elems[index.0]
    }
}

impl<const D: usize> From<[Complex; D]> for DiagonalMatrix {
    fn from(value: [Complex; D]) -> Self {
        Self {
            elems: value.to_vec(),
            size: D,
        }
    }
}

impl FromIterator<Complex> for DiagonalMatrix {
    fn from_iter<T: IntoIterator<Item = Complex>>(iter: T) -> Self {
        let elems: Vec<_> = iter.into_iter().collect();
        Self {
            size: elems.len(),
            elems,
        }
    }
}

impl IntoIterator for DiagonalMatrix {
    type Item = Complex;
    type IntoIter = IntoIter<Complex>;
    fn into_iter(self) -> Self::IntoIter {
        self.elems.into_iter()
    }
}
impl<'a> IntoIterator for &'a DiagonalMatrix {
    type Item = &'a Complex;
    type IntoIter = Iter<'a, Complex>;
    fn into_iter(self) -> Self::IntoIter {
        self.elems.iter()
    }
}

impl Add for DiagonalMatrix {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.size, rhs.size,
            "Attempted to add matrices of different sizes"
        );
        self.iter().zip(rhs.iter()).map(|(&l, &r)| l + r).collect()
    }
}

impl Mul<f64> for DiagonalMatrix {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|&c| c * rhs)
    }
}

impl Sub for DiagonalMatrix {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(
            self.size, rhs.size,
            "Attempted to subtract matrices of different sizes"
        );
        self.iter().zip(rhs.iter()).map(|(&l, &r)| l - r).collect()
    }
}

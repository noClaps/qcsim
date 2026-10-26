use std::{
    ops::{Index, IndexMut},
    slice::{Iter, IterMut},
};

use crate::Complex;

#[derive(Clone)]
pub struct StateVector(Vec<Complex>);

impl StateVector {
    pub fn zeros(size: usize) -> Self {
        Self(vec![Complex::ZERO; size])
    }
    pub fn iter(&self) -> Iter<'_, Complex> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> IterMut<'_, Complex> {
        self.0.iter_mut()
    }
    pub const fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for StateVector {
    type Output = Complex;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}
impl IndexMut<usize> for StateVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl FromIterator<Complex> for StateVector {
    fn from_iter<T: IntoIterator<Item = Complex>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

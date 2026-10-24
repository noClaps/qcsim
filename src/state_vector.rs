use std::{
    ops::{Index, IndexMut},
    slice::Iter,
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

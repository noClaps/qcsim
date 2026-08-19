use std::{
    ops::{Index, IndexMut},
    slice::Iter,
};

use crate::Complex;

/// A type representing a state vector
#[derive(Clone)]
pub struct StateVector(Vec<Complex>);

impl StateVector {
    /// Initialise the [`StateVector`] with `size` zeros
    pub fn zeros(size: usize) -> Self {
        Self(vec![Complex::ZERO; size])
    }
    /// Convert the [`StateVector`] into an iterator
    pub fn iter(&self) -> Iter<'_, Complex> {
        self.0.iter()
    }
    /// Get the length of the [`StateVector`]
    pub const fn len(&self) -> usize {
        self.0.len()
    }
    /// Check if the [`StateVector`] is empty
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Index<usize> for StateVector {
    type Output = Complex;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for StateVector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl FromIterator<Complex> for StateVector {
    fn from_iter<T: IntoIterator<Item = Complex>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

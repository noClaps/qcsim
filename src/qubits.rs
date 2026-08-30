use std::ops::Range;

pub enum Qubits {
    Single(usize),
    Array(Vec<usize>),
}

impl From<usize> for Qubits {
    fn from(value: usize) -> Self {
        Self::Single(value)
    }
}
impl From<Range<usize>> for Qubits {
    fn from(value: Range<usize>) -> Self {
        Self::Array(value.collect())
    }
}
impl<const N: usize> From<&[usize; N]> for Qubits {
    fn from(value: &[usize; N]) -> Self {
        Self::Array(value.to_vec())
    }
}
impl From<&[usize]> for Qubits {
    fn from(value: &[usize]) -> Self {
        Self::Array(value.to_vec())
    }
}
impl From<Vec<usize>> for Qubits {
    fn from(value: Vec<usize>) -> Self {
        Self::Array(value)
    }
}

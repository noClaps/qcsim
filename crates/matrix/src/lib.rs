use num_complex::Complex64;

#[cfg(test)]
mod tests;
mod trait_impl;

#[macro_export]
macro_rules! matrix {
    ($($($el:expr),*$(,)?);*$(;)?) => {
        qcsim_matrix::Matrix(vec![$(vec![$($el,)*],)*])
    };
}

/// An NxN square matrix
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix(pub Vec<Vec<Complex64>>);

impl Matrix {
    pub const fn size(&self) -> usize {
        self.0.len()
    }

    pub fn zero(size: usize) -> Self {
        Matrix(vec![vec![Complex64::ZERO; size]; size])
    }

    pub fn tensor_prod(&self, mat: &Matrix) -> Matrix {
        let mut output = Matrix::zero(self.size() * mat.size());
        for (i, row1) in self.0.iter().enumerate() {
            for (j, &val1) in row1.iter().enumerate() {
                for (k, row2) in mat.0.iter().enumerate() {
                    for (l, &val2) in row2.iter().enumerate() {
                        output[i * mat.size() + k][j * row2.len() + l] = val1 * val2;
                    }
                }
            }
        }
        output
    }

    pub fn tensor_prod_list(matrices: &[Matrix]) -> Matrix {
        matrices
            .iter()
            .fold(Matrix::diag(matrices[0].size()), |acc, m| {
                acc.tensor_prod(m)
            })
            .clone()
    }

    pub fn diag(size: usize) -> Self {
        let mut mat = Self::zero(size);
        for (i, row) in mat.0.iter_mut().enumerate() {
            for (j, el) in row.iter_mut().enumerate() {
                if i == j {
                    *el = Complex64::ONE;
                }
            }
        }
        mat
    }

    pub fn adjoint(&self) -> Self {
        let mut out = Self::zero(self.size());
        for (i, row) in self.0.iter().enumerate() {
            for (j, el) in row.iter().enumerate() {
                out[j][i] = el.conj();
            }
        }
        out
    }

    pub fn pow(&self, pow: i128) -> Self {
        match pow {
            0 => Self::diag(self.size()),
            ..0 => {
                let mut out = self.adjoint();
                for _ in 0..(-pow) - 1 {
                    out = out * self.adjoint();
                }
                out
            }
            1.. => {
                let mut out = self.clone();
                for _ in 0..pow - 1 {
                    out = out * self.clone()
                }
                out
            }
        }
    }
}

use num_complex::Complex64;
use qcsim_gates::{Ctrl, Gate};
use qcsim_matrix::{Matrix, matrix};

#[derive(Clone)]
pub struct Computer(Vec<Complex64>, usize);

impl Computer {
    /// Creates a new quantum computer. Initialises `size` qubits to state |00...0>.
    #[must_use]
    pub fn new(size: usize) -> Self {
        let mut register = vec![Complex64::ZERO; 1 << size];
        register[0] = Complex64::ONE;
        Self(register, size)
    }

    /// Returns the measured states as a string.
    #[must_use]
    pub fn measure(self) -> String {
        let rand = fastrand::f64();
        let mut total = 0.0;
        for (i, state) in self.0.iter().enumerate() {
            total += state.norm_sqr();
            if total >= rand {
                return format!("{:0>1$}", format!("{i:b}"), self.1);
            }
        }
        format!("{:b}", self.0.len() - 1)
    }

    pub fn apply_gate(&mut self, gate: &Gate, index: usize) {
        let matrix = match gate.ctrl {
            Some(Ctrl::Ctrl(n)) => {
                let mut mat1 = vec![Gate::identity().matrix; self.1];
                mat1[n] = matrix! {
                    Complex64::ONE, Complex64::ZERO;
                    Complex64::ZERO, Complex64::ZERO
                };
                let mut mat2 = vec![Gate::identity().matrix; self.1];
                mat2[n] = matrix! {
                    Complex64::ZERO, Complex64::ZERO;
                    Complex64::ZERO, Complex64::ONE
                };
                mat2[index] = gate.matrix.clone();
                Matrix::tensor_prod_list(&mat1) + Matrix::tensor_prod_list(&mat2)
            }
            Some(Ctrl::NegCtrl(n)) => {
                let mut mat1 = vec![Gate::identity().matrix; self.1];
                mat1[n] = matrix! {
                    Complex64::ONE, Complex64::ZERO;
                    Complex64::ZERO, Complex64::ZERO
                };
                mat1[index] = gate.matrix.clone();
                let mut mat2 = vec![Gate::identity().matrix; self.1];
                mat2[n] = matrix! {
                    Complex64::ONE, Complex64::ZERO;
                    Complex64::ZERO, Complex64::ZERO
                };
                Matrix::tensor_prod_list(&mat1) + Matrix::tensor_prod_list(&mat2)
            }
            None => {
                let mut matrices = vec![Gate::identity().matrix; self.1];
                matrices[index] = gate.matrix.clone();
                Matrix::tensor_prod_list(&matrices)
            }
        };

        self.0 = matrix * &self.0;
    }
}
// impl Computer {
//     /// Applies the Pauli X operator on the qubit. The index starts at 0 for the first index.
//     pub fn pauli_x(&mut self, index: usize) {
//         let mut matrices = vec![gate!(I); self.1];
//         matrices[index] = gate!(X);
//         let matrix = Matrix::tensor_prod_list(&matrices);
//         self.0 = matrix * &self.0
//     }

//     /// Applies the Pauli Y operator on the qubit. The index starts at 0 for the first index.
//     pub fn pauli_y(&mut self, index: usize) {
//         let mut matrices = vec![gate!(I); self.1];
//         matrices[index] = gate!(Y);
//         let matrix = Matrix::tensor_prod_list(&matrices);
//         self.0 = matrix * &self.0
//     }

//     /// Applies the Pauli Z operator on the qubit. The index starts at 0 for the first index.
//     pub fn pauli_z(&mut self, index: usize) {
//         let mut matrices = vec![gate!(I); self.1];
//         matrices[index] = gate!(Z);
//         let matrix = Matrix::tensor_prod_list(&matrices);
//         self.0 = matrix * &self.0
//     }

//     /// Applies the Hadamard operator on the qubit. The index starts at 0 for the first index.
//     pub fn hadamard(&mut self, index: usize) {
//         let mut matrices = vec![gate!(I); self.1];
//         matrices[index] = gate!(H);
//         let matrix = Matrix::tensor_prod_list(&matrices);
//         self.0 = matrix * &self.0
//     }

//     /// Applies the Phase operator on the qubit. The index starts at 0 for the first index.
//     pub fn phase(&mut self, index: usize) {
//         let mut matrices = vec![gate!(I); self.1];
//         matrices[index] = gate!(PHASE, 1.0);
//         let matrix = Matrix::tensor_prod_list(&matrices);
//         self.0 = matrix * &self.0
//     }

//     /// Applies the π/8 operator on the qubit. The index starts at 0 for the first index.
//     pub fn pi_by_8(&mut self, index: usize) {
//         let mut matrices = vec![gate!(I); self.1];
//         matrices[index] = gate!(PI / 8);
//         let matrix = Matrix::tensor_prod_list(&matrices);
//         self.0 = matrix * &self.0
//     }

//     /// Applies the Controlled Not operator on the qubits. The index starts at 0 for the first index.
//     pub fn controlled_not(&mut self, index1: usize, index2: usize) {
//         let mut matrices0 = vec![gate!(I); self.1];
//         matrices0[index1] = gate!(|0><0|);

//         let mut matrices1 = vec![gate!(I); self.1];
//         matrices1[index1] = gate!(|1><1|);
//         matrices1[index2] = gate!(X);

//         let matrix0 = Matrix::tensor_prod_list(&matrices0);
//         let matrix1 = Matrix::tensor_prod_list(&matrices1);
//         let matrix = matrix0 + matrix1;

//         self.0 = matrix * &self.0
//     }

//     /// Applies the Controlled Z operator on the qubits. The index starts at 0 for the first index.
//     pub fn controlled_z(&mut self, index1: usize, index2: usize) {
//         let mut matrices0 = vec![gate!(I); self.1];
//         matrices0[index1] = gate!(|0><0|);

//         let mut matrices1 = vec![gate!(I); self.1];
//         matrices1[index1] = gate!(|1><1|);
//         matrices1[index2] = gate!(Z);

//         let matrix0 = Matrix::tensor_prod_list(&matrices0);
//         let matrix1 = Matrix::tensor_prod_list(&matrices1);
//         let matrix = matrix0 + matrix1;

//         self.0 = matrix * &self.0
//     }

//     /// Applies the Swap operator on the qubits. The index starts at 0 for the first index.
//     pub fn swap(&mut self, index1: usize, index2: usize) {
//         let mut matrices0 = vec![gate!(I); self.1];
//         matrices0[index1] = gate!(|0><1|);
//         matrices0[index2] = gate!(|1><0|);

//         let mut matrices1 = vec![gate!(I); self.1];
//         matrices1[index1] = gate!(|1><0|);
//         matrices1[index2] = gate!(|0><1|);

//         let matrix0 = Matrix::tensor_prod_list(&matrices0);
//         let matrix1 = Matrix::tensor_prod_list(&matrices1);
//         let matrix = matrix0 + matrix1;

//         self.0 = matrix * &self.0
//     }

//     /// Applies the Toffoli operator on the qubits. The index starts at 0 for the first index.
//     pub fn toffoli(&mut self, index1: usize, index2: usize, index3: usize) {
//         let mut matrices0 = vec![gate!(I); self.1];
//         matrices0[index1] = gate!(|0><0|);
//         matrices0[index2] = gate!(|0><0|);

//         let mut matrices1 = vec![gate!(I); self.1];
//         matrices1[index1] = gate!(|0><0|);
//         matrices1[index2] = gate!(|1><1|);

//         let mut matrices2 = vec![gate!(I); self.1];
//         matrices2[index1] = gate!(|1><1|);
//         matrices2[index2] = gate!(|0><0|);

//         let mut matrices3 = vec![gate!(I); self.1];
//         matrices3[index1] = gate!(|1><1|);
//         matrices3[index2] = gate!(|1><1|);
//         matrices3[index3] = gate!(X);

//         let matrix0 = Matrix::tensor_prod_list(&matrices0);
//         let matrix1 = Matrix::tensor_prod_list(&matrices1);
//         let matrix2 = Matrix::tensor_prod_list(&matrices2);
//         let matrix3 = Matrix::tensor_prod_list(&matrices3);
//         let matrix = matrix0 + matrix1 + matrix2 + matrix3;

//         self.0 = matrix * &self.0
//     }
// }

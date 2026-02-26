use std::{collections::HashMap, f64::consts::FRAC_PI_4, ops::AddAssign};

use crate::{complex::Complex, qubit::Qubit};

fn matmul<const N: usize>(matrix: [[Complex; N]; N], vector: [Complex; N]) -> [Complex; N] {
    let mut out_vec = [Complex::ZERO; N];
    for r in 0..matrix.len() {
        let mut row_sum = Complex::ZERO;
        for c in 0..matrix[r].len() {
            row_sum += matrix[r][c] * vector[c];
        }
        out_vec[r] = row_sum;
    }
    out_vec
}

// TODO: Ensure this with types when `generic_const_exprs` has been implemented.
// https://doc.rust-lang.org/beta/unstable-book/language-features/generic-const-exprs.html
/// Ensure that M == 1 << N.
trait Apply<const N: usize, const M: usize> {
    fn apply(&mut self, matrix: [[Complex; M]; M], indices: [usize; N]);
}

pub struct Computer(Vec<Qubit>);

impl Computer {
    /// Creates a new quantum computer.
    pub fn new<const N: usize>(qubits: [Qubit; N]) -> Self {
        Self(qubits.to_vec())
    }

    /// Returns the measured states as a string. The returned string will have the measured value of
    /// the first qubit as the first character, the second qubit as the second character, and so on
    /// for as many qubits as were input into the computer. The count input determines how many
    /// times to measure. If the count is 1, then only the measured state is returned, but if it is
    /// greater than 1, then a table of the states and the number of times they appeared is
    /// returned.
    pub fn measure(self, count: usize) -> String {
        let mut states: HashMap<String, usize> = HashMap::new();
        for _ in 0..count {
            let mut measured_state = String::new();
            for q in &self.0 {
                measured_state += &q.measure().to_string();
            }
            if count == 1 {
                return measured_state;
            }

            match states.get_mut(&measured_state) {
                Some(v) => v.add_assign(1),
                None => {
                    states.insert(measured_state, 1);
                }
            };
        }

        let mut output: Vec<_> = states.iter().map(|(k, v)| format!("{k}: {v}")).collect();
        output.sort();
        output.join("\n")
    }

    /// Applies the Pauli X operator on the qubit. The index starts at 0 for the first index.
    pub fn pauli_x(&mut self, index: usize) {
        self.apply(
            [[Complex::ZERO, Complex::ONE], [Complex::ONE, Complex::ZERO]],
            [index],
        );
    }

    /// Applies the Pauli Y operator on the qubit. The index starts at 0 for the first index.
    pub fn pauli_y(&mut self, index: usize) {
        self.apply(
            [[Complex::ZERO, -Complex::I], [Complex::I, Complex::ZERO]],
            [index],
        );
    }

    /// Applies the Pauli Z operator on the qubit. The index starts at 0 for the first index.
    pub fn pauli_z(&mut self, index: usize) {
        self.apply(
            [
                [Complex::ONE, Complex::ZERO],
                [Complex::ZERO, -Complex::ONE],
            ],
            [index],
        );
    }

    /// Applies the Hadamard operator on the qubit. The index starts at 0 for the first index.
    pub fn hadamard(&mut self, index: usize) {
        self.apply(
            [
                [
                    Complex::ONE / Complex::SQRT_2,
                    Complex::ONE / Complex::SQRT_2,
                ],
                [
                    Complex::ONE / Complex::SQRT_2,
                    -Complex::ONE / Complex::SQRT_2,
                ],
            ],
            [index],
        );
    }

    /// Applies the Phase operator on the qubit. The index starts at 0 for the first index.
    pub fn phase(&mut self, index: usize) {
        self.apply(
            [[Complex::ONE, Complex::ZERO], [Complex::ZERO, Complex::I]],
            [index],
        );
    }

    /// Applies the π/8 operator on the qubit. The index starts at 0 for the first index.
    pub fn pi_by_8(&mut self, index: usize) {
        self.apply(
            [
                [Complex::ONE, Complex::ZERO],
                [Complex::ZERO, Complex::rect(1.0, FRAC_PI_4.exp())],
            ],
            [index],
        );
    }

    /// Applies the Controlled Not operator on the qubits. The index starts at 0 for the first index.
    pub fn controlled_not(&mut self, index1: usize, index2: usize) {
        self.apply(
            [
                [Complex::ONE, Complex::ZERO, Complex::ZERO, Complex::ZERO],
                [Complex::ZERO, Complex::ONE, Complex::ZERO, Complex::ZERO],
                [Complex::ZERO, Complex::ZERO, Complex::ZERO, Complex::ONE],
                [Complex::ZERO, Complex::ZERO, Complex::ONE, Complex::ZERO],
            ],
            [index1, index2],
        );
    }

    /// Applies the Controlled Z operator on the qubits. The index starts at 0 for the first index.
    pub fn controlled_z(&mut self, index1: usize, index2: usize) {
        self.apply(
            [
                [Complex::ONE, Complex::ZERO, Complex::ZERO, Complex::ZERO],
                [Complex::ZERO, Complex::ONE, Complex::ZERO, Complex::ZERO],
                [Complex::ZERO, Complex::ZERO, Complex::ONE, Complex::ZERO],
                [Complex::ZERO, Complex::ZERO, Complex::ZERO, -Complex::ONE],
            ],
            [index1, index2],
        );
    }

    /// Applies the Swap operator on the qubits. The index starts at 0 for the first index.
    pub fn swap(&mut self, index1: usize, index2: usize) {
        self.apply(
            [
                [Complex::ONE, Complex::ZERO, Complex::ZERO, Complex::ZERO],
                [Complex::ZERO, Complex::ZERO, Complex::ONE, Complex::ZERO],
                [Complex::ZERO, Complex::ONE, Complex::ZERO, Complex::ZERO],
                [Complex::ZERO, Complex::ZERO, Complex::ZERO, -Complex::ONE],
            ],
            [index1, index2],
        );
    }

    /// Applies the Toffoli operator on the qubits. The index starts at 0 for the first index.
    pub fn toffoli(&mut self, index1: usize, index2: usize, index3: usize) {
        self.apply(
            [
                [
                    Complex::ONE,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                ],
                [
                    Complex::ZERO,
                    Complex::ONE,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                ],
                [
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ONE,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                ],
                [
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ONE,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                ],
                [
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ONE,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                ],
                [
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ONE,
                    Complex::ZERO,
                    Complex::ZERO,
                ],
                [
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ONE,
                ],
                [
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ZERO,
                    Complex::ONE,
                    Complex::ZERO,
                ],
            ],
            [index1, index2, index3],
        );
    }
}

impl Apply<1, 2> for Computer {
    fn apply(&mut self, matrix: [[Complex; 2]; 2], indices: [usize; 1]) {
        let q = self.0[indices[0]];

        let input_amps = [q.0, q.1];
        let output_amps = matmul(matrix, input_amps);

        self.0[indices[0]] = Qubit(output_amps[0], output_amps[1]);
    }
}

impl Apply<2, 4> for Computer {
    fn apply(&mut self, matrix: [[Complex; 4]; 4], indices: [usize; 2]) {
        let q0 = self.0[indices[0]];
        let q1 = self.0[indices[1]];

        let input_amps = [
            q0.0 * q1.0, // 00
            q0.0 * q1.1, // 01
            q0.1 * q1.0, // 10
            q0.1 * q1.1, // 11
        ];
        let output_amps = matmul(matrix, input_amps);

        self.0[indices[0]] = Qubit(
            output_amps[0] + output_amps[1],
            output_amps[2] + output_amps[3],
        );
        self.0[indices[1]] = Qubit(
            output_amps[0] + output_amps[2],
            output_amps[1] + output_amps[3],
        );
    }
}

impl Apply<3, 8> for Computer {
    fn apply(&mut self, matrix: [[Complex; 8]; 8], indices: [usize; 3]) {
        let q0 = self.0[indices[0]];
        let q1 = self.0[indices[1]];
        let q2 = self.0[indices[2]];

        let input_amps = [
            q0.0 * q1.0 * q2.0, // 000
            q0.0 * q1.0 * q2.1, // 001
            q0.0 * q1.1 * q2.0, // 010
            q0.0 * q1.1 * q2.1, // 011
            q0.1 * q1.0 * q2.0, // 100
            q0.1 * q1.0 * q2.1, // 101
            q0.1 * q1.1 * q2.0, // 110
            q0.1 * q1.1 * q2.1, // 111
        ];
        let output_amps = matmul(matrix, input_amps);

        self.0[indices[0]] = Qubit(
            output_amps[0] + output_amps[1] + output_amps[2] + output_amps[3],
            output_amps[4] + output_amps[5] + output_amps[6] + output_amps[7],
        );
        self.0[indices[1]] = Qubit(
            output_amps[0] + output_amps[1] + output_amps[4] + output_amps[5],
            output_amps[2] + output_amps[3] + output_amps[6] + output_amps[7],
        );
        self.0[indices[2]] = Qubit(
            output_amps[0] + output_amps[2] + output_amps[4] + output_amps[6],
            output_amps[1] + output_amps[3] + output_amps[5] + output_amps[7],
        );
    }
}

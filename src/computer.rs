use num_complex::Complex64;
use std::f64::consts::{PI, SQRT_2};

use crate::qubit::Qubit;

const ZERO: Complex64 = Complex64::ZERO;
const ONE: Complex64 = Complex64::ONE;
const I: Complex64 = Complex64::I;

#[derive(Clone)]
pub struct Computer {
    state: Vec<Qubit>,
}

impl Computer {
    /// Creates a new quantum computer. The input is the vector of qubits
    pub fn new(qubits: Vec<Qubit>) -> Self {
        Self { state: qubits }
    }

    /// Returns the measured states as a string. The returned string will have
    /// the measured value of the first qubit as the first character, the
    /// second qubit as the second character, and so on for as many qubits as
    /// were input into the computer.
    pub fn measure(&mut self) -> String {
        let mut output = String::new();
        for q in self.state.clone() {
            let random_val = rand::random_range(0.0..=1.0);
            let zero_prob = q.probability_zero();
            output += format!("{}", if random_val <= zero_prob { 0 } else { 1 }).as_str()
        }

        output
    }

    fn apply1(&mut self, index: usize, matrix: [[Complex64; 2]; 2]) -> &mut Self {
        if self.state.len() < 1 {
            eprintln!("[ERROR] Not enough qubits in computer!");
            return self;
        }
        if index >= self.state.len() {
            eprintln!("[ERROR] Index greater than number of qubits");
            return self;
        }

        let zero = self.state[index].zero * matrix[0][0] + self.state[index].one * matrix[0][1];
        let one = self.state[index].zero * matrix[1][0] + self.state[index].one * matrix[1][1];

        self.state[index] = Qubit::new(zero, one);

        self
    }

    fn apply2(&mut self, index1: usize, index2: usize, matrix: [[Complex64; 4]; 4]) -> &mut Self {
        if self.state.len() < 2 {
            eprintln!("[ERROR] Not enough qubits in computer!");
            return self;
        }
        if index1 >= self.state.len() || index2 >= self.state.len() {
            eprintln!("[ERROR] Index greater than number of qubits");
            return self;
        }

        let q1 = self.state[index1];
        let q2 = self.state[index2];
        let input_amps = [
            q1.zero * q2.zero, // 00
            q1.zero * q2.one,  // 01
            q1.one * q2.zero,  // 10
            q1.one * q2.one,   // 11
        ];

        let mut output_amps = [Complex64::ZERO; 4];
        for (i, amp) in input_amps.into_iter().enumerate() {
            output_amps[0] += matrix[0][i] * amp;
            output_amps[1] += matrix[1][i] * amp;
            output_amps[2] += matrix[2][i] * amp;
            output_amps[3] += matrix[3][i] * amp;
        }

        self.state[index1] = Qubit::new(
            output_amps[0] + output_amps[1],
            output_amps[2] + output_amps[3],
        );
        self.state[index2] = Qubit::new(
            output_amps[0] + output_amps[2],
            output_amps[1] + output_amps[3],
        );

        self
    }

    fn apply3(
        &mut self,
        index1: usize,
        index2: usize,
        index3: usize,
        matrix: [[Complex64; 8]; 8],
    ) -> &mut Self {
        if self.state.len() < 3 {
            eprintln!("[ERROR] Not enough qubits in computer!");
            return self;
        }
        if index1 >= self.state.len() || index2 >= self.state.len() || index3 >= self.state.len() {
            eprintln!("[ERROR] Index greater than number of qubits");
            return self;
        }

        let q1 = self.state[index1];
        let q2 = self.state[index2];
        let q3 = self.state[index3];

        let input_amps = [
            q1.zero * q2.zero * q3.zero, // 000
            q1.zero * q2.zero * q3.one,  // 001
            q1.zero * q2.one * q3.zero,  // 010
            q1.zero * q2.one * q3.one,   // 011
            q1.one * q2.zero * q3.zero,  // 100
            q1.one * q2.zero * q3.one,   // 101
            q1.one * q2.one * q3.zero,   // 110
            q1.one * q2.one * q3.one,    // 111
        ];

        let mut output_amps = [Complex64::ZERO; 8];

        for (i, amp) in input_amps.into_iter().enumerate() {
            output_amps[0] += matrix[0][i] * amp;
            output_amps[1] += matrix[1][i] * amp;
            output_amps[2] += matrix[2][i] * amp;
            output_amps[3] += matrix[3][i] * amp;
            output_amps[4] += matrix[4][i] * amp;
            output_amps[5] += matrix[5][i] * amp;
            output_amps[6] += matrix[6][i] * amp;
            output_amps[7] += matrix[7][i] * amp;
        }

        self.state[index1] = Qubit::new(
            output_amps[0] + output_amps[1] + output_amps[2] + output_amps[3],
            output_amps[4] + output_amps[5] + output_amps[6] + output_amps[7],
        );
        self.state[index2] = Qubit::new(
            output_amps[0] + output_amps[1] + output_amps[4] + output_amps[5],
            output_amps[2] + output_amps[3] + output_amps[6] + output_amps[7],
        );
        self.state[index3] = Qubit::new(
            output_amps[0] + output_amps[2] + output_amps[4] + output_amps[6],
            output_amps[1] + output_amps[3] + output_amps[5] + output_amps[7],
        );

        self
    }

    pub fn pauli_x(&mut self, index: usize) -> &mut Self {
        let matrix = [[ZERO, ONE], [ONE, ZERO]];
        self.apply1(index, matrix)
    }

    pub fn pauli_y(&mut self, index: usize) -> &mut Self {
        let matrix = [[ZERO, -I], [I, ZERO]];
        self.apply1(index, matrix)
    }

    pub fn pauli_z(&mut self, index: usize) -> &mut Self {
        let matrix = [[ONE, ZERO], [ZERO, -ONE]];
        self.apply1(index, matrix)
    }

    pub fn hadamard(&mut self, index: usize) -> &mut Self {
        let matrix = [[ONE / SQRT_2, ONE / SQRT_2], [ONE / SQRT_2, -ONE / SQRT_2]];
        self.apply1(index, matrix)
    }

    pub fn phase(&mut self, index: usize) -> &mut Self {
        let matrix = [[ONE, ZERO], [ZERO, I]];
        self.apply1(index, matrix)
    }

    pub fn pi_by_8(&mut self, index: usize) -> &mut Self {
        let matrix = [[ONE, ZERO], [ZERO, Complex64::from_polar(1., PI / 4.)]];
        self.apply1(index, matrix)
    }

    pub fn controlled_not(&mut self, index1: usize, index2: usize) -> &mut Self {
        let matrix = [
            [ONE, ZERO, ZERO, ZERO],
            [ZERO, ONE, ZERO, ZERO],
            [ZERO, ZERO, ZERO, ONE],
            [ZERO, ZERO, ONE, ZERO],
        ];
        self.apply2(index1, index2, matrix)
    }

    pub fn controlled_z(&mut self, index1: usize, index2: usize) -> &mut Self {
        let matrix = [
            [ONE, ZERO, ZERO, ZERO],
            [ZERO, ONE, ZERO, ZERO],
            [ZERO, ZERO, ONE, ZERO],
            [ZERO, ZERO, ZERO, -ONE],
        ];
        self.apply2(index1, index2, matrix)
    }

    pub fn swap(&mut self, index1: usize, index2: usize) -> &mut Self {
        let matrix = [
            [ONE, ZERO, ZERO, ZERO],
            [ZERO, ZERO, ONE, ZERO],
            [ZERO, ONE, ZERO, ZERO],
            [ZERO, ZERO, ZERO, ONE],
        ];
        self.apply2(index1, index2, matrix)
    }

    pub fn toffoli(&mut self, index1: usize, index2: usize, index3: usize) -> &mut Self {
        let matrix = [
            [ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO],
            [ZERO, ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO],
            [ZERO, ZERO, ONE, ZERO, ZERO, ZERO, ZERO, ZERO],
            [ZERO, ZERO, ZERO, ONE, ZERO, ZERO, ZERO, ZERO],
            [ZERO, ZERO, ZERO, ZERO, ONE, ZERO, ZERO, ZERO],
            [ZERO, ZERO, ZERO, ZERO, ZERO, ONE, ZERO, ZERO],
            [ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ONE],
            [ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ONE, ZERO],
        ];
        self.apply3(index1, index2, index3, matrix)
    }
}

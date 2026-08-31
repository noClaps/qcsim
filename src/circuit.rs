use crate::{Array1, Complex, Gate, StateVector, qubits::Qubits};

/// A quantum circuit
#[derive(Clone)]
pub struct QuantumCircuit {
    pub qubits: StateVector,
    pub num_qubits: usize,
}

impl QuantumCircuit {
    /// Initialise a quantum circuit with state |00...0>
    pub fn new(num_qubits: usize) -> Self {
        let mut qubits = Array1::zeros(1 << num_qubits);
        qubits[0] = Complex::new(1.0, 0.0);
        Self { qubits, num_qubits }
    }

    /// Measure the state of the qubits. Consumes the circuit instance.
    pub fn measure(self) -> usize {
        let rand = fastrand::f64_inclusive();
        let mut total = 0.0;
        for (i, state) in self.qubits.iter().enumerate() {
            total += state.norm_sqr();
            if total >= rand {
                return i;
            }
        }
        self.qubits.len() - 1
    }

    /// Reset the qubits to the |00...0> state
    pub fn reset(&mut self) {
        self.qubits = Array1::zeros(1 << self.num_qubits);
        self.qubits[0] = Complex::new(1.0, 0.0);
    }

    /// Apply unitary `gate` to `qubits`
    pub fn unitary(&mut self, gate: &Gate, qubits: impl Into<Qubits>) {
        let qubits = match qubits.into() {
            Qubits::Single(q) => {
                self.apply_single_qubit(gate, q);
                return;
            }
            Qubits::Array(qs) if qs.len() == 1 => {
                self.apply_single_qubit(gate, qs[0]);
                return;
            }
            Qubits::Array(qs) => qs,
        };
        let state_len = self.qubits.len();
        let n = state_len.trailing_zeros() as usize;
        let k = qubits.len();
        let local_dim = 1 << k;

        assert_eq!(
            qubits.len(),
            match gate {
                Gate::Normal(matrix) => matrix.nrows().ilog2(),
                Gate::Diagonal(matrix) => matrix.len().ilog2(),
            } as usize,
            "Cannot apply {}-qubit gate to {} qubits",
            match gate {
                Gate::Normal(matrix) => matrix.nrows().ilog2(),
                Gate::Diagonal(matrix) => matrix.len().ilog2(),
            },
            qubits.len()
        );

        let bit_position = |q: usize| -> usize { n - 1 - q };
        let target_mask = qubits
            .iter()
            .fold(0, |acc, &q| acc | (1 << bit_position(q)));

        for base in 0..state_len {
            // isolate all of the qubits that will remain unchanged.
            // e.g. CNOT(0,2) -> qubit 1 is unchanged
            // this filter will only allow through 0q0 where q can be 0 or 1
            if base & target_mask != 0 {
                continue;
            }
            match gate {
                Gate::Normal(matrix) => {
                    // all of the possible combinations for the qubits being targeted
                    // e.g. 0q0, 0q1, 1q0, 1q1
                    let indices = (0..local_dim).map(|local_index| {
                        let mut global_index = base;
                        for (j, &qubit) in qubits.iter().enumerate() {
                            let local_bit = (local_index >> (k - 1 - j)) & 1;
                            let global_bit = bit_position(qubit);
                            global_index |= local_bit << global_bit;
                        }
                        global_index
                    });

                    // create input from qubits at `indices`
                    // indices = [000, 001, 100, 101] -> input = [A_000, A_001, A_100, A_101]
                    // where A_000 is the amplitude for qubit |000>
                    let input = Array1::from_iter(indices.clone().map(|i| self.qubits[i]));
                    // apply gate
                    let output = matrix.dot(&input);
                    // write output back into qubits
                    // indices = [000, 001, 100, 101]
                    // qubits[indices[n]] = output[n]
                    for (local_index, global_index) in indices.enumerate() {
                        self.qubits[global_index] = output[local_index];
                    }
                }
                Gate::Diagonal(matrix) => {
                    for local_index in 0..local_dim {
                        let mut global_index = base;

                        for (j, &qubit) in qubits.iter().enumerate() {
                            let local_bit = (local_index >> (k - 1 - j)) & 1;
                            let global_bit = bit_position(qubit);

                            global_index |= local_bit << global_bit;
                        }

                        self.qubits[global_index] *= matrix[local_index];
                    }
                }
            }
        }
    }

    fn apply_single_qubit(&mut self, gate: &Gate, qubit: usize) {
        let mask = 1 << (self.num_qubits - 1 - qubit);
        let block_size = mask << 1;

        match gate {
            Gate::Normal(matrix) => {
                assert_eq!(
                    matrix.nrows(),
                    2,
                    "Cannot apply {}-qubit gate to 1 qubit",
                    matrix.nrows().ilog2()
                );
                let g00 = matrix[(0, 0)];
                let g01 = matrix[(0, 1)];
                let g10 = matrix[(1, 0)];
                let g11 = matrix[(1, 1)];

                for block_start in (0..self.qubits.len()).step_by(block_size) {
                    for offset in 0..mask {
                        let i0 = block_start + offset;
                        let i1 = i0 + mask;

                        let a0 = self.qubits[i0];
                        let a1 = self.qubits[i1];

                        self.qubits[i0] = g00 * a0 + g01 * a1;
                        self.qubits[i1] = g10 * a0 + g11 * a1;
                    }
                }
            }
            Gate::Diagonal(matrix) => {
                assert_eq!(
                    matrix.len(),
                    2,
                    "Cannot apply {}-qubit gate to 1 qubit",
                    matrix.len().ilog2()
                );
                let d0 = matrix[0];
                let d1 = matrix[1];

                for block_start in (0..self.qubits.len()).step_by(block_size) {
                    for offset in 0..mask {
                        let i0 = block_start + offset;
                        let i1 = i0 + mask;

                        self.qubits[i0] *= d0;
                        self.qubits[i1] *= d1;
                    }
                }
            }
        }
    }
}

macro_rules! impl_gate {
    ($name:ident, $($arg:ident: $ty:ty,)* $doc:literal) => {
        #[doc = $doc]
        pub fn $name(&mut self, $($arg: $ty,)* idx: impl Into<$crate::qubits::Qubits>) {
            match idx.into() {
                $crate::qubits::Qubits::Single(idx) => self.unitary(&$crate::Gate::$name($($arg,)*), idx),
                $crate::qubits::Qubits::Array(idxs) => {
                    for idx in idxs {
                        self.unitary(&$crate::Gate::$name($($arg,)*), idx);
                    }
                }
            }

        }
    };
    ($name:ident, $($arg:ident: $ty:ty,)* [$($qubit:ident),* $(,)?], $doc:literal) => {
        #[doc = $doc]
        pub fn $name(&mut self, $($arg: $ty,)* $($qubit: usize,)*) {
            self.unitary(&$crate::Gate::$name($($arg,)*), &[$($qubit,)*])
        }
    };
}

/// Single qubit gates
impl QuantumCircuit {
    impl_gate!(h, "Single-qubit Hadamard gate");
    impl_gate!(i, "Identity gate.");
    impl_gate!(id, "Identity gate.");
    impl_gate!(p, theta: f64, "Single-qubit rotation around the Z axis.");
    impl_gate!(r, theta: f64, phi: f64, "Rotation θ around the cos(ϕ)x+sin(ϕ)y axis.");
    impl_gate!(rx, theta: f64, "Single-qubit rotation about the X axis.");
    impl_gate!(ry, theta: f64, "Single-qubit rotation about the Y axis.");
    impl_gate!(rz, phi: f64, "Single-qubit rotation about the Z axis.");
    impl_gate!(s, "Single qubit S gate (√Z).");
    impl_gate!(sdg, "Single qubit S-adjoint gate (S†).");
    impl_gate!(sx, "The single-qubit Sqrt(X) gate (√X).");
    impl_gate!(sxdg, "The inverse single-qubit Sqrt(X) gate.");
    impl_gate!(t, "Single qubit T gate (∜Z).");
    impl_gate!(tdg, "Single qubit T-adjoint gate (T†).");
    impl_gate!(u, theta: f64, phi: f64, lam: f64, "Generic single-qubit rotation in terms of ZYZ Euler angles.");
    impl_gate!(x, "The single-qubit Pauli-X gate (σ_x).");
    impl_gate!(y, "The single-qubit Pauli-Y gate (σ_y).");
    impl_gate!(z, "The single-qubit Pauli-Z gate (σ_z).");
}

/// 2 qubit gates
impl QuantumCircuit {
    impl_gate!(ch, [q0, q1], "Controlled-Hadamard gate.");
    impl_gate!(cp, theta: f64, [q0, q1], "Controlled-Phase gate.");
    impl_gate!(crx, theta: f64, [q0, q1], "Controlled-RX gate.");
    impl_gate!(cry, theta: f64, [q0, q1], "Controlled-RY gate.");
    impl_gate!(crz, theta: f64, [q0, q1], "Controlled-RZ gate.");
    impl_gate!(cs, [q0, q1], "Controlled-S gate.");
    impl_gate!(csdg, [q0, q1], "Controlled-S† gate.");
    impl_gate!(csx, [q0, q1], "Controlled-√X gate.");
    impl_gate!(cu, theta: f64, phi: f64, lam: f64, gamma: f64, [q0, q1], "Controlled-U gate (4-parameter two-qubit gate).");
    impl_gate!(cx, [q0, q1], "Controlled-X gate.");
    impl_gate!(cnot, [q0, q1], "Controlled-X gate.");
    impl_gate!(cy, [q0, q1], "Controlled-Y gate.");
    impl_gate!(cz, [q0, q1], "Controlled-Z gate.");
    impl_gate!(dcx, [q0, q1], "Double-CX gate.");
    impl_gate!(ecr, [q0, q1], "An echoed cross-resonance gate.");
    impl_gate!(iswap, [q0, q1], "iSWAP gate.");
    impl_gate!(rxx, theta: f64, [q0, q1], "A parametric 2-qubit X⊗X interaction (rotation about XX).");
    impl_gate!(ryy, theta: f64, [q0, q1], "A parametric 2-qubit Y⊗Y interaction (rotation about YY).");
    impl_gate!(rzx, theta: f64, [q0, q1], "A parametric 2-qubit Z⊗X interaction (rotation about ZX).");
    impl_gate!(rzz, theta: f64, [q0, q1], "A parametric 2-qubit Z⊗Z interaction (rotation about ZZ).");
    impl_gate!(swap, [q0, q1], "The SWAP gate.");
    impl_gate!(xx_minus_yy, theta: f64, beta: f64, [q0, q1], "XX-YY interaction gate.");
    impl_gate!(xx_plus_yy, theta: f64, beta: f64, [q0, q1], "XX+YY interaction gate.");
}

/// 3+ qubit gates
impl QuantumCircuit {
    impl_gate!(
        c3sx,
        [q0, q1, q2, q3],
        "The 3-qubit controlled sqrt-X gate."
    );
    impl_gate!(c3x, [q0, q1, q2, q3], "The X gate controlled on 3 qubits.");
    impl_gate!(c4x, [q0, q1, q2, q3, q4], "The 4-qubit controlled X gate.");
    impl_gate!(ccx, [q0, q1, q2], "CCX gate, also known as Toffoli gate.");
    impl_gate!(ccz, [q0, q1, q2], "CCZ gate.");
    impl_gate!(
        cswap,
        [q0, q1, q2],
        "Controlled-SWAP gate, also known as the Fredkin gate."
    );
    impl_gate!(
        fredkin,
        [q0, q1, q2],
        "Controlled-SWAP gate, also known as the Fredkin gate."
    );
    impl_gate!(
        rccx,
        [q0, q1, q2],
        "The simplified Toffoli gate, also referred to as Margolus gate."
    );
    impl_gate!(
        rcccx,
        [q0, q1, q2, q3],
        "The simplified 3-controlled Toffoli gate."
    );
}

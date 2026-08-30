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
            Qubits::Single(q) => vec![q],
            Qubits::Array(qs) => qs,
        };
        let state_len = self.qubits.len();
        let n = state_len.trailing_zeros() as usize;
        let k = qubits.len();
        let local_dim = 1 << k;

        assert_eq!(
            qubits.len(),
            gate.size.ilog2() as usize,
            "Cannot apply {} qubit gate to {} qubit(s)",
            gate.size.ilog2(),
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
            let output = gate.matrix.dot(&input);
            // write output back into qubits
            // indices = [000, 001, 100, 101]
            // qubits[indices[n]] = output[n]
            for (local_index, global_index) in indices.enumerate() {
                self.qubits[global_index] = output[local_index];
            }
        }
    }
}

macro_rules! impl_gate {
    (1, $name:ident, $($arg:ident: $ty:ty,)* $doc:literal) => {
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
    (2, $name:ident, $($arg:ident: $ty:ty,)* $doc:literal) => {
        #[doc = $doc]
        pub fn $name(&mut self, $($arg: $ty,)* q0: usize, q1: usize) {
            self.unitary(&$crate::Gate::$name($($arg,)*), &[q0, q1])
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
    impl_gate!(1, h, "Single-qubit Hadamard gate");
    impl_gate!(1, i, "Identity gate.");
    impl_gate!(1, id, "Identity gate.");
    impl_gate!(1, p, theta: f64, "Single-qubit rotation around the Z axis.");
    impl_gate!(1, r, theta: f64, phi: f64, "Rotation θ around the cos(ϕ)x+sin(ϕ)y axis.");
    impl_gate!(1, rx, theta: f64, "Single-qubit rotation about the X axis.");
    impl_gate!(1, ry, theta: f64, "Single-qubit rotation about the Y axis.");
    impl_gate!(1, rz, phi: f64, "Single-qubit rotation about the Z axis.");
    impl_gate!(1, s, "Single qubit S gate (√Z).");
    impl_gate!(1, sdg, "Single qubit S-adjoint gate (S†).");
    impl_gate!(1, sx, "The single-qubit Sqrt(X) gate (√X).");
    impl_gate!(1, sxdg, "The inverse single-qubit Sqrt(X) gate.");
    impl_gate!(1, t, "Single qubit T gate (∜Z).");
    impl_gate!(1, tdg, "Single qubit T-adjoint gate (T†).");
    impl_gate!(1, u, theta: f64, phi: f64, lam: f64, "Generic single-qubit rotation in terms of ZYZ Euler angles.");
    impl_gate!(1, x, "The single-qubit Pauli-X gate (σ_x).");
    impl_gate!(1, y, "The single-qubit Pauli-Y gate (σ_y).");
    impl_gate!(1, z, "The single-qubit Pauli-Z gate (σ_z).");
}

/// 2 qubit gates
impl QuantumCircuit {
    impl_gate!(2, ch, "Controlled-Hadamard gate.");
    impl_gate!(2, cp, theta: f64, "Controlled-Phase gate.");
    impl_gate!(2, crx, theta: f64, "Controlled-RX gate.");
    impl_gate!(2, cry, theta: f64, "Controlled-RY gate.");
    impl_gate!(2, crz, theta: f64, "Controlled-RZ gate.");
    impl_gate!(2, cs, "Controlled-S gate.");
    impl_gate!(2, csdg, "Controlled-S† gate.");
    impl_gate!(2, csx, "Controlled-√X gate.");
    impl_gate!(2, cu, theta: f64, phi: f64, lam: f64, gamma: f64, "Controlled-U gate (4-parameter two-qubit gate).");
    impl_gate!(2, cx, "Controlled-X gate.");
    impl_gate!(2, cnot, "Controlled-X gate.");
    impl_gate!(2, cy, "Controlled-Y gate.");
    impl_gate!(2, cz, "Controlled-Z gate.");
    impl_gate!(2, dcx, "Double-CX gate.");
    impl_gate!(2, ecr, "An echoed cross-resonance gate.");
    impl_gate!(2, iswap, "iSWAP gate.");
    impl_gate!(2, rxx, theta: f64, "A parametric 2-qubit X⊗X interaction (rotation about XX).");
    impl_gate!(2, ryy, theta: f64, "A parametric 2-qubit Y⊗Y interaction (rotation about YY).");
    impl_gate!(2, rzx, theta: f64, "A parametric 2-qubit Z⊗X interaction (rotation about ZX).");
    impl_gate!(2, rzz, theta: f64, "A parametric 2-qubit Z⊗Z interaction (rotation about ZZ).");
    impl_gate!(2, swap, "The SWAP gate.");
    impl_gate!(2, xx_minus_yy, theta: f64, beta: f64, "XX-YY interaction gate.");
    impl_gate!(2, xx_plus_yy, theta: f64, beta: f64, "XX+YY interaction gate.");
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

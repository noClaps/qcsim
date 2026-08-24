use crate::QuantumCircuit;

macro_rules! impl_gate {
    ($name:ident, $($arg:ident: $ty:ty,)* $doc:literal) => {
        #[doc = $doc]
        pub fn $name(&mut self, $($arg: $ty,)* q0: usize, q1: usize) {
            self.unitary(&$crate::Gate::$name($($arg,)*), &[q0, q1])
        }
    };
}

/// 2 qubit gates
impl QuantumCircuit {
    impl_gate!(ch, "Controlled-Hadamard gate.");
    impl_gate!(cp, theta: f64, "Controlled-Phase gate.");
    impl_gate!(crx, theta: f64, "Controlled-RX gate.");
    impl_gate!(cry, theta: f64, "Controlled-RY gate.");
    impl_gate!(crz, theta: f64, "Controlled-RZ gate.");
    impl_gate!(cs, "Controlled-S gate.");
    impl_gate!(csdg, "Controlled-S† gate.");
    impl_gate!(csx, "Controlled-√X gate.");
    impl_gate!(cu, theta: f64, phi: f64, lam: f64, gamma: f64, "Controlled-U gate (4-parameter two-qubit gate).");
    impl_gate!(cx, "Controlled-X gate.");
    impl_gate!(cnot, "Controlled-X gate.");
    impl_gate!(cy, "Controlled-Y gate.");
    impl_gate!(cz, "Controlled-Z gate.");
    impl_gate!(dcx, "Double-CX gate.");
    impl_gate!(ecr, "An echoed cross-resonance gate.");
    impl_gate!(iswap, "iSWAP gate.");
    impl_gate!(rxx, theta: f64, "A parametric 2-qubit X⊗X interaction (rotation about XX).");
    impl_gate!(ryy, theta: f64, "A parametric 2-qubit Y⊗Y interaction (rotation about YY).");
    impl_gate!(rzx, theta: f64, "A parametric 2-qubit Z⊗X interaction (rotation about ZX).");
    impl_gate!(rzz, theta: f64, "A parametric 2-qubit Z⊗Z interaction (rotation about ZZ).");
    impl_gate!(swap, "The SWAP gate.");
    impl_gate!(xx_minus_yy, theta: f64, beta: f64, "XX-YY interaction gate.");
    impl_gate!(xx_plus_yy, theta: f64, beta: f64, "XX+YY interaction gate.");
}

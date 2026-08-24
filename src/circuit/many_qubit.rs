use crate::QuantumCircuit;

macro_rules! impl_gate {
    ($name:ident, $($arg:ident: $ty:ty,)* [$($qubit:ident),* $(,)?], $doc:literal) => {
        #[doc = $doc]
        pub fn $name(&mut self, $($arg: $ty,)* $($qubit: usize,)*) {
            self.unitary(&$crate::Gate::$name($($arg,)*), &[$($qubit,)*])
        }
    };
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

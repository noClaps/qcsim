use crate::QuantumCircuit;

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

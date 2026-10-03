use crate::{Complex, Gate, gate::gate};

const I: Complex = Complex::I;

/// 3+ qubit gates
impl Gate {
    /// The 3-qubit controlled sqrt-X gate.
    #[must_use]
    pub fn c3sx() -> Self {
        Self::sx().control(3)
    }
    /// The X gate controlled on 3 qubits.
    #[must_use]
    pub fn c3x() -> Self {
        Self::x().control(3)
    }
    /// The 4-qubit controlled X gate.
    #[must_use]
    pub fn c4x() -> Self {
        Self::x().control(4)
    }
    /// CCX gate, also known as Toffoli gate.
    #[must_use]
    pub fn ccx() -> Self {
        Self::x().control(2)
    }
    /// CCZ gate.
    #[must_use]
    pub fn ccz() -> Self {
        Self::z().control(2)
    }
    /// Controlled-SWAP gate, also known as the Fredkin gate.
    #[must_use]
    pub fn cswap() -> Self {
        Self::swap().control(1)
    }
    /// Controlled-SWAP gate, also known as the Fredkin gate.
    #[must_use]
    pub fn fredkin() -> Self {
        Self::cswap()
    }
    /// The simplified Toffoli gate, also referred to as Margolus gate.
    #[must_use]
    pub fn rccx() -> Self {
        Self::Normal(gate![
            [1., 0., 0., 0., 0., 0., 0., 0.],
            [0., 1., 0., 0., 0., 0., 0., 0.],
            [0., 0., 1., 0., 0., 0., 0., 0.],
            [0., 0., 0., 1., 0., 0., 0., 0.],
            [0., 0., 0., 0., 1., 0., 0., 0.],
            [0., 0., 0., 0., 0., -1., 0., 0.],
            [0., 0., 0., 0., 0., 0., 0., -I],
            [0., 0., 0., 0., 0., 0., I, 0.],
        ])
    }
    /// The simplified 3-controlled Toffoli gate.
    #[must_use]
    pub fn rcccx() -> Self {
        Gate::Normal(gate![
            [
                1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.
            ],
            [
                0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.
            ],
            [
                0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.
            ],
            [
                0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.
            ],
            [
                0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.
            ],
            [
                0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0.
            ],
            [
                0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0., 0.
            ],
            [
                0., 0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0., 0.
            ],
            [
                0., 0., 0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 0., 0.
            ],
            [
                0., 0., 0., 0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0., 0.
            ],
            [
                0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0., 0.
            ],
            [
                0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 1., 0., 0., 0., 0.
            ],
            [
                0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., I, 0., 0., 0.
            ],
            [
                0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., -I, 0., 0.
            ],
            [
                0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 1.
            ],
            [
                0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., -1., 0.
            ]
        ])
    }
}

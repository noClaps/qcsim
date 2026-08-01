use qcsim_matrix::Matrix;

mod clifford;
mod identity;

#[derive(Clone)]
pub enum Ctrl {
    Ctrl(usize),
    NegCtrl(usize),
}

#[derive(Clone)]
pub struct Gate {
    pub matrix: Matrix,
    pub ctrl: Option<Ctrl>,
}

impl Gate {
    pub const fn new(matrix: Matrix) -> Self {
        Self { matrix, ctrl: None }
    }
    pub const fn ctrl(&mut self, ctrl: usize) {
        self.ctrl = Some(Ctrl::Ctrl(ctrl))
    }
    pub fn with_ctrl(self, ctrl: usize) -> Self {
        Self {
            matrix: self.matrix,
            ctrl: Some(Ctrl::Ctrl(ctrl)),
        }
    }
    pub const fn neg_ctrl(&mut self, ctrl: usize) {
        self.ctrl = Some(Ctrl::NegCtrl(ctrl))
    }
    pub fn with_neg_ctrl(self, ctrl: usize) -> Self {
        Self {
            matrix: self.matrix,
            ctrl: Some(Ctrl::NegCtrl(ctrl)),
        }
    }
    pub fn pow(&mut self, pow: i128) {
        self.matrix = self.matrix.pow(pow);
    }
    pub fn inv(&mut self) {
        self.matrix = self.matrix.adjoint();
    }
}

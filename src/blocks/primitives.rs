// use crate::block::{Block};

pub struct Primitive {
    pub n_qubits: u32,
}

impl Primitive {
    pub fn new(n_qubits: u32) -> Primitive {
	Primitive {
	    n_qubits: n_qubits,
	}
    }
}

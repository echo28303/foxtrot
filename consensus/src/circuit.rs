// consensus/src/circuit.rs

use winterfell::{
    math::fields::f128::BaseElement, 
    FieldExtension, HashFunction, ProofOptions, StarkProof, Trace, VerifierError, 
};
use winterfell::crypto::hashers::Blake3_256;

pub struct SquareRootCircuit {
    pub x: BaseElement,
    pub y: BaseElement,
}

impl SquareRootCircuit {
    pub fn new(x: BaseElement, y: BaseElement) -> Self {
        Self { x, y }
    }

    pub fn generate_trace(&self) -> Vec<BaseElement> {
        vec![self.x, self.x * self.x]
    }
}


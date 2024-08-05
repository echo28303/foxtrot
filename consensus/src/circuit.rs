use winterfell::{FieldExtension, HashFunction, ProofOptions, StarkProof, Trace, VerifierError};
use blake3::Hasher;
use winter_math::fields::f128::BaseElement;


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


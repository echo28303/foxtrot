// consensus/src/trace.rs

use winterfell::{
    math::fields::f128::BaseElement, 
    FieldExtension, HashFunction, ProofOptions, StarkProof, Trace, VerifierError, 
};
use winterfell::crypto::hashers::Blake3_256;
use crate::circuit::SquareRootCircuit;

pub fn generate_square_root_trace(x: BaseElement, y: BaseElement) -> Vec<BaseElement> {
    let circuit = SquareRootCircuit::new(x, y);
    circuit.generate_trace()
}


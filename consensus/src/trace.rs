use winter_math::fields::f128::BaseElement;
use crate::circuit::SquareRootCircuit;

pub fn generate_square_root_trace(x: BaseElement, y: BaseElement) -> Vec<BaseElement> {
    let circuit = SquareRootCircuit::new(x, y);
    circuit.generate_trace()
}

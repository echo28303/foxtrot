use winterfell::TraceTable;
use winter_math::ToElements; // Import ToElements from winter_math
use winter_math::fields::f128::BaseElement;

pub struct PublicInputs {
    pub inputs: Vec<BaseElement>,
}

impl ToElements<BaseElement> for PublicInputs {
    fn to_elements(&self) -> Vec<BaseElement> {
        self.inputs.clone()
    }
}

pub fn create_trace(inputs: Vec<BaseElement>) -> TraceTable<BaseElement> {
    let mut trace = TraceTable::new(1, inputs.len());
    for (i, input) in inputs.iter().enumerate() {
        trace.set(0, i, *input);
    }
    trace
}

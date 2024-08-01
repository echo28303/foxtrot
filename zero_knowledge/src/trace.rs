use winter_prover::TraceTable; // Use TraceTable from winter-prover
use winter_math::{ ToElements, FieldElement };
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
    let trace_length = if inputs.len() >= 8 { inputs.len() } else { 8 }; // Ensure trace length is at least 8
    let mut trace = TraceTable::new(1, trace_length);

    // Fill the trace with provided inputs or pad with zeros if inputs are less than trace_length
    for (i, input) in inputs.iter().enumerate().take(trace_length) {
        trace.set(0, i, *input);
    }
    for i in inputs.len()..trace_length {
        trace.set(0, i, BaseElement::ZERO); // Pad with zero if inputs are less than trace_length
    }

    trace
}

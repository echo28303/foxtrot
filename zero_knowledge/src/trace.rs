use winter_prover::TraceTable; // Use TraceTable from winter-prover
use winter_math::{ToElements, FieldElement};
use winter_math::fields::f128::BaseElement;
use blake3::Hasher; // Import Blake3 hasher
use winter_utils::Serializable;
use winterfell::Trace;

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

pub fn generate_blake3_hash(trace: &TraceTable<BaseElement>) -> [u8; 32] {
    let mut hasher = Hasher::new();
    for row in 0..trace.length() {
        for col in 0..trace.width() {
            let elem = trace.get(col, row);
            hasher.update(&Serializable::to_bytes(&elem));
        }
    }
    *hasher.finalize().as_bytes()
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

impl ToBytes for BaseElement {
    fn to_bytes(&self) -> Vec<u8> {
        Serializable::to_bytes(self)
    }
}

impl ToBytes for TraceTable<BaseElement> {
    fn to_bytes(&self) -> Vec<u8> {
        let num_rows = self.length();
        let num_columns = self.width();
        let mut bytes = Vec::new();

        for row in 0..num_rows {
            for col in 0..num_columns {
                let elem = self.get(col, row);
                bytes.extend_from_slice(&Serializable::to_bytes(&elem));
            }
        }
        bytes
    }
}

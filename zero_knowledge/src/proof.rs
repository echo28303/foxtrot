use winter_crypto::{hashers::Blake3_256, DefaultRandomCoin, RandomCoin};
use winter_verifier::{verify, Proof as WinterProof, VerifierError, AcceptableOptions};
use winter_prover::{TraceTable, Trace};
use winter_air::{ProofOptions, Air, FieldExtension};
use crate::air::{YourAir, PublicInputs};
use crate::errors::CustomError;
use crate::prover::YourProver;
use std::sync::Arc;
use winter_math::{fields::f128::BaseElement, FieldElement, StarkField};
use winterfell::Prover;
pub use winter_verifier::Proof;
use blake3::Hasher;
use winter_fri::FriOptions; // Import FriOptions

pub struct ZkProver {
    trace: TraceTable<BaseElement>,
}

impl ZkProver {
    pub fn new(trace: TraceTable<BaseElement>) -> Self {
        Self { trace }
    }

    pub fn generate_proof(&self) -> Result<WinterProof, CustomError> {
        let trace_info = self.trace.info().clone();
        let proof_options = ProofOptions::new(
            8,   // Number of queries
            16,  // Blowup factor
            0,   // Grinding factor
            FieldExtension::None,
            4,   // FRI folding factor
            16,  // FRI reduction factor
        );
        let fri_options = FriOptions::new(16, 4, 15); // Ensure proper configuration with FriOptions
        let air = Arc::new(YourAir::new(trace_info, PublicInputs { inputs: vec![] }, proof_options));
        let seed = [BaseElement::new(1)];
        let coin = DefaultRandomCoin::<Blake3_256<BaseElement>>::new(&seed);
        let prover = YourProver::new(air.clone(), coin, &fri_options); // Pass FriOptions by reference

        // Use air and fri_options to avoid the unused variable warning
        println!("Air context created with trace info: {:?}", air.trace_info());
        println!("Using FRI options with blowup factor: {}", fri_options.blowup_factor());

        let proof = prover.prove(self.trace.clone()).map_err(CustomError::ProverError)?;
        Ok(proof)
    }
}

pub fn verify_proof(proof: WinterProof) -> Result<(), VerifierError> {
    let trace_info = proof.context.trace_info().clone();
    let air = YourAir::new(trace_info, PublicInputs { inputs: vec![] }, proof.context.options().clone());

    // Use air to avoid the unused variable warning
    println!("Verifying with Air context: {:?}", air.trace_info());

    let acceptable_options = AcceptableOptions::OptionSet(vec![proof.context.options().clone()]);

    verify::<YourAir, Blake3_256<BaseElement>, DefaultRandomCoin<Blake3_256<BaseElement>>>(
        proof,
        PublicInputs { inputs: vec![] },
        &acceptable_options,
    )
}

// Define create_trace function
pub fn create_trace<E: FieldElement + StarkField>(values: Vec<E>) -> TraceTable<E> {
    let mut trace_table = TraceTable::new(1, values.len());
    for (i, value) in values.into_iter().enumerate() {
        trace_table.set(0, i, value);
    }
    trace_table
}

// Generate a Blake3 hash
pub fn generate_blake3_hash(data: &[u8]) -> [u8; 32] {
    let mut hasher = Hasher::new();
    hasher.update(data);
    *hasher.finalize().as_bytes()
}

use winter_math::{fields::f128::BaseElement, FieldElement};
use winter_crypto::{hashers::Blake3_256, DefaultRandomCoin};
use winter_air::{Air, TraceInfo, AuxRandElements, ProofOptions};
use winter_prover::{Prover, StarkDomain, TracePolyTable, DefaultTraceLde};
use winter_prover::matrix::ColMatrix;
use winterfell::TraceTable;
use winter_fri::FriOptions; // Import FriOptions
use std::sync::Arc;
use crate::air::{YourAir, PublicInputs};
use crate::evaluator::YourConstraintEvaluator;

pub struct YourProver<'a> {
    air: Arc<YourAir>,
    _coin: DefaultRandomCoin<Blake3_256<BaseElement>>,
    _fri_options: &'a FriOptions, // Store FriOptions by reference
}

impl<'a> YourProver<'a> {
    pub fn new(air: Arc<YourAir>, coin: DefaultRandomCoin<Blake3_256<BaseElement>>, fri_options: &'a FriOptions) -> Self {
        Self { air, _coin: coin, _fri_options: fri_options }
    }
}

impl<'a> Prover for YourProver<'a> {
    type BaseField = BaseElement;
    type Air = YourAir;
    type Trace = TraceTable<Self::BaseField>;
    type HashFn = Blake3_256<Self::BaseField>;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;
    type TraceLde<E: FieldElement<BaseField = Self::BaseField>> = DefaultTraceLde<E, Self::HashFn>;
    type ConstraintEvaluator<'b, E: FieldElement<BaseField = Self::BaseField>> = YourConstraintEvaluator<'b, E>;

    fn options(&self) -> &ProofOptions {
        &self.air.options()
    }

    fn get_pub_inputs(&self, _trace: &Self::Trace) -> <Self::Air as Air>::PublicInputs {
        PublicInputs { inputs: vec![] }
    }

    fn new_trace_lde<E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        _trace_info: &TraceInfo,
        _trace: &ColMatrix<Self::BaseField>,
        _domain: &StarkDomain<Self::BaseField>,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>) {
        // Use fri_options in creating the trace LDE
        if self._fri_options.blowup_factor() == 16 {
            println!("FRI options blowup factor is set correctly.");
        }
        // Your implementation for creating a new trace LDE
        unimplemented!()
    }

    fn new_evaluator<'b, E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        air: &'b Self::Air,
        _aux_rand_elements: Option<AuxRandElements<E>>,
        coefficients: winter_prover::ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'b, E> {
        YourConstraintEvaluator {
            air,
            periodic_values: vec![],
            coefficients,
        }
    }
}

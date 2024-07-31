use winter_math::{fields::f128::BaseElement, FieldElement};
use winter_crypto::{hashers::Blake3_256, DefaultRandomCoin};
use winter_air::{Air, TraceInfo, AuxRandElements, ProofOptions};
use winter_prover::{Prover, StarkDomain, TracePolyTable, DefaultTraceLde};
use winter_prover::matrix::ColMatrix;
use winterfell::TraceTable;
use std::sync::Arc;
use crate::air::{YourAir, PublicInputs};
use crate::evaluator::YourConstraintEvaluator;

pub struct YourProver {
    air: Arc<YourAir>,
    coin: DefaultRandomCoin<Blake3_256<BaseElement>>,
}

impl YourProver {
    pub fn new(air: Arc<YourAir>, coin: DefaultRandomCoin<Blake3_256<BaseElement>>) -> Self {
        Self { air, coin }
    }
}

impl Prover for YourProver {
    type BaseField = BaseElement;
    type Air = YourAir;
    type Trace = TraceTable<Self::BaseField>;
    type HashFn = Blake3_256<Self::BaseField>;
    type RandomCoin = DefaultRandomCoin<Self::HashFn>;
    type TraceLde<E: FieldElement<BaseField = Self::BaseField>> = DefaultTraceLde<E, Self::HashFn>;
    type ConstraintEvaluator<'a, E: FieldElement<BaseField = Self::BaseField>> = YourConstraintEvaluator<'a, E>;

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
        // Your implementation for creating a new trace LDE
        unimplemented!()
    }

    fn new_evaluator<'a, E: FieldElement<BaseField = Self::BaseField>>(
        &self,
        air: &'a Self::Air,
        _aux_rand_elements: Option<AuxRandElements<E>>,
        coefficients: winter_prover::ConstraintCompositionCoefficients<E>,
    ) -> Self::ConstraintEvaluator<'a, E> {
        YourConstraintEvaluator {
            air,
            periodic_values: vec![],
            coefficients,
        }
    }
}

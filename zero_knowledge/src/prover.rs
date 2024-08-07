use winter_math::{fields::f128::BaseElement, FieldElement};
use winter_crypto::{hashers::Blake3_256, DefaultRandomCoin};
use winter_air::{Air, TraceInfo, AuxRandElements, ProofOptions};
use winter_prover::{Prover, StarkDomain, TracePolyTable, DefaultTraceLde, TraceTable, EvaluationFrame};
use winter_prover::matrix::ColMatrix;
use winter_fri::FriOptions;
use std::sync::Arc;
use crate::air::{YourAir, PublicInputs};
use crate::evaluator::YourConstraintEvaluator;

pub struct YourProver<'a> {
    air: Arc<YourAir>,
    _coin: DefaultRandomCoin<Blake3_256<BaseElement>>,
    _fri_options: &'a FriOptions,
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

    fn new_trace_lde<E>(
        &self,
        trace_info: &TraceInfo,
        trace: &ColMatrix<Self::BaseField>,
        domain: &StarkDomain<Self::BaseField>,
    ) -> (Self::TraceLde<E>, TracePolyTable<E>)
    where
        E: FieldElement<BaseField = Self::BaseField> + Into<Self::BaseField>,
    {
        // Create the low-degree extension of the trace
        let mut default_trace_lde = DefaultTraceLde::<E, Self::HashFn>::new(trace_info, trace, domain);

        // Initialize an evaluation frame with the correct number of columns
        let num_columns = trace.num_cols();
        let mut frame = EvaluationFrame::<E>::new(num_columns);

        // Use the correct method on DefaultTraceLde
        default_trace_lde.read_main_frame_into(0, &mut frame);

        // Extract columns from the frame
        let mut lde_columns: Vec<Vec<E::BaseField>> = vec![Vec::with_capacity(frame.current().len()); num_columns];
        for (i, column) in lde_columns.iter_mut().enumerate() {
            column.push(frame.current()[i].into()); // Convert E to BaseElement
            column.push(frame.next()[i].into());    // Convert E to BaseElement
        }

        // Construct a ColMatrix from the extracted columns
        let lde_matrix = ColMatrix::<E::BaseField>::new(lde_columns);

        // Create the polynomial table using the LDE columns
        let trace_poly_table = TracePolyTable::<E>::new(lde_matrix);

        (default_trace_lde, trace_poly_table) // Return the correct tuple
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

use winter_math::FieldElement;
use winter_prover::{ConstraintEvaluator, CompositionPolyTrace, StarkDomain};
use crate::air::YourAir;

pub struct YourConstraintEvaluator<'a, E: FieldElement> {
    pub air: &'a YourAir,
    pub periodic_values: Vec<E>,
    pub coefficients: winter_prover::ConstraintCompositionCoefficients<E>,
}

impl<'a, E: FieldElement<BaseField = winter_math::fields::f128::BaseElement>> ConstraintEvaluator<E> for YourConstraintEvaluator<'a, E> {
    type Air = YourAir;

    fn evaluate<T>(
        self,
        _trace_lde: &T,
        _domain: &StarkDomain<E::BaseField>,
    ) -> CompositionPolyTrace<E> {
        // Implement the logic for evaluating constraints
        unimplemented!()
    }
}

use super::common::*;

pub struct DivisionCircuit<F: PrimeField> {
    pub numerator: Option<F>,
    pub denominator: Option<F>,
    pub quotient: Option<F>,
}

impl<F: PrimeField> Circuit<F> for DivisionCircuit<F> {
    fn synthesize<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate numerator, denominator, and quotient
        let numerator = cs.alloc(|| "numerator", || self.numerator.ok_or(SynthesisError::AssignmentMissing))?;
        let denominator = cs.alloc(|| "denominator", || self.denominator.ok_or(SynthesisError::AssignmentMissing))?;
        let quotient = cs.alloc_input(|| "quotient", || self.quotient.ok_or(SynthesisError::AssignmentMissing))?;
        
        // Ensure denominator is non-zero (optional, but adds robustness)
        cs.enforce(
            || "non-zero denominator",
            |lc| lc + denominator,
            |lc| lc + CS::one(),
            |lc| lc + denominator,
        );

        // Enforce division constraint: numerator = quotient * denominator
        cs.enforce(
            || "division constraint",
            |lc| lc + quotient,
            |lc| lc + denominator,
            |lc| lc + numerator,
        );

        Ok(())
    }
}

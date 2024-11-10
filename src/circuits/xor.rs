use super::common::*;

// Circuit: a XOR b = result
pub struct XorCircuit<F: PrimeField> {
    pub a: Option<F>,
    pub b: Option<F>,
    pub result: Option<F>,
}

impl<F: PrimeField> Circuit<F> for XorCircuit<F> {
    fn synthesize<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let a = cs.alloc(|| "a", || self.a.grab())?;
        let b = cs.alloc(|| "b", || self.b.grab())?;
        let result = cs.alloc_input(|| "result", || self.result.grab())?;

        let two_ab = cs.alloc(
            || "2 * a * b",
            || Ok(F::from(2) * self.a.grab()? * self.b.grab()?),
        )?;

        // Enforce 2 * a * b = result
        cs.enforce(
            || "2 * a * b = result",
            |lc| lc + (F::from(2), a),
            |lc| lc + b,
            |lc| lc + two_ab,
        );

        // XOR constraint: a + b - 2 * a * b = result
        cs.enforce(
            || "xor constraint",
            |lc| lc + a + b - two_ab,
            |lc| lc + CS::one(),
            |lc| lc + result,
        );

        Ok(())
    }
}

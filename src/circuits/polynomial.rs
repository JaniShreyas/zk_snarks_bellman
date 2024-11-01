use super::common::*;

// Circuit is: 2 * x_squared + 3y + 5 = z
pub struct PolynomialCircuit<F: PrimeField> {
    pub x: Option<F>,
    pub y: Option<F>,
    pub z: Option<F>,
}

impl<F: PrimeField> Circuit<F> for PolynomialCircuit<F> {
    fn synthesize<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let x = cs.alloc(|| "x", || self.x.grab())?;
        let y = cs.alloc(|| "y", || self.y.grab())?;
        let x_squared = cs.alloc(|| "x^2", || self.x.grab().map(|e| e.square()))?;

        let z = cs.alloc_input(|| "z", || self.z.grab())?;

        // MISSING CONSTRAINT: We need to enforce that x_squared = x * x

        // Here's how the implementation should look:

        // 1. Enforce x * x = x_squared
        cs.enforce(
            || "x squared constraint",
            |lc| lc + x,
            |lc| lc + x,
            |lc| lc + x_squared,
        );

        // 2. Enforce 2 * x_squared + 3y + 5 = z
        cs.enforce(
            || "z constraint",
            |lc| lc + (F::from(2), x_squared) + (F::from(3), y) + (F::from(5), CS::one()),
            |lc| lc + CS::one(),
            |lc| lc + z,
        );

        Ok(())
    }
}
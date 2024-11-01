use super::common::*;

// The circuit implements
// ax^3y^2 + bx^2y + cxy + d = result
pub struct MultiVarPolynomialCircuit<F: PrimeField> {
    pub x: Option<F>,
    pub y: Option<F>,
    pub a: Option<F>,
    pub b: Option<F>,
    pub c: Option<F>,
    pub d: Option<F>,
    pub result: Option<F>,
}

impl<F: PrimeField> Circuit<F> for MultiVarPolynomialCircuit<F> {
    fn synthesize<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate input variables
        let x = cs.alloc(|| "x", || self.x.grab())?;
        let y = cs.alloc(|| "y", || self.y.grab())?;
        let a = cs.alloc(|| "a", || self.a.grab())?;
        let b = cs.alloc(|| "b", || self.b.grab())?;
        let c = cs.alloc(|| "c", || self.c.grab())?;
        let d = cs.alloc(|| "d", || self.d.grab())?;

        // Allocate result as an input (since it’s verified externally)
        let result = cs.alloc_input(|| "result", || self.result.grab())?;

        // Step 1: Intermediate terms for x^2, x^3, and y^2
        let x2 = cs.alloc(|| "x^2", || Ok(self.x.grab()? * self.x.grab()?))?;
        cs.enforce(|| "x^2 constraint", |lc| lc + x, |lc| lc + x, |lc| lc + x2);

        let x3 = cs.alloc(
            || "x^3",
            || Ok(self.x.grab()? * self.x.grab()? * self.x.grab()?),
        )?;
        cs.enforce(|| "x^3 constraint", |lc| lc + x2, |lc| lc + x, |lc| lc + x3);

        let y2 = cs.alloc(|| "y^2", || Ok(self.y.grab()? * self.y.grab()?))?;
        cs.enforce(|| "y^2 constraint", |lc| lc + y, |lc| lc + y, |lc| lc + y2);

        // Step 2: Terms for ax^3y^2, bx^2y, and cxy
        let ax3 = cs.alloc(
            || "ax^3",
            || Ok(self.a.grab()? * self.x.grab()? * self.x.grab()? * self.x.grab()?),
        )?;
        cs.enforce(
            || "ax^3 constraint",
            |lc| lc + a,
            |lc| lc + x3,
            |lc| lc + ax3,
        );

        let ax3y2 = cs.alloc(
            || "ax^3y^2",
            || {
                Ok(self.a.grab()?
                    * self.x.grab()?
                    * self.x.grab()?
                    * self.x.grab()?
                    * self.y.grab()?
                    * self.y.grab()?)
            },
        )?;

        cs.enforce(
            || "ax^3y^2 constraint",
            |lc| lc + ax3,
            |lc| lc + y2,
            |lc| lc + ax3y2,
        );

        let bx2 = cs.alloc(
            || "bx^2",
            || Ok(self.b.grab()? * self.x.grab()? * self.x.grab()?),
        )?;
        cs.enforce(
            || "bx^2 constraint",
            |lc| lc + b,
            |lc| lc + x2,
            |lc| lc + bx2,
        );

        let bx2y = cs.alloc(
            || "bx^2y",
            || Ok(self.b.grab()? * self.x.grab()? * self.x.grab()? * self.y.grab()?),
        )?;

        cs.enforce(
            || "bx^2y constraint",
            |lc| lc + bx2,
            |lc| lc + y,
            |lc| lc + bx2y,
        );

        let cx = cs.alloc(|| "cx", || Ok(self.c.grab()? * self.x.grab()?))?;
        cs.enforce(|| "cx constraint", |lc| lc + c, |lc| lc + x, |lc| lc + cx);

        let cxy = cs.alloc(
            || "cxy",
            || Ok(self.c.grab()? * self.x.grab()? * self.y.grab()?),
        )?;

        cs.enforce(
            || "cxy constraint",
            |lc| lc + cx,
            |lc| lc + y,
            |lc| lc + cxy,
        );

        // Step 3: Enforce the final polynomial constraint: ax^3y^2 + bx^2y + cxy + d = result
        cs.enforce(
            || "polynomial constraint",
            |lc| lc + ax3y2 + bx2y + cxy + d,
            |lc| lc + CS::one(),
            |lc| lc + result,
        );

        Ok(())
    }
}

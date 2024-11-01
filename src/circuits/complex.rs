use super::common::*;

// A circuit that implements:
// 1. Polynomial evaluation: ax³ + bx² + cx + d
// 2. Matrix multiplication (2x2)
pub struct ComplexCircuit<F: PrimeField> {
    // Polynomial inputs
    pub x: Option<F>,
    pub a: Option<F>,
    pub b: Option<F>,
    pub c: Option<F>,
    pub d: Option<F>,

    // Matrix 1 (2x2)
    pub m1: Option<[[F; 2]; 2]>,
    // Matrix 2 (2x2)
    pub m2: Option<[[F; 2]; 2]>,

    // Expected outputs
    pub poly_result: Option<F>,
    pub matrix_result: Option<[[F; 2]; 2]>,
}

impl<F: PrimeField> Circuit<F> for ComplexCircuit<F> {
    fn synthesize<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate polynomial inputs
        let d = cs.alloc(|| "d", || self.d.grab())?;

        // Allocate the polynomial result
        let poly_result = cs.alloc_input(|| "polynomial result", || self.poly_result.grab())?;

        let ax3 = cs.alloc(
            || "ax^3",
            || Ok(self.a.grab()? * self.x.grab()? * self.x.grab()? * self.x.grab()?),
        )?;

        let bx2 = cs.alloc(
            || "bx^2",
            || Ok(self.b.grab()? * self.x.grab()? * self.x.grab()?),
        )?;

        let cx = cs.alloc(|| "cx", || Ok(self.c.grab()? * self.x.grab()?))?;

        // Enforce polynomial ax³ + bx² + cx + d = result
        cs.enforce(
            || "polynomial constraint",
            |lc| lc + CS::one(),
            |lc| lc + ax3 + bx2 + cx + d,
            |lc| lc + poly_result,
        );

        // Matrix multiplication implementation
        let mut m1_vars = [[None; 2]; 2];
        let mut m2_vars = [[None; 2]; 2];
        let mut result_vars = [[None; 2]; 2];

        // Allocate matrix inputs
        for i in 0..2 {
            for j in 0..2 {
                m1_vars[i][j] = Some(cs.alloc(
                    || format!("m1[{}][{}]", i, j),
                    || self.m1.map(|m| m[i][j]).grab(),
                )?);

                m2_vars[i][j] = Some(cs.alloc(
                    || format!("m2[{}][{}]", i, j),
                    || self.m2.map(|m| m[i][j]).grab(),
                )?);

                result_vars[i][j] = Some(cs.alloc_input(
                    || format!("result[{}][{}]", i, j),
                    || self.matrix_result.map(|m| m[i][j]).grab(),
                )?);
            }
        }

        // Matrix multiplication constraints
        for i in 0..2 {
            for j in 0..2 {
                let mut sum = None;
                for k in 0..2 {
                    let product = cs.alloc(
                        || format!("prod[{}][{}][{}]", i, j, k),
                        || Ok(self.m1.grab()?[i][k] * self.m2.grab()?[k][j]),
                    )?;

                    // Enforce m1[i][k] * m2[k][j] = product
                    cs.enforce(
                        || format!("matrix mult constraint {},{},{}", i, j, k),
                        |lc| lc + m1_vars[i][k].unwrap(),
                        |lc| lc + m2_vars[k][j].unwrap(),
                        |lc| lc + product,
                    );

                    if sum.is_none() {
                        sum = Some(product);
                    } else {
                        let new_sum = cs.alloc(
                            || format!("sum[{}][{}][{}]", i, j, k),
                            || {
                                Ok(self.m1.grab()?[i][k] * self.m2.grab()?[k][j]
                                    + self.m1.grab()?[i][k - 1] * self.m2.grab()?[k - 1][j])
                            },
                        )?;

                        // Enforce sum + product = new_sum
                        cs.enforce(
                            || format!("sum constraint {},{},{}", i, j, k),
                            |lc| lc + CS::one(),
                            |lc| lc + sum.unwrap() + product,
                            |lc| lc + new_sum,
                        );

                        sum = Some(new_sum);
                    }
                }

                // Final result constraint
                cs.enforce(
                    || format!("result constraint {},{}", i, j),
                    |lc| lc + CS::one(),
                    |lc| lc + sum.unwrap(),
                    |lc| lc + result_vars[i][j].unwrap(),
                );
            }
        }

        Ok(())
    }
}

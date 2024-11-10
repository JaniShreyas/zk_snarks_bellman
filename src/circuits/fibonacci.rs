use super::common::*;

// Circuit: Fibonacci sequence
pub struct FibonacciCircuit<F: PrimeField> {
    pub f0: Option<F>,        // First term
    pub f1: Option<F>,        // Second term
    pub fn_val: Option<F>,    // Expected Fibonacci term
    pub n: usize,             // Fibonacci position
}

impl<F: PrimeField> Circuit<F> for FibonacciCircuit<F> {
    fn synthesize<CS: ConstraintSystem<F>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        // Allocate the initial variables
        let mut prev_var = cs.alloc(|| "f0", || self.f0.grab())?;
        let mut current_var = cs.alloc(|| "f1", || self.f1.grab())?;
        let fn_val = cs.alloc_input(|| "fn_val", || self.fn_val.grab())?;

        // Keep track of the actual values for allocation
        let mut prev_val = self.f0;
        let mut current_val = self.f1;

        for i in 2..=self.n {
            // Calculate next value in the sequence
            let next_val = match (prev_val, current_val) {
                (Some(p), Some(c)) => Some(p + c),
                _ => None,
            };

            // Allocate the next variable using the calculated value
            let next_var = cs.alloc(
                || format!("f{}", i),
                || next_val.grab()
            )?;
            
            // Constraint: prev + current = next
            cs.enforce(
                || format!("fibonacci constraint for term {}", i),
                |lc| lc + prev_var + current_var,
                |lc| lc + CS::one(),
                |lc| lc + next_var,
            );

            // Update both variables and values for next iteration
            prev_var = current_var;
            current_var = next_var;
            prev_val = current_val;
            current_val = next_val;
        }

        // Final constraint to check if current term equals fn_val
        cs.enforce(
            || "final result constraint",
            |lc| lc + CS::one(),
            |lc| lc + current_var,
            |lc| lc + fn_val,
        );

        Ok(())
    }
}
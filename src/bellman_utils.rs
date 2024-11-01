extern crate bellman;
extern crate bincode;
extern crate ff;
extern crate rand;

use std::time::Instant;

use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bls12_381::{Bls12, Scalar as Fr};
use ff::PrimeField;

use crate::experimentation_utils::{proof_to_bytes, write_to_csv};

pub trait OptionExt<T> {
    fn grab(&self) -> Result<T, SynthesisError>;
}

impl<T: Copy> OptionExt<T> for Option<T> {
    fn grab(&self) -> Result<T, SynthesisError> {
        self.ok_or(SynthesisError::AssignmentMissing)
    }
}

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

pub fn verify_polynomial() {
    use bellman::groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
    };
    use rand::thread_rng;

    let rng = &mut thread_rng();

    // Generate random parameters
    let params = {
        let c = PolynomialCircuit::<Fr> {
            x: None,
            y: None,
            z: None,
        };

        generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    let pvk = prepare_verifying_key(&params.vk);

    let c = PolynomialCircuit {
        x: Some(Fr::from(256)),
        y: Some(Fr::from(729)),
        z: Some(Fr::from(133264)),
    };

    // Generate proof
    let start = Instant::now();
    let proof = create_random_proof(c, &params, rng).unwrap();
    let proof_generation_time = start.elapsed();
    println!("Proof generation time: {:?}", proof_generation_time);

    // Method 1: Get raw proof size using size_of_val
    let proof_size = size_of_val(&proof);
    println!("Raw proof size: {} bytes", proof_size);

    // Method 2: Convert proof components to bytes and measure
    let proof_bytes = proof_to_bytes(&proof);
    println!("Serialized proof size: {} bytes", proof_bytes.len());

    // Print individual component sizes
    // println!("Component sizes:");
    println!("  G1 (A) size: {} bytes", proof_bytes[0..48].len());
    println!("  G2 (B) size: {} bytes", proof_bytes[48..144].len());
    println!("  G1 (C) size: {} bytes", proof_bytes[144..].len());

    // Verification remains the same...
    let start = Instant::now();
    let result = verify_proof(&pvk, &proof, &[Fr::from(133264)]);
    let proof_verification_time = start.elapsed();
    println!("Proof verification time: {:?}", proof_verification_time);
    assert!(result.is_ok());

    let num_constraints = 2; // Based on the synthesize function above
    println!("Number of constraints: {}", num_constraints);

    // Write results to CSV
    write_to_csv(
        "results.csv",
        "polynomial",
        proof_generation_time.as_secs_f64(),
        proof_size as i32,
        proof_bytes.len() as i32,
        proof_verification_time.as_secs_f64(),
        num_constraints,
    )
    .unwrap();
}

pub fn verify_complex() {
    use bellman::groth16::{
        create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
    };
    use rand::thread_rng;

    let rng = &mut thread_rng();

    // Generate random parameters
    let params = {
        let c = ComplexCircuit::<Fr> {
            x: None,
            a: None,
            b: None,
            c: None,
            d: None,
            m1: None,
            m2: None,
            poly_result: None,
            matrix_result: None,
        };

        generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    let pvk = prepare_verifying_key(&params.vk);

    let c = ComplexCircuit {
        x: Some(Fr::from(256)),
        a: Some(Fr::from(2)),
        b: Some(Fr::from(3)),
        c: Some(Fr::from(5)),
        d: Some(Fr::from(7)),
        m1: Some([[Fr::from(1), Fr::from(2)], [Fr::from(3), Fr::from(4)]]),
        m2: Some([[Fr::from(5), Fr::from(6)], [Fr::from(7), Fr::from(8)]]),
        poly_result: Some(Fr::from(33752327)),
        matrix_result: Some([[Fr::from(19), Fr::from(22)], [Fr::from(43), Fr::from(50)]]),
    };

    // Generate proof
    let start = Instant::now();
    let proof = create_random_proof(c, &params, rng).unwrap();
    let proof_generation_time = start.elapsed();
    println!("Proof generation time: {:?}", proof_generation_time);

    // Method 1: Get raw proof size using size_of_val
    let proof_size = size_of_val(&proof);
    println!("Raw proof size: {} bytes", proof_size);

    // Method 2: Convert proof components to bytes and measure
    let proof_bytes = proof_to_bytes(&proof);
    println!("Serialized proof size: {} bytes", proof_bytes.len());

    // Print individual component sizes
    // println!("Component sizes:");
    println!("  G1 (A) size: {} bytes", proof_bytes[0..48].len());
    println!("  G2 (B) size: {} bytes", proof_bytes[48..144].len());
    println!("  G1 (C) size: {} bytes", proof_bytes[144..].len());

    // Verification remains the same...
    let start = Instant::now();
    let result = verify_proof(
        &pvk,
        &proof,
        &[
            Fr::from(33752327),
            Fr::from(19),
            Fr::from(22),
            Fr::from(43),
            Fr::from(50),
        ],
    );
    let proof_verification_time = start.elapsed();
    println!("Proof verification time: {:?}", proof_verification_time);
    assert!(result.is_ok());

    let num_constraints = 2; // Based on the synthesize function above
    println!("Number of constraints: {}", num_constraints);

    // Write results to CSV
    write_to_csv(
        "results.csv",
        "complex",
        proof_generation_time.as_secs_f64(),
        proof_size as i32,
        proof_bytes.len() as i32,
        proof_verification_time.as_secs_f64(),
        num_constraints,
    )
    .unwrap();
}
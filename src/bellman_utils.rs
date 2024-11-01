use crate::circuits::{polynomial::PolynomialCircuit, matrix_multiplication::MatrixMultiplication};

use std::time::Instant;
use bls12_381::{Bls12, Scalar as Fr};


use crate::experimentation_utils::{proof_to_bytes, write_to_csv};

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
        let c = MatrixMultiplication::<Fr> {
            m1: None,
            m2: None,
            matrix_result: None,
        };

        generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    let pvk = prepare_verifying_key(&params.vk);

    let c = MatrixMultiplication {
        m1: Some([[Fr::from(1), Fr::from(2)], [Fr::from(3), Fr::from(4)]]),
        m2: Some([[Fr::from(5), Fr::from(6)], [Fr::from(7), Fr::from(8)]]),
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
            Fr::from(19),
            Fr::from(22),
            Fr::from(43),
            Fr::from(50),
        ],
    );
    let proof_verification_time = start.elapsed();
    println!("Proof verification time: {:?}", proof_verification_time);
    assert!(result.is_ok());

    let num_constraints = 20; // Based on the synthesize function above
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


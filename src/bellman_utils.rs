use crate::circuits::{
    division::DivisionCircuit, 
    matrix_multiplication::MatrixMultiplication,
    multivar_polynomial::MultiVarPolynomialCircuit,
    polynomial::PolynomialCircuit,
    xor::XorCircuit,
    fibonacci::FibonacciCircuit,
};

use bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof,
};
use bls12_381::{Bls12, Scalar as Fr};
use rand::thread_rng;
use std::time::Instant;

use crate::experimentation_utils::{proof_to_bytes, write_to_csv};

fn generate_and_verify_proof<C: bellman::Circuit<Fr>>(
    c: C,
    params: &bellman::groth16::Parameters<Bls12>,
    pvk: &bellman::groth16::PreparedVerifyingKey<Bls12>,
    public_inputs: &[Fr],
    num_constraints: usize,
    csv_label: &str,
) {
    let rng = &mut thread_rng();

    // Generate proof
    let start = Instant::now();
    let proof = create_random_proof(c, params, rng).unwrap();
    let proof_generation_time = start.elapsed();
    println!("Proof generation time: {:?}", proof_generation_time);

    // Method 1: Get raw proof size using size_of_val
    let proof_size = std::mem::size_of_val(&proof);
    println!("Raw proof size: {} bytes", proof_size);

    // Method 2: Convert proof components to bytes and measure
    let proof_bytes = proof_to_bytes(&proof);
    println!("Serialized proof size: {} bytes", proof_bytes.len());

    // Print individual component sizes
    println!("  G1 (A) size: {} bytes", proof_bytes[0..48].len());
    println!("  G2 (B) size: {} bytes", proof_bytes[48..144].len());
    println!("  G1 (C) size: {} bytes", proof_bytes[144..].len());

    // Verification remains the same...
    let start = Instant::now();
    let result = verify_proof(pvk, &proof, public_inputs);
    let proof_verification_time = start.elapsed();
    println!("Proof verification time: {:?}", proof_verification_time);
    assert!(result.is_ok());

    println!("Number of constraints: {} \n", num_constraints);

    // Write results to CSV
    write_to_csv(
        "results.csv",
        csv_label,
        proof_generation_time.as_secs_f64(),
        proof_size as i32,
        proof_bytes.len() as i32,
        proof_verification_time.as_secs_f64(),
        num_constraints as i32,
    )
    .unwrap();
}

pub fn verify_polynomial() {
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

    generate_and_verify_proof(c, &params, &pvk, &[Fr::from(133264)], 2, "polynomial");
}

pub fn verify_matrix_multiplication() {
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

    // let c = MatrixMultiplication {
    //     m1: Some([[Fr::from(1), Fr::from(2)], [Fr::from(3), Fr::from(4)]]),
    //     m2: Some([[Fr::from(5), Fr::from(6)], [Fr::from(7), Fr::from(8)]]),
    //     matrix_result: Some([[Fr::from(19), Fr::from(22)], [Fr::from(43), Fr::from(50)]]),
    // };

    let c = MatrixMultiplication {
        m1: Some([[Fr::from(123), Fr::from(456)], 
              [Fr::from(789), Fr::from(101)]]),
        m2: Some([[Fr::from(112), Fr::from(131)], 
              [Fr::from(415), Fr::from(161)]]),
        matrix_result: Some([[Fr::from(203016), Fr::from(89529)], 
            [Fr::from(130283), Fr::from(119620)]]),
    };

    generate_and_verify_proof(
        c,
        &params,
        &pvk,
        &[Fr::from(203016), Fr::from(89529), Fr::from(130283), Fr::from(119620)],
        20,
        "mat_mul",
    );
}

pub fn verify_multivar_polynomial() {
    let rng = &mut thread_rng();

    // Generate random parameters
    let params = {
        let c = MultiVarPolynomialCircuit::<Fr> {
            x: None,
            y: None,
            a: None,
            b: None,
            c: None,
            d: None,
            result: None,
        };

        generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    let pvk = prepare_verifying_key(&params.vk);

    let c = MultiVarPolynomialCircuit {
        x: Some(Fr::from(222)),
        y: Some(Fr::from(333)),
        a: Some(Fr::from(444)),
        b: Some(Fr::from(555)),
        c: Some(Fr::from(666)),
        d: Some(Fr::from(777)),
        result: Some(Fr::from(538688548680321)),
    };

    generate_and_verify_proof(
        c,
        &params,
        &pvk,
        &[Fr::from(538688548680321)],
        10,
        "multivar_poly",
    );
}

pub fn verify_division() {
    let rng = &mut thread_rng();

    // Generate random parameters
    let params = {
        let c = DivisionCircuit::<Fr> {
            numerator: None,
            denominator: None,
            quotient: None,
        };
        generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    let pvk = prepare_verifying_key(&params.vk);

    // Provide concrete values to test (e.g., 8 / 2 = 4)
    let c = DivisionCircuit {
        numerator: Some(Fr::from(4003859412)),
        denominator: Some(Fr::from(45678)),
        quotient: Some(Fr::from(87654)),
    };

    generate_and_verify_proof(
        c,
        &params,
        &pvk,
        &[Fr::from(87654)], // expected output (quotient)
        2,
        "division",
    );
}

pub fn verify_xor() {
    let rng = &mut thread_rng();

    // Generate random parameters
    let params = {
        let c = XorCircuit::<Fr> {
            a: None,
            b: None,
            result: None,
        };
        generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    let pvk = prepare_verifying_key(&params.vk);

    let c = crate::circuits::xor::XorCircuit {
        a: Some(Fr::from(1)),
        b: Some(Fr::from(0)),
        result: Some(Fr::from(1)),
    };

    generate_and_verify_proof(c, &params, &pvk, &[Fr::from(1)], 2, "xor");
}

pub fn verify_fibonacci(n: usize, expected_fn_val: u64) {
    let rng = &mut thread_rng();

    // Generate random parameters
    let params = {
        let c = FibonacciCircuit::<Fr> {
            f0: None,
            f1: None,
            fn_val: None,
            n,
        };
        generate_random_parameters::<Bls12, _, _>(c, rng).unwrap()
    };

    let pvk = prepare_verifying_key(&params.vk);

    // Initialize the circuit with initial values for f0 and f1, and the target Fibonacci value
    let c = FibonacciCircuit {
        f0: Some(Fr::from(0)),
        f1: Some(Fr::from(1)),
        fn_val: Some(Fr::from(expected_fn_val)),
        n,
    };

    // Call the general proof generation and verification function
    generate_and_verify_proof(
        c,
        &params,
        &pvk,
        &[Fr::from(expected_fn_val)], // public input is the expected nth Fibonacci number
        n, // total number of constraints is n
        "fibonacci",
    );
}

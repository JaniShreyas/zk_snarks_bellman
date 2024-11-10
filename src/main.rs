extern crate bellman;
extern crate bincode;
extern crate ff;
extern crate rand;

mod bellman_utils;
mod circuits;
mod experimentation_utils;

use bellman_utils::{
    verify_division, verify_fibonacci, verify_matrix_multiplication, verify_multivar_polynomial,
    verify_polynomial, verify_xor,
};

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    // Uncomment the function you want to run

    let iterations = 1;
    let fibo_n = 10;
    let expected_fn_val = 55;

    println!("Running tests");

    for _ in 0..iterations {
        verify_polynomial();
        verify_matrix_multiplication();
        verify_multivar_polynomial();
        verify_division();
        verify_xor();
        verify_fibonacci(fibo_n, expected_fn_val);
    }
}

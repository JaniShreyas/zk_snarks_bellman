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

    let iterations = 10;

    println!("Running tests");

    let fibo_tests = vec![
        (10, 55),
        (15, 610),
        (20, 6765),
        (25, 75025),
    ];

    for _ in 0..iterations {
        verify_polynomial();
        verify_matrix_multiplication();
        verify_multivar_polynomial();
        verify_division();
        verify_xor();

        for (n, expected) in &fibo_tests {
            verify_fibonacci(*n, *expected);
        }
    }
}

extern crate bellman;
extern crate bincode;
extern crate ff;
extern crate rand;

mod bellman_utils;
mod circuits;
mod experimentation_utils;

use bellman_utils::{
    verify_division, verify_matrix_multiplication, verify_multivar_polynomial, verify_polynomial,
};

use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // Uncomment the function you want to run
    verify_polynomial();
    verify_matrix_multiplication();
    verify_multivar_polynomial();
    verify_division();
}

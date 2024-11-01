extern crate bellman;
extern crate bincode;
extern crate ff;
extern crate rand;

mod bellman_utils;
mod experimentation_utils;
mod circuits;

use bellman_utils::{verify_matrix_multiplication, verify_polynomial, verify_multivar_polynomial};

use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // Uncomment the function you want to run
    verify_polynomial();
    verify_matrix_multiplication();
    verify_multivar_polynomial();
}
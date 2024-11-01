extern crate bellman;
extern crate bincode;
extern crate ff;
extern crate rand;

mod bellman_utils;
mod experimentation_utils;
mod circuits;

use bellman_utils::{verify_complex, verify_polynomial};
fn main() {
    // Uncomment the function you want to run
    verify_polynomial();
    verify_complex();
}
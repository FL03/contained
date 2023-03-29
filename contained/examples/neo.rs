/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained_sdk as contained;

use contained::core::{
    actors::Execute,
    turing::{instructions::Instruction, Program, Tape},
    Scope, State,
};
use contained::music::{
    neo::triads::{tonic::Tonic, Surface, Triad, TriadClass},
    Note,
};

// Test alphabet; allows programs to be written leveraging the complete 12 note alphabet
const TEST_ALPHABET: [i64; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

fn main() {
    // Initialize a new triad
    let triad = Triad::new(0.into(), TriadClass::Major);
    // Initialize a new, stateful instance
    let instance = Surface::new(triad.clone());
}

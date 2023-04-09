/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained_sdk as contained;

use contained::music::neo::triads::{Surface, Triad, TriadClass};

fn main() {
    // Initialize a new triad
    let triad = Triad::new(0.into(), TriadClass::Major);
    // Initialize a new, stateful instance
    let _instance = Surface::new(triad);
}

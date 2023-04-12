/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::prelude::Resultant;
use contained::turing::{
    instructions::{Instruction, Move},
    machine::{Driver, Machine},
    Program, State, Tape, Turing,
};

pub const TEST_ALPHABET: [&str; 3] = ["a", "b", "c"];

fn main() -> Resultant {
    let alphabet = vec!["a", "b", "c"];

    let tape = alphabet.clone();
    let scope = Driver::from(Tape::norm(tape));

    let instructions: Vec<Instruction<&str>> = vec![
        (State::default(), "a", State::default(), "c", Move::Right).into(),
        (State::default(), "b", State::default(), "a", Move::Right).into(),
        (State::default(), "c", State::invalid(), "a", Move::Stay).into(),
    ];

    // Setup the program
    let mut program = Program::new(alphabet, State::invalid());
    // Instruction set; turn ["a", "b", "c"] into ["c", "a", "a"]
    program.extend(instructions);

    let mut machine = Machine::new(scope, program);

    assert!(machine.execute().is_ok());
    assert_eq!(machine.tape().clone(), Tape::norm(["c", "a", "a"]));
    println!("{:?}", machine);

    Ok(())
}
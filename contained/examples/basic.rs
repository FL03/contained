/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::core::turing::{Instruction, Machine, Move, Operator, Program, Tape, Tapes, Turing};
use contained::core::{
    states::{State, States},
    Extend, Resultant, Scope,
};

fn main() -> Resultant {
    let alphabet = vec!["a", "b", "c"];

    let tape = alphabet.clone();
    let scope = Operator::build(Tapes::norm(tape));

    let instructions: Vec<Instruction<&str>> = vec![
        (State::default(), "c", State::default(), "a", Move::Right).into(),
        (State::default(), "b", State::default(), "a", Move::Right).into(),
        (
            State::default(),
            "c",
            States::invalid().into(),
            "a",
            Move::Stay,
        )
            .into(),
    ];

    // Setup the program
    let mut program = Program::new(alphabet, States::invalid().into());
    // Instruction set; turn ["a", "b", "c"] into ["c", "a", "a"]
    program.extend(instructions)?;

    let res = Machine::new(scope).execute(program.clone())?;
    assert_eq!(res.tape().clone(), Tape::new(vec!["c", "a", "a"]));
    println!("{:?}", res);

    Ok(())
}

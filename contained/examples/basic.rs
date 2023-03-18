/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::core::turing::{
    instructions::{Instruction, Move},
    tapes::{Tape, Tapes},
    Machine, Operator, Program, Turing,
};
use contained::core::{states::State, Extend, Resultant, Scope};

fn main() -> Resultant {
    let alphabet = vec!["a", "b", "c"];

    let tape = alphabet.clone();
    let scope = Operator::build(Tapes::norm(tape));

    let instructions: Vec<Instruction<&str>> = vec![
        (State::valid(), "c", State::valid(), "a", Move::Right).into(),
        (State::valid(), "b", State::valid(), "a", Move::Right).into(),
        (State::valid(), "c", State::invalid(), "a", Move::Stay).into(),
    ];

    // Setup the program
    let mut program = Program::new(alphabet, State::invalid());
    // Instruction set; turn ["a", "b", "c"] into ["c", "a", "a"]
    program.extend(instructions)?;

    let mut res = Machine::new(scope, program);
    res.execute()?;
    assert_eq!(res.tape().clone(), Tape::new(vec!["c", "a", "a"]));
    println!("{:?}", res);

    Ok(())
}

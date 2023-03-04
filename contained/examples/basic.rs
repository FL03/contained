/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::actors::turing::{Configuration, Machine, Move, Program, Tape, Turing};
use contained::actors::{Resultant, Scope, State, States};

fn main() -> Resultant {
    let alphabet = vec!["a", "b", "c"];

    let tape = Tape::new(alphabet.clone());
    let mut cnf = Configuration::build(tape, None);

    // Setup the program
    let mut program = Program::new(alphabet, States::invalid().into());
    // Instruction set; turn ["a", "b", "c"] into ["c", "a", "a"]
    program.insert((State::default(), "a", State::default(), "c", Move::Right).into())?;
    program.insert((State::default(), "b", State::default(), "a", Move::Right).into())?;
    program.insert(
        (
            State::default(),
            "c",
            State::from(States::invalid()),
            "a",
            Move::Left,
        )
            .into(),
    )?;

    let a = Machine::new("a", program.clone())?;
    let res = a.execute(&mut cnf)?;
    assert_eq!(res.tape().clone(), Tape::new(vec!["c", "a", "a"]));
    println!("{:?}", res);

    Ok(())
}

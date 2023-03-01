/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::actors::turing::{Configuration, Machine, Move, Program, Tape, Turing};
use contained::actors::{Resultant, State, States};

fn main() -> Resultant {
    let alphabet = vec!["a", "b", "c"];

    let tape = Tape::new(alphabet.clone());
    let mut cnf = Configuration::norm(tape)?;

    // Setup the program
    let final_state = State::from(States::invalid());
    let mut program = Program::new(alphabet, final_state);
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
    assert_eq!(res.tape().tape().clone(), vec!["c", "a", "a"]);
    println!("{:?}", res);

    Ok(())
}

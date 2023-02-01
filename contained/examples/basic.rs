/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::turing::{Configuration, Machine, Move, Program, Programatic, Tape, Turing};
use contained::{Resultant, State};

fn main() -> Resultant {
    let alphabet = vec!["a", "b", "c"];

    let tape = Tape::new(alphabet.clone());
    let mut cnf = Configuration::norm(tape)?;

    // Setup the program
    let final_state = State::new(2);
    let mut program = Program::new(alphabet, final_state);
    program.insert((1.into(), "a", 1.into(), "c", Move::Right).into())?;
    program.insert((1.into(), "b", 1.into(), "a", Move::Right).into())?;
    program.insert((1.into(), "c", 0.into(), "a", Move::Left).into())?;

    let a = Machine::new("a", program.clone())?;

    println!("{:?}", a.execute(&mut cnf)?);

    Ok(())
}

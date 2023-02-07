/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::neo::{cmp::Note, Tonnetz, Triad, Triadic};
use contained::turing::{Configuration, Machine, Move, Program, Programatic, Turing};
use contained::{Resultant, State, States};

fn main() -> Resultant {
    let triad = Triad::from((0, 3, 5));
    let _tonnetz = Tonnetz::new(triad.clone());

    let alphabet: Vec<Note> = triad.clone().into_iter().collect();
    let mut cnf: Configuration<Note> = triad.config();

    // Setup the program
    let final_state = State::from(&States::invalid());
    let mut program = Program::new(alphabet, final_state);
    // Instruction set; turn ["c", "d#", "f"] into ["d#", "d#", "d#"]
    program.insert(
        (
            State::default(),
            0.into(),
            State::default(),
            3.into(),
            Move::Right,
        )
            .into(),
    )?;
    program.insert(
        (
            State::default(),
            3.into(),
            State::default(),
            3.into(),
            Move::Right,
        )
            .into(),
    )?;
    program.insert(
        (
            State::default(),
            5.into(),
            State::from(&States::invalid()),
            3.into(),
            Move::Left,
        )
            .into(),
    )?;

    let a = Machine::new(0.into(), program.clone())?;
    let res = a.execute(&mut cnf)?;
    println!("{:?}", res);

    Ok(())
}

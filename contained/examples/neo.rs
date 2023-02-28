/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::actors::turing::{Configuration, Program, Programatic, Turing};
use contained::actors::{Resultant, State, States};
use contained::{
    core::Note,
    neo::{Triad, Triads},
};

fn main() -> Resultant {
    let triad = Triad::new(0.into(), Triads::Diminshed);

    let alphabet: Vec<Note> = triad.clone().into_iter().collect();
    let mut cnf: Configuration<Note> = triad.config();

    // Setup the program
    let final_state = State::from(&States::invalid());
    let mut program = Program::new(alphabet, final_state);
    // Instruction set; turn ["C", "D#", "F#"] into ["F#", "D#", "D#"]
    program.insert(
        (
            State::default(),
            0.into(),
            State::default(),
            6.into(),
            1.into(),
        )
            .into(),
    )?;
    program.insert(
        (
            State::default(),
            3.into(),
            State::default(),
            3.into(),
            1.into(),
        )
            .into(),
    )?;
    program.insert(
        (
            State::default(),
            6.into(),
            State::from(&States::invalid()),
            3.into(),
            2.into(),
        )
            .into(),
    )?;

    let res = triad.machine(program)?.execute(&mut cnf)?;
    println!("{:?}", res);

    Ok(())
}

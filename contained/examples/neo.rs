/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::core::states::{State, States};
use contained::core::turing::{Instruction, Program, Turing};
use contained::core::{Extend, Resultant};
use contained::music::{
    neo::{Triad, Triads},
    Note,
};

fn main() -> Resultant {
    let triad = Triad::new(0.into(), Triads::Diminshed);
    let alphabet: Vec<Note> = triad.clone().into_iter().collect();

    let instructions: Vec<Instruction<Note>> = vec![
        (
            State::default(),
            0.into(),
            State::default(),
            6.into(),
            1.into(),
        )
            .into(),
        (
            State::default(),
            3.into(),
            State::default(),
            3.into(),
            1.into(),
        )
            .into(),
        (
            State::default(),
            6.into(),
            States::invalid().into(),
            3.into(),
            2.into(),
        )
            .into(),
    ];

    // Setup the program
    let mut program = Program::new(alphabet, States::Invalid.into());

    // Instruction set; turn ["C", "D#", "F#"] into ["F#", "D#", "D#"]
    program.extend(instructions)?;

    let res = triad.machine().execute(program)?;
    println!("{:?}", res);

    Ok(())
}

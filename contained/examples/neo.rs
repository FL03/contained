/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::core::states::State;
use contained::core::turing::{instructions::Instruction, Tape, Turing};
use contained::core::{Extend, Resultant};
use contained::music::{
    neo::triads::{Triad, Triads},
    Note,
};

fn main() -> Resultant {
    tracing_subscriber::fmt::init();
    // Setup the triad
    let triad = Triad::new(0.into(), Triads::Diminished);
    //
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
            State::invalid(),
            3.into(),
            2.into(),
        )
            .into(),
    ];
    tracing::info!("Instructions: \n{:?}", instructions.clone());
    // Initialize a new machine
    let mut machine = triad.clone().machine(Some(Tape::new(triad)));
    // Extend the program; turn [0, 3, 6] into [6, 3, 3]
    machine.program.extend(instructions)?;
    tracing::info!("Success: inserted the instructions into the machine...");
    // Execute the program
    tracing::info!("Executing the program...");
    println!("{:?}", machine.execute()?);

    Ok(())
}

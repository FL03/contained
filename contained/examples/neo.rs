/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::core::turing::{instructions::Instruction, Tape};
use contained::core::{actors::Execute, states::State, Resultant};
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
    let mut machine = triad
        .clone()
        .actor(Some(Tape::norm([0.into(), 3.into(), 6.into()])));
    // Extend the program; turn [0, 3, 6] into [6, 3, 3]
    machine.extend(instructions);
    tracing::info!("Success: inserted the instructions into the machine...");
    // Execute the program
    tracing::info!("Executing the program...");
    assert!(machine.execute().is_ok());
    println!("{:?}", machine.memory.as_ref());

    Ok(())
}

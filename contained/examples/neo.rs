/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained_sdk;

use contained_sdk::core::{
    actors::Execute,
    turing::{instructions::Instruction, Tape},
    State,
};
use contained_sdk::music::{
    neo::triads::{Instance, Triad, Triads},
    Note,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let tape: Tape<Note> = Tape::norm([0.into(), 3.into(), 6.into()]);
    // Setup the triad
    let triad = Triad::new(0.into(), Triads::Diminished);
    // Initialize a new instance
    let instance = Instance::new(triad.clone());
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
    let mut actor = instance.actor(Some(tape));
    // Extend the program; turn [0, 3, 6] into [6, 3, 3]
    actor.extend(instructions);
    // Execute the program; assert that the program executed successfully
    assert!(actor.execute().is_ok());
    // Print the machine memory
    println!("{:?}", actor.memory.as_ref());

    Ok(())
}

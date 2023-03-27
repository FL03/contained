/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained_sdk as contained;

use contained::core::{
    actors::Execute,
    turing::{instructions::Instruction, Program, Tape},
    Scope, State,
};
use contained::music::{
    neo::triads::{tonic::Tonic, Surface, Triad, Triads},
    Note,
};

// Test alphabet; allows programs to be written leveraging the complete 12 note alphabet
const TEST_ALPHABET: [i64; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

fn main() {
    // Initialize a new triad
    let triad = Triad::new(0.into(), Triads::Major);
    // Initialize a new, stateful instance
    let instance = Surface::new(triad.clone());
    // Initialize a new tape
    let tape: Tape<Note> = Tape::norm([0.into(), 1.into(), 3.into()]);

    // Instruction set; turn ["a", "b", "c"] into ["c", "a", "a"]
    let instructions: Vec<Instruction<Note>> = vec![
        (
            State::default(),
            0.into(),
            State::default(),
            7.into(),
            1.into(),
        )
            .into(),
        (
            State::default(),
            1.into(),
            State::default(),
            4.into(),
            1.into(),
        )
            .into(),
        (
            State::default(),
            3.into(),
            State::invalid(),
            0.into(),
            0.into(),
        )
            .into(),
    ];
    // Setup the program
    let program = Program::new(
        TEST_ALPHABET.iter().map(Note::from).collect::<Vec<_>>(),
        State::invalid(),
    );
    // Initialize a new machine
    let mut tonic = Tonic::new(program, instance);
    // Extend the machine memory; insert [0, 1, 4] into the tape
    tonic.extend(tape);
    // Extend the program; turn [0, 1, 4] into [7, 4, 0]
    tonic.extend(instructions);
    // Execute the program
    assert!(tonic.execute().is_ok());
    // Assert the result
    assert_eq!(tonic.tape(), Tape::norm([7.into(), 4.into(), 0.into()]));
    // Assert that the current triad is not the same as the original
    // assert_ne!(tonic.surface().lock().unwrap().triad(), triad);
}

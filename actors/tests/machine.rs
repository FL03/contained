#[cfg(test)]
use contained_actors::turing::{
    Instruction, Machine, Move, Operator, Program, Tape, Tapes, Turing,
};
use contained_actors::{
    states::{State, States},
    Extend, Scope,
};

pub const TEST_ALPHABET: [&str; 3] = ["a", "b", "c"];

#[test]
fn test_machine() {
    let alphabet = vec!["a", "b", "c"];

    let tape = Tape::new(alphabet.clone());
    let scope = Operator::build(Tapes::normal(tape));

    let instructions: Vec<Instruction<&str>> = vec![
        (State::default(), "a", State::default(), "c", Move::Right).into(),
        (State::default(), "b", State::default(), "a", Move::Right).into(),
        (
            State::default(),
            "c",
            States::invalid().into(),
            "a",
            Move::Stay,
        )
            .into(),
    ];

    // Setup the program
    let mut program = Program::new(alphabet, States::invalid().into());
    // Instruction set; turn ["a", "b", "c"] into ["c", "a", "a"]
    program.extend(instructions).unwrap();

    let res = Machine::new(scope).execute(program.clone());

    assert!(res.is_ok());
    assert_eq!(res.unwrap().tape().clone(), Tape::new(["c", "a", "a"]));
}

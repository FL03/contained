#[cfg(test)]
use contained_actors::turing::{Operator, Machine, Move, Program, Tape, Tapes, Turing};
use contained_actors::{Scope, State, States};

pub const TEST_ALPHABET: [&str; 3] = ["a", "b", "c"];

#[test]
fn test_machine() {
    let alphabet = vec!["a", "b", "c"];

    let tape = Tape::new(TEST_ALPHABET);
    let cnf = Operator::build(Tapes::normal(tape));

    // Setup the program
    let final_state = State::from(States::invalid());
    let mut program = Program::new(alphabet, final_state);
    // Instruction set; turn ["a", "b", "c"] into ["c", "a", "a"]
    program
        .insert((State::default(), "a", State::default(), "c", Move::Right).into())
        .unwrap();
    program
        .insert((State::default(), "b", State::default(), "a", Move::Right).into())
        .unwrap();
    program
        .insert(
            (
                State::default(),
                "c",
                State::from(States::invalid()),
                "a",
                Move::Left,
            )
                .into(),
        )
        .unwrap();
    let mut a = Machine::new(cnf.clone());

    let res = a.execute(program.clone());
    assert!(res.is_ok());
    assert_eq!(res.unwrap().tape().clone(), Tape::new(["c", "a", "a"]));
}

use contained::turing::{Configuration, Machine, Move, Program, Programatic, Tape, Turing};
#[cfg(test)]
use contained::{State, States};

#[test]
fn test_machine() {
    let alphabet = vec!["a", "b", "c"];

    let tape = Tape::new(alphabet.clone());
    let cnf = Configuration::norm(tape);

    // Setup the program
    let final_state = State::from(&States::invalid());
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
                State::from(&States::invalid()),
                "a",
                Move::Left,
            )
                .into(),
        )
        .unwrap();
    let a = Machine::new("a", program.clone());
    assert!(a.is_ok());

    assert!(Machine::new("", program.clone()).is_err());
    let res = a.unwrap().execute(&mut cnf.unwrap());
    assert!(res.is_ok());
    assert_eq!(res.unwrap().tape().tape().clone(), vec!["c", "a", "a"]);
}

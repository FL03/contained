/*
    Appellation: basic <example>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
extern crate contained;

use contained::turing::{Configuration, Instruction, Machine, Move, Program, Programatic, Tape, Turing};

fn main() -> Result<(), String> {
    let alphabet = vec!["a", "b", "c"];

    let tape = Tape::new(alphabet.clone());
    let mut cnf = Configuration::norm(tape)?;

    let inst = Instruction::from((1.into(), "a", 0.into(), "b", Move::Right));
    let mut program = Program::new(alphabet, 2.into());
    program.insert(inst.clone()).unwrap();

    let a = Machine::new("b", program.clone())?;


    println!("{:?}", a.execute(&mut cnf)?);

    Ok(())
}

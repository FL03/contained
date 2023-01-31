/*
    Appellation: turing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
pub use self::{configuration::*, instructions::*, machine::*, programs::*, tapes::*};

pub(crate) mod configuration;
pub(crate) mod instructions;
pub(crate) mod machine;
pub(crate) mod programs;
pub(crate) mod tapes;

pub type Dirac<S, T> = dyn Fn(S) -> T;

pub trait Symbolic: Clone + Default + Eq + PartialEq + ToString {}

impl Symbolic for char {}

impl Symbolic for &str {}

impl Symbolic for String {}

pub trait Transition<S: Clone> {
    type Output;

    fn data(&self) -> &S;
    fn dirac(&self) -> &Dirac<S, Self::Output>;
    fn resultant(&self) -> Self::Output {
        self.dirac()(self.data().clone())
    }
}

pub trait Turing {
    type Symbol: Symbolic;
    ///
    fn execute(
        &self,
        cnf: &mut Configuration<Self::Symbol>,
    ) -> crate::Resultant<Configuration<Self::Symbol>> {
        self.execute_until(cnf, |cnf| cnf.state == 0.into())
    }
    ///
    fn execute_once(
        &self,
        cnf: &mut Configuration<Self::Symbol>,
    ) -> crate::Resultant<Configuration<Self::Symbol>>;
    ///
    fn execute_until(
        &self,
        cnf: &mut Configuration<Self::Symbol>,
        until: impl Fn(&Configuration<Self::Symbol>) -> bool,
    ) -> crate::Resultant<Configuration<Self::Symbol>>;
    /// Translates and returns a mutated [`Tape`] using the [`TuringMachine::execute`]
    /// method as the [`Configuration::new_std`].
    fn translate_std(&self, tape: Tape<Self::Symbol>) -> Result<Tape<Self::Symbol>, String> {
        let mut conf = Configuration::std(tape)?;
        let exec = self.execute(&mut conf)?;
        Ok(exec.tape())
    }

    /// Translates and returns a mutated [`Tape`] using the [`TuringMachine::execute`]
    /// method as the [`Configuration::new_nrm`].
    fn translate_nrm(&self, tape: Tape<Self::Symbol>) -> Result<Tape<Self::Symbol>, String> {
        let mut conf = Configuration::norm(tape)?;
        let exec = self.execute(&mut conf)?;
        Ok(exec.tape())
    }
}

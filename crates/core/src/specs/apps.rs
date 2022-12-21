/*
    Appellation: apps <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::Locked;

use scsys::BoxResult;

pub trait AppSpec: Default {
    type Cnf;
    type Ctx;
    type State;
    fn init() -> Self;
    fn context(&self) -> Self::Ctx;
    fn name(&self) -> String;
    fn settings(&self) -> Self::Cnf;
    fn setup(&mut self) -> BoxResult<&Self>;
    fn slug(&self) -> String {
        self.name().to_ascii_lowercase()
    }
    fn state(&self) -> &Locked<Self::State>;
}

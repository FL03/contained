/*
    Appellation: compute <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: This module implements a basic framework for building dynamic, multiway systems
*/

pub mod surface;

use crate::Error;

pub trait Compute {
    type Output;

    fn compute(&self) -> Result<Self::Output, Error>;
}

/*
    Appellation: reqres <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

pub use self::{request::*, response::*};

mod request;
mod response;



pub enum Operation {
    Add,
    Get,
    Remove,
    Run,
    Update
}
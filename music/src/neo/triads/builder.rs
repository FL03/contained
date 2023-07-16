/*
    Appellation: builder <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! Triad Builder
//! 


use super::{Triad, Triads};
use crate::prelude::{Interval, Note};

/// [TriadBuilder] is a simple struct that allows for the construction of a [Triad] from a [Note] and an [Interval].
pub struct TriadBuilder {
    notes: [Note; 3],
    root: Note,
    interval: Interval,
}

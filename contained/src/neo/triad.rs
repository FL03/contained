/*
    Appellation: triad <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        def. A triad is a set of three notes
*/
use crate::neo::{Note, Pitch};
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Triad<I: Pitch, J: Pitch, K: Pitch>(Note<I>, Note<J>, Note<K>);

/*
    Appellation: atable <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: an adjacency table
*/
use super::Node;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AdjacencyTable<N: Node, V>(HashMap<N, Vec<(N, V)>>);

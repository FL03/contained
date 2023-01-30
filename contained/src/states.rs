/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use decanter::prelude::{Hash, Hashable};
use scsys::prelude::{fnl_remove, StatePack};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States {
    Invalid = 0,
    #[default]
    Valid = 1,
}

impl States {
    pub fn invalid() -> Self {
        Self::Invalid
    }
    pub fn valid() -> Self {
        Self::Valid
    }
}

impl StatePack for States {}

impl std::fmt::Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            fnl_remove(serde_json::to_string(&self).unwrap()).to_ascii_lowercase()
        )
    }
}

impl From<i64> for States {
    fn from(val: i64) -> States {
        match val {
            0 => States::Invalid,
            1 => States::Valid,
            _ => States::Invalid,
        }
    }
}

impl From<States> for i64 {
    fn from(val: States) -> i64 {
        val as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use scsys::prelude::{State, Stateful, StatefulExt};

    #[test]
    fn test_default_state() {
        let a = State::<States>::default();
        let mut b = a.clone();

        assert_eq!(&a, &b);
        assert_eq!(a.state() as i64, 1);

        b.update_state(None, States::Invalid);
        assert_eq!(b.state(), States::Invalid)
    }
}

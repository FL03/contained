/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::{fnl_remove, StatePack};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(
    Clone, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum States {
    Error = 0,
    Idle = 1,
    Complete = 2,
    Derive = 3,
    Process = 4,
    Request = 5,
    Response = 6,
}

impl States {
    pub fn idle() -> Self {
        Self::Idle
    }
}

impl StatePack for States {}

impl Default for States {
    fn default() -> Self {
        Self::idle()
    }
}

impl std::fmt::Display for States {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            fnl_remove(serde_json::to_string(&self).unwrap()).to_ascii_lowercase()
        )
    }
}

impl From<States> for i64 {
    fn from(val: States) -> Self {
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

        b.update_state(None, States::Complete);
        assert_eq!(b.state(), States::Complete)
    }
}

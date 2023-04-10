/*
    Appellation: surface <triads>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Generically, a surface describes a type of topological compute surface. Here we implement a surface for triads, which are the fundamental unit of computation in contained.
*/
use super::*;
use crate::neo::{Transform, LPR};
use contained_core::{turing::State, Shared};
use decanter::prelude::Hashable;
use futures::Future;
use scsys::prelude::AsyncResult;
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::Poll,
};

#[derive(Clone, Debug, Default, Hashable)]
pub struct Surface {
    state: State,
    triad: Triad,
}

impl Surface {
    pub fn new(triad: Triad) -> Self {
        Self {
            state: State::default(),
            triad,
        }
    }
    pub fn state(&self) -> State {
        self.state
    }
    pub async fn transform(&mut self, lpr: LPR) -> AsyncResult<Triad> {
        match self.clone().await.state() {
            State::Invalid => Err("Invalid state".into()),
            State::Valid => {
                let next = self.triad.transform(lpr);
                self.triad = next.clone();
                Ok(next)
            }
        }
    }
    pub fn triad(&self) -> &Triad {
        &self.triad
    }
}

impl Future for Surface {
    type Output = Self;

    fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        if self.state() == State::Invalid {
            cx.waker().wake_by_ref();
            return Poll::Pending;
        } else {
            return Poll::Ready(self.clone());
        }
    }
}

impl std::fmt::Display for Surface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = serde_json::json!(
            {
                "state": self.state.to_string(),
                "triad": self.triad.to_string(),
            }
        );
        write!(f, "{}", msg)
    }
}

/*
    Appellation: frequency <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin, task::Poll, thread, time::Duration};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Frequency(Duration);

impl Frequency {
    pub fn new(duration: Duration) -> Self {
        Self(duration)
    }
    pub fn adjust(&mut self, sec: u64, nano: u32) {
        self.0 = Duration::new(sec, nano);
    }
    pub fn freq(&self) -> f64 {
        1.0 / self.0.as_secs_f64()
    }
    pub fn period(&self) -> Duration {
        self.0.clone()
    }
}

impl Default for Frequency {
    fn default() -> Self {
        Self(Duration::new(1, 0))
    }
}

impl Future for Frequency {
    type Output = Self;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // Get a handle to the waker for the current task
        let waker = cx.waker().clone();
        let freq = self.period();
        // Spawn a timer thread.
        thread::spawn(move || {
            thread::sleep(freq);

            waker.wake();
        });

        Poll::Pending
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Wave {
    amplitude: f64,
    freq: Frequency,
    ts: i64,
}

impl Future for Wave {
    type Output = Self;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // Get a handle to the waker for the current task
        let waker = cx.waker().clone();
        let freq = self.freq.clone();
        // Spawn a timer thread.
        thread::spawn(move || {
            thread::sleep(freq.period());

            waker.wake();
        });

        Poll::Pending
    }
}

impl Stream for Wave {
    type Item = ();

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        if self.amplitude == 0.0 {
            // No more delays
            return Poll::Ready(None);
        }

        match Pin::new(&mut self.freq).poll(cx) {
            Poll::Ready(_) => {
                // TODO: Create a means of introducing a frequency dampener into the wave
                self.amplitude -= 1.0;
                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

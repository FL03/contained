/*
    Appellation: frequency <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll,};
use std::time::{Duration, Instant};
use std::{future::Future, pin::Pin, thread};

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Frequency {
    // The current count; the number of cycles completed.
    cursor: usize,
    // The period of a frequency; the time it takes to complete a cycle.
    period: Duration,
}

impl Frequency {
    pub fn new(period: Duration) -> Self {
        Self { cursor: 0, period }
    }
    /// Adjusts the frequency.
    pub fn adjust(&mut self, sec: u64, nano: u32) {
        self.period = Duration::new(sec, nano);
    }
    pub fn cursor(&self) -> usize {
        self.cursor
    }
    /// Returns the current frequency.
    pub fn freq(&self) -> f64 {
        1.0 / self.period.as_secs_f64()
    }
    /// Returns the current period.
    pub fn period(&self) -> Duration {
        self.period.clone()
    }
}

impl Default for Frequency {
    fn default() -> Self {
        let period = Duration::new(1, 0);
        Self::new(period)
    }
}

impl Future for Frequency {
    type Output = Self;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let now = Instant::now();
        // Get a handle to the waker for the current task
        let waker = cx.waker().clone();
        let period = self.period.clone();
        // Spawn a timer thread.
        thread::spawn(move || {
            thread::sleep(period);

            waker.wake();
        });
        if Instant::now() >= now {
            self.cursor += 1;
            Poll::Ready(self.clone())
        } else {
            Poll::Pending
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_frequency() {
        let freq = Frequency::new(Duration::new(1, 0));
        assert_eq!(freq, Default::default());
    }
}

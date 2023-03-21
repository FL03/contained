/*
    Appellation: frequency <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use futures::Stream;
use serde::{Deserialize, Serialize};

use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant};
use std::{future::Future, pin::Pin, thread};
use tokio::sync::Notify;

pub async fn delay(dur: Duration) {
    let when = Instant::now() + dur;
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    thread::spawn(move || {
        let now = Instant::now();

        if now < when {
            thread::sleep(when - now);
        }

        notify2.notify_one();
    });

    notify.notified().await;
}

/// The `Delay` future represents an asynchronous sleep.
#[derive(Clone, Debug)]
pub struct Delay {
    // This is Some when we have spawned a thread, and None otherwise.
    waker: Option<Arc<Mutex<Waker>>>,
    // The `Instant` at which the delay will complete.
    when: Instant,
}

impl Delay {
    pub fn new(when: Instant) -> Self {
        Self { waker: None, when }
    }
    /// Adjusts the delay to be `duration` earlier.
    pub fn decrease(&mut self, duration: Duration) {
        self.when -= duration;
    }
    /// Adjusts the delay to be `duration` later.
    pub fn increase(&mut self, duration: Duration) {
        self.when += duration;
    }
    /// Returns the `Waker` that will be notified when the delay completes.
    pub fn waker(&self) -> Option<Arc<Mutex<Waker>>> {
        self.waker.clone()
    }
    /// Returns the `Instant` at which the delay will complete.
    pub fn when(&self) -> Instant {
        self.when.clone()
    }
}

impl Default for Delay {
    fn default() -> Self {
        Self::new(Instant::now())
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // First, if this is the first time the future is called, spawn the
        // timer thread. If the timer thread is already running, ensure the
        // stored `Waker` matches the current task's waker.
        if let Some(waker) = &self.waker {
            let mut waker = waker.lock().unwrap();

            // Check if the stored waker matches the current task's waker.
            // This is necessary as the `Delay` future instance may move to
            // a different task between calls to `poll`. If this happens, the
            // waker contained by the given `Context` will differ and we
            // must update our stored waker to reflect this change.
            if !waker.will_wake(cx.waker()) {
                *waker = cx.waker().clone();
            }
        } else {
            let when = self.when;
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());

            // This is the first time `poll` is called, spawn the timer thread.
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                // The duration has elapsed. Notify the caller by invoking
                // the waker.
                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }

        // Once the waker is stored and the timer thread is started, it is
        // time to check if the delay has completed. This is done by
        // checking the current instant. If the duration has elapsed, then
        // the future has completed and `Poll::Ready` is returned.
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            // The duration has not elapsed, the future has not completed so
            // return `Poll::Pending`.
            //
            // The `Future` trait contract requires that when `Pending` is
            // returned, the future ensures that the given waker is signalled
            // once the future should be polled again. In our case, by
            // returning `Pending` here, we are promising that we will
            // invoke the given waker included in the `Context` argument
            // once the requested duration has elapsed. We ensure this by
            // spawning the timer thread above.
            //
            // If we forget to invoke the waker, the task will hang
            // indefinitely.
            Poll::Pending
        }
    }
}

impl PartialEq for Delay {
    fn eq(&self, other: &Self) -> bool {
        self.when == other.when
    }
}

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
        // Stores the current cycle count.
        let cycle = self.cursor;
        // Increment the cycle count.
        self.cursor += 1;
        // Stores the current period
        let period = self.period();
        // Find the time at which the next cycle will complete.
        let when = Instant::now() + Duration::from_secs_f64(self.freq());
        // Clone a thread-safe waker for use in the timer thread.
        let waker = Arc::new(Mutex::new(cx.waker().clone()));

        // Spawn a new timer thread that will notify the current task when compelte
        thread::spawn(move || {
            let now = Instant::now();

            if now < when {
                thread::sleep(when - now);
            }
            //
            waker.lock().unwrap().wake_by_ref();
        });

        if Instant::now() >= when {
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
    async fn test_delay() {
        assert!(Delay::default().waker.is_none());

        let start = Instant::now();
        let dur = Duration::new(1, 0);

        let mut delay = Delay::new(start.clone());
        delay.increase(dur.clone());
        assert_eq!(delay.when, start + dur);
    }

    #[tokio::test]
    async fn test_frequency() {
        let freq = Frequency::new(Duration::new(1, 0));
        assert_eq!(freq, Default::default());
    }
}

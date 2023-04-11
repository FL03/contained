/*
    Appellation: delay <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
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
        self.when
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_delay() {
        assert!(Delay::default().waker.is_none());

        let start = Instant::now();
        let dur = Duration::new(1, 0);

        let mut delay = Delay::new(start);
        delay.increase(dur);
        assert_eq!(delay.when, start + dur);
    }
}

/*
    Appellation: epoch <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use decanter::prelude::Hashable;
use scsys::prelude::{SerdeDisplay, Timestamp};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub enum EpochPerspective {
    Before,
    During,
    After,
}

/// An [Epoch] consists of a start time and optionally, a duration (seconds). If None, the system assumes an infinite duration
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Hashable,
    Ord,
    PartialEq,
    PartialOrd,
    SerdeDisplay,
    Serialize,
)]
pub struct Epoch {
    pub duration: Duration,
    pub start: i64,
}

impl Epoch {
    pub fn new(duration: Option<Duration>, start: impl Into<i64>) -> Self {
        Self {
            duration: duration.unwrap_or_else(|| Duration::from_secs(1)),
            start: start.into(),
        }
    }
    pub fn duration(&self) -> Duration {
        self.duration
    }
    /// Returns the [Duration] of time since the [Epoch] was created
    pub fn elapsed(&self) -> Duration {
        let now: i64 = Timestamp::default().into();
        Duration::from_secs((now - self.start) as u64)
    }
    /// Returns the end time of the [Epoch]
    pub fn end(&self) -> i64 {
        self.start + self.duration.as_secs() as i64
    }
    /// Returns true if the epoch has expired
    pub fn is_expired(&self) -> bool {
        self.elapsed() > self.duration
    }
    /// Returns the start time of the [Epoch]
    pub fn start(&self) -> i64 {
        self.start
    }
    pub fn tstep(&self, n: usize) -> Vec<i64> {
        let step_size = self.duration.div_f64(n as f64).as_secs() as i64;
        (0..n)
            .map(|i| self.start + (i as i64 * step_size))
            .collect()
    }
}

impl Default for Epoch {
    fn default() -> Self {
        Self::new(Some(Duration::new(1, 0)), Timestamp::default())
    }
}

impl From<Duration> for Epoch {
    fn from(duration: Duration) -> Self {
        Self::new(Some(duration), Timestamp::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_epoch() {
        let epoch = Epoch::from(Duration::new(2, 0));
        assert_eq!(epoch.duration(), Duration::new(2, 0));
        assert_eq!(
            epoch.end(),
            epoch.start() + epoch.duration().as_secs() as i64
        );
        assert_eq!(epoch.is_expired(), false);
        assert_eq!(epoch.tstep(1), vec![epoch.start()]);
        assert_eq!(
            epoch.tstep(2),
            vec![
                epoch.start(),
                epoch.start() + epoch.duration().div_f64(2.0).as_secs() as i64
            ]
        );
    }
}

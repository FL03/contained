/*
    Appellation: frequency <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: Frequency is the number of occurrences of a repeating event or signal per unit time
*/
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Frequency {
    pub cycle: usize,
    pub period: Duration,
}

impl Frequency {
    pub fn new(period: Duration) -> Self {
        Self { cycle: 0, period }
    }
    pub fn freq(&self) -> Duration {
        Duration::new(1, 0).div_f64(self.period.as_secs_f64())
    }
    pub fn interval(&self) -> time::Interval {
        time::interval(self.freq())
    }
    pub fn period(&self) -> Duration {
        self.period
    }
}

impl Default for Frequency {
    fn default() -> Self {
        Self::new(Duration::new(1, 0))
    }
}

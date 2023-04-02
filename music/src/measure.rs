/*
    Appellation: measure <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: In music theory, a measure (or bar) refers to a single unit of time featuring a specific number of beats played at a particular tempo
*/
use tokio::time::Interval;

pub struct Measure {
    
    interval: Interval,
}

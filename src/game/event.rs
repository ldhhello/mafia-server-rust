use super::time::Time;

pub enum Event {
    Start,
    TimeChanged(Time),
}
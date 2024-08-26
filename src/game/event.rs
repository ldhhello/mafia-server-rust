use super::time::Time;

pub enum Event {
    Start,
    TimeChanged(Time),
    IncreaseTime(usize),
    DecreaseTime(usize),
    Chat(usize, String),
    Hand(usize, usize),
    Skill(i32, Vec<String>, usize)
}

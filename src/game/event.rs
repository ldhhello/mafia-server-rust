use super::time::Time;
use super::GameString;

pub enum Event {
    Start,
    TimeChanged(Time),
    IncreaseTime(usize),
    DecreaseTime(usize),
    Chat(usize, String),
    Hand(usize, usize),
    Skill(i32, Vec<GameString>, usize),
    Memo(usize, usize),
}

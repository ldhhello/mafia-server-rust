use super::time::Time;

#[derive(Clone, Copy, PartialEq)]
pub enum LifeStatus {
    Alive,
    Dead,
    Seongbul
}

#[derive(Clone)]
pub struct Status {
    pub modified_time: bool,
    pub life_status: LifeStatus,
    pub connected: bool, /* 보조 직업의 경우 마피아와 접선했는지 여부, 간호사의 경우 의사와 접선했는지 여부 */
    pub lynched: bool,
    pub hand: usize
}

impl Status {
    pub fn new() -> Self {
        Self {
            modified_time: false,
            life_status: LifeStatus::Alive,
            connected: false,
            lynched: false,
            hand: usize::max_value(),
        }
    }
    pub fn reset(&mut self, time: Time) {
        self.hand = usize::max_value();
        match time {
            Time::Null => (),
            Time::Night => {
                self.lynched = false;
            },
            Time::Day => {
                self.modified_time = false;
            },
            Time::Vote => {},
            Time::FinalObjection => (),
            Time::YesnoVote => (),
            Time::End => ()
        }
    }
}

impl Default for Status {
    fn default() -> Self {
        Self::new()
    }
}

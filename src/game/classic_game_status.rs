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
    pub connected: bool /* 보조 직업의 경우 마피아와 접선했는지 여부, 간호사의 경우 의사와 접선했는지 여부 */
}

impl Status {
    pub fn new() -> Self {
        Self {
            modified_time: false,
            life_status: LifeStatus::Alive,
            connected: false
        }
    }
    pub fn reset(&mut self, time: Time) {
        match time {
            Time::Null => (),
            Time::Night => (),
            Time::Day => {
                self.modified_time = false;
            },
            Time::Vote => (),
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

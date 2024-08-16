use super::{JobList, Team};

pub enum HandType {
    NoHand,
    FixedHand,
    MovingHand
}

pub struct JobOption {
    pub job_id: JobList,
    pub team: Team,
    pub hand_type: HandType,
    pub vote_right: i32,
    
}

pub struct JobStatus {
    
}
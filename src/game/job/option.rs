use super::{JobList, Team};

pub enum HandType {
    NoHand,
    FixedHand,
    MovingHand,
    MafiaHand, /* 마피아는 총구가 공유되기 때문에 따로 처리함 */
}

pub struct JobOption {
    pub job_id: JobList,
    pub team: Team,
    pub hand_type: HandType,
    pub vote_right: i32,
    pub shared_hand: bool
}

pub struct JobStatus {

}

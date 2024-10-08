use crate::{game::{self, classic_game_status::Status, job::{default_chat::chat_default, job::ChatFn}}, room::room::ChatType};

use crate::game::{time::Time, event::Event};

use super::super::{job::Job, option::{HandType, JobOption}, JobList, Team};

pub struct Citizen;

impl Citizen {
    pub fn new() -> Self { Self {} }
}

impl Job for Citizen {
    fn option(&self) -> JobOption {
        JobOption {
            job_id: JobList::Citizen,
            team: Team::CitizenTeam,
            hand_type: HandType::NoHand,
            vote_right: 1,
            shared_hand: false,
        }
    }

    fn is_valid_hand(&self, time: Time, job: &Box<dyn Job + Send>, status: &Status, idx: usize) -> bool {
        return false;
    }

    fn hand(&self, time: Time, job: &Box<dyn Job + Send>, status: &Status, my_idx: usize, target_idx: usize) -> Vec<Event> {
        vec![]
    }

    fn on_got_murderred(&self, players: &Vec<Box<dyn Job>>, idx: usize) -> Vec<Event> {
        todo!()
    }

    fn chat(&self, time: Time, my_status: &Status) -> ChatFn {
        if let Some(chat_fn) = chat_default(time, my_status) {
            return chat_fn;
        }

        Box::new(|_, _| {
            ChatType::Null
        })
    }
}

use crate::{game::{self, classic_game_status::{LifeStatus, Status}, job::{default_chat::chat_default, job::ChatFn}}, room::room::ChatType};

use crate::game::time::Time;

use super::{super::{job::Job, option::{HandType, JobOption}, JobList, Team}, chat_mafia};

pub struct Mafia;

impl Mafia {
    pub fn new() -> Self { Self {} }
}

impl Job for Mafia {
    fn option(&self) -> JobOption {
        JobOption {
            job_id: JobList::Mafia,
            team: Team::MafiaTeam,
            hand_type: HandType::MovingHand,
            vote_right: 1,
            shared_hand: true,
        }
    }

    fn is_valid_hand(&self, time: Time, job: &Box<dyn Job + Send>, status: &Status, idx: usize) -> bool {
        if time != Time::Night {
            return false;
        }
        return status.life_status == LifeStatus::Alive;
    }

    // 마피아의 능력은 특별히 여기서 처리하지 않고 classic_game.rs에서 처리한다.
    fn hand(&self, time: Time, job: &Box<dyn Job + Send>, status: &Status, my_idx: usize, target_idx: usize) -> Vec<game::event::Event> {
        vec![]
    }

    fn on_got_murderred(&self, players: &Vec<Box<dyn Job>>, idx: usize) -> Vec<game::event::Event> {
        todo!()
    }

    fn chat(&self, time: Time, my_status: &Status) -> ChatFn {
        if let Some(chat_fn) = chat_default(time, my_status) {
            return chat_fn;
        }

        if let Some(chat_fn) = chat_mafia(time, my_status) {
            return chat_fn;
        }

        Box::new(|_, _| {
            ChatType::Null
        })
    }
}

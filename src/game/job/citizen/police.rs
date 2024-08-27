use crate::{game::{self, classic_game_status::{LifeStatus, Status}, job::{default_chat::chat_default, job::ChatFn}, GameString}, room::room::ChatType};

use crate::game::{time::Time, event::Event};

use super::super::{job::Job, option::{HandType, JobOption}, JobList, Team};

use crate::method::skill;

pub struct Police;

impl Police {
    pub fn new() -> Self { Self {} }
}

impl Job for Police {
    fn option(&self) -> JobOption {
        JobOption {
            job_id: JobList::Police,
            team: Team::CitizenTeam,
            hand_type: HandType::FixedHand,
            vote_right: 1,
            shared_hand: false,
        }
    }

    fn is_valid_hand(&self, time: Time, job: &Box<dyn Job + Send>, status: &Status, idx: usize) -> bool {
        if time != Time::Night {
            return false;
        }

        if status.life_status != LifeStatus::Alive {
            return false;
        }
        else {
            return true;
        }
    }

    fn hand(&self, time: Time, job: &Box<dyn Job + Send>, status: &Status, my_idx: usize, target_idx: usize) -> Vec<Event> {
        if time != Time::Night {
            return vec![];
        }

        if job.option().job_id == JobList::Mafia {
            return vec![
                Event::Skill(skill::CAUGHT_MAFIA, vec![GameString::Nickname(target_idx)], my_idx),
                Event::Memo(my_idx, target_idx),
            ];
        }
        else {
            return vec![
                Event::Skill(skill::NO_MAFIA, vec![GameString::Nickname(target_idx)], my_idx)
            ];
        }
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

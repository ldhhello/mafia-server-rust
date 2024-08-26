use crate::{game::{self, classic_game_status::Status, job::{default_chat::chat_default, job::ChatFn}}, room::room::ChatType};

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
        return false;
    }

    fn hand(&self, time: Time, job: &Box<dyn Job + Send>, status: &Status, idx: usize) -> Vec<Event> {
        if time != Time::Night {
            return vec![];
        }

        if job.option().job_id == JobList::Mafia {
            return vec![Event::Skill(skill::CAUGHT_MAFIA, vec!["김승억".into()], idx /* 여기 내 idx 들어가야ㄷ 되는데? */)];
        }
        return vec![]; /* todo */
        //todo!()
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

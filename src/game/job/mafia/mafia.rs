use crate::{game::{self, classic_game_status::Status, job::{default_chat::chat_default, job::ChatFn, option::JobStatus}}, room::room::ChatType};

use crate::game::time::Time;

use super::super::{job::Job, option::{HandType, JobOption}, JobList, Team};

pub struct Mafia;

impl Mafia {
    pub fn new() -> Mafia { Mafia {} }
}

impl Job for Mafia {
    fn option(&self) -> JobOption {
        JobOption {
            job_id: JobList::Mafia,
            team: Team::MafiaTeam,
            hand_type: HandType::MovingHand,
            vote_right: 1,
        }
    }

    fn status<'a>(&'a mut self) -> &'a mut JobStatus {
        todo!()
    }

    fn is_valid_hand(&mut self, players: &Vec<Box<dyn Job>>, idx: usize) -> bool {
        todo!()
    }

    fn hand(&self, players: &Vec<Box<dyn Job>>, idx: usize) -> Vec<game::event::Event> {
        todo!()
    }

    fn on_got_murderred(&self, players: &Vec<Box<dyn Job>>, idx: usize) -> Vec<game::event::Event> {
        todo!()
    }

    fn chat(&self, time: Time, my_status: &Status) -> ChatFn {
        if let Some(chat_fn) = chat_default(time, my_status) {
            return chat_fn;
        }

        if time == Time::Night {
            return Box::new(|job, status| {
                if job.option().job_id == JobList::Mafia ||
                (job.option().team == Team::MafiaTeam && status.connected) {
                    ChatType::Mafia
                }
                else {
                    ChatType::Null
                }
            });
        }

        return Box::new(|_, _| {
            ChatType::Null
        })
    }
}

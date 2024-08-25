use crate::{game::{classic_game_status::{LifeStatus, Status}, time::Time}, room::room::ChatType};

use super::{job::ChatFn, JobList, Team};

pub mod mafia;

fn chat_mafia(time: Time, my_status: &Status) -> Option<ChatFn> {
    if my_status.life_status == LifeStatus::Alive && time == Time::Night {
        return Some(Box::new(|job, status| {
            if status.life_status != LifeStatus::Alive ||
            job.option().job_id == JobList::Mafia ||
            (job.option().team == Team::MafiaTeam && status.connected) {
                ChatType::Mafia
            }
            else {
                ChatType::Null
            }
        }));
    }

    return None;
}

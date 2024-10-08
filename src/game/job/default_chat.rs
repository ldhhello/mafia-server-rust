use crate::{game::{classic_game_status::{LifeStatus, Status}, time::Time}, room::room::ChatType};
use super::JobList;
use super::job::ChatFn;

pub fn chat_default(time: Time, my_status: &Status) -> Option<ChatFn> {
    if my_status.life_status == LifeStatus::Dead {
        Some(Box::new(|job, status| {
            if status.life_status != LifeStatus::Alive || job.option().job_id == JobList::Shaman {
                ChatType::Dead
            }
            else {
                ChatType::Null
            }
        }))
    }
    else if my_status.life_status == LifeStatus::Seongbul {
        Some(Box::new(|_, _| {
            ChatType::Null
        }))
    }
    else if time == Time::Day || time == Time::Vote || time == Time::YesnoVote {
        Some(Box::new(|_, _| {
            ChatType::Normal
        }))
    }
    else if time == Time::FinalObjection && my_status.lynched {
        Some(Box::new(|_, _| {
            ChatType::Normal
        }))
    }
    else {
        None
    }
}

use crate::{game::{self, job::option::JobStatus}, room::room::ChatType};

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
    
    fn chat(&self, message: String) -> Box<dyn Fn(&Box<dyn Job>) -> ChatType> {
        Box::new(|job| {
            ChatType::Mafia
        })
    }
}
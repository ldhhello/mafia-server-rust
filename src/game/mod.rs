mod classic_game;
mod classic_game_status;
mod error;
pub mod event;
mod job;
mod time;

use crate::{room::room::Room, server::session::Session};
use async_trait::async_trait;
use event::Event;
use job::JobList;
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

use classic_game::ClassicGame;

#[derive(Clone, Copy)]
pub enum GameType {
    ClassicGame,
}

#[async_trait]
pub trait Game {
    async fn run(&self) -> Result<(), Box<dyn Error>>;
    async fn send(&self, event: Event) -> Result<(), Box<dyn Error>>;
}

impl GameType {
    pub fn new(self, room: Arc<Room>) -> Box<dyn Game + Send + Sync> {
        match self {
            Self::ClassicGame => Box::new(ClassicGame::new(room)),
        }
    }
}


pub enum GameString {
    Nickname(usize), /* Nickname(idx): idx번 픽의 닉네임 */
    Jobname(JobList), /* Jobname(job): job의 이름 */
}

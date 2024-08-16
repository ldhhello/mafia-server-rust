mod classic_game;
mod event;
mod job;
mod time;

use std::{error::Error, sync::Arc};
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::server::session::Session;

use classic_game::ClassicGame;

#[derive(Clone, Copy)]
pub enum GameType {
    ClassicGame
}

#[async_trait]
pub trait Game {
    async fn run(&self) -> Result<(), Box<dyn Error>>;
}

impl GameType {
    pub fn new(self, players: Arc<Mutex<Vec<Option<Arc<Session>>>>>) -> Box<dyn Game + Send> {
        match self {
            Self::ClassicGame => Box::new(ClassicGame::new(players))
        }
    }
}
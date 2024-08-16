use std::error::Error;
use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::server::session::Session;
use crate::timer::Timer;
use super::job::job::Job;
use super::time::Time;
use tokio::sync::mpsc;
use super::event::Event;

use super::Game;

type Players = Vec<Option<Arc<Session>>>;

pub struct ClassicGame {
    tx: Arc<mpsc::Sender<Event>>,
}

impl ClassicGame {
    pub fn new(players: Arc<Mutex<Players>>) -> ClassicGame {
        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
            if let Err(e) = Self::event_loop(players, rx).await {
                eprintln!("ClassicGame::event_loop error: {}", e);
            }
        });

        ClassicGame {
            tx: Arc::new(tx),
        }
    }

    async fn event_loop(players: Arc<Mutex<Players>>, mut rx: mpsc::Receiver<Event>) -> Result<(), Box<dyn Error>> {
        loop {
            if let Some(event) = rx.recv().await {
                match event {
                    Event::TimeChanged(time) => {
                        println!("Time changed to {:?}!!", time);
                    }
                }
            }
            else {
                println!("ClassicGame::event_loop finished!");
                return Ok(());
            }
        }
    }
}

#[async_trait]
impl Game for ClassicGame {
    async fn run(&self) -> Result<(), Box<dyn Error>> {
        self.tx.send(Event::TimeChanged(Time::Night)).await?;

        Ok(())
    }
}
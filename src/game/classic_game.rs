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

pub struct ClassicGame {
    players: Arc<Mutex<Vec<Option<Arc<Session>>>>>,
    tx: Arc<mpsc::Sender<Event>>,
    timer: Timer,
}

impl ClassicGame {
    pub fn new(players: Arc<Mutex<Vec<Option<Arc<Session>>>>>) -> ClassicGame {
        let (tx, rx) = mpsc::channel(32);

        tokio::spawn(async move {
            if let Err(e) = Self::event_loop(rx).await {
                eprintln!("ClassicGame::event_loop error: {}", e);
            }
        });

        ClassicGame {
            players,
            tx: Arc::new(tx),
            timer: Timer::new(),
        }
    }

    async fn event_loop(mut rx: mpsc::Receiver<Event>) -> Result<(), Box<dyn Error>> {
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
    async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.tx.send(Event::TimeChanged(Time::Night)).await?;

        Ok(())
    }
}
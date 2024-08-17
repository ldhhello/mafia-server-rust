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
use super::job::{JobList, Team};
use rand::Rng;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;

use super::Game;

type Players = Vec<Option<Arc<Session>>>;

const JOB_ARR: [JobList; 12] = [JobList::Mafia, JobList::Police, JobList::Doctor, JobList::Special, // 4인
    JobList::Special, // 5인
    JobList::Assist, // 6인
    JobList::Special, // 7인 
    JobList::Mafia, // 8인
    JobList::Cult, // 9인
    JobList::Special, // 10인
    JobList::Mafia, // 11인
    JobList::Special, // 12인
];
const SPECIAL_CNT: [usize; 13] = [usize::MAX, 0, 0, 0, 1, 2, 2, 3, 3, 3, 4, 4, 5];
const SPECIAL_ARR: [JobList; 10] = [JobList::Couple, JobList::Soldier, JobList::Politician, JobList::Shaman, 
    JobList::Reporter, JobList::Gangster, JobList::Ghoul, JobList::Terrorist,
    JobList::Detective, JobList::Priest, //JobList::Magician, JobList::Hacker,
    //JobList::Prophet, JobList::Judge, JobList::Nurse, JobList::Mentalist, 
    //
];
const ASSIST_ARR: [JobList; 6] = [JobList::Spy, JobList::Beastman, JobList::Madam, JobList::Thief, JobList::Scientist, JobList::Witch];

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
        let mut job = Vec::new();

        job.push(0);

        loop {
            if let Some(event) = rx.recv().await {
                match event {
                    Event::Start => {
                        println!("Game has been started!");
                        Self::initialize_job().await?;

                    },
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

    async fn initialize_job() -> Result<(), Box<dyn Error>> {
        let mut job_arr = JOB_ARR[0..4].to_vec();
        let special_cnt = [-1, 0, 0, 0, 1, 2, 2, 3, 3, 3, 4, 4, 5];

        let mut special_arr = [JobList::Couple, JobList::Soldier, JobList::Politician, JobList::Shaman, 
            JobList::Reporter, JobList::Gangster, JobList::Ghoul, JobList::Terrorist,
            JobList::Detective, JobList::Priest, //JobList::Magician, JobList::Hacker,
            //JobList::Prophet, JobList::Judge, JobList::Nurse, JobList::Mentalist, 
            //
        ];

        let mut rng = rand::thread_rng();
        let mut irs = Irs::default();

        irs.shuffle(&mut job_arr, &mut rng)?;

        println!("{:?}", job_arr);

        Ok(())
    }
}

#[async_trait]
impl Game for ClassicGame {
    async fn run(&self) -> Result<(), Box<dyn Error>> {
        self.tx.send(Event::Start).await?;

        Ok(())
    }
}
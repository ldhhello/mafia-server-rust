use std::error;
use std::sync::Arc;
use async_trait::async_trait;
use crate::game::time::TIME_LENGTH;
use crate::method;
use crate::packet::binarydata::BinaryData;
use crate::packet::Packet;
use crate::room::room::Room;
use crate::timer::Timer;
use super::job::job::Job;
use super::job::option::HandType;
use super::time::Time;
use tokio::sync::mpsc;
use super::event::Event;
use super::job::{JobList, Team};
use rand::{thread_rng, Rng, SeedableRng};
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use mt19937::MT19937;
use super::error::Error;
use crate::room::player::Players;
use super::classic_game_status::Status;
use crate::room::room::ChatType;

use super::Game;

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
// 4인방 연인 배정 문제 때문에 0번 인덱스가 무조건 연인이다.
const SPECIAL_ARR: [JobList; 10] = [JobList::Couple, JobList::Soldier, JobList::Politician, JobList::Shaman,
    JobList::Reporter, JobList::Gangster, JobList::Ghoul, JobList::Terrorist,
    JobList::Detective, JobList::Priest, //JobList::Magician, JobList::Hacker,
    //JobList::Prophet, JobList::Judge, JobList::Nurse, JobList::Mentalist,
    //
];
const ASSIST_ARR: [JobList; 4] = [JobList::Spy, JobList::Beastman, JobList::Madam, JobList::Thief];

pub struct ClassicGame {
    tx: Arc<mpsc::Sender<Event>>,
}

impl ClassicGame {
    // todo: players가 아니라 room을 받아야 할 듯
    pub fn new(room: Arc<Room>) -> ClassicGame {
        let (tx, rx) = mpsc::channel(32);

        let tx = Arc::new(tx);
        let tx_ = tx.clone();

        tokio::spawn(async move {
            if let Err(e) = Self::event_loop(room, tx_, rx).await {
                eprintln!("ClassicGame::event_loop error: {}", e);
            }
        });

        ClassicGame {
            tx
        }
    }

    // event_loop 내에서 send를 그냥 하면 재수없으면 데드락이 걸릴 수도 있다.
    // 이 함수를 대신 호출하자
    fn send_spawn(tx: &Arc<mpsc::Sender<Event>>, msg: Event) {
        let tx = tx.clone();
        tokio::spawn(async move {
            if let Err(e) = tx.send(msg).await {
                eprintln!("Error while send: {}", e);
            }
        });
    }
    async fn event_loop(room: Arc<Room>, tx: Arc<mpsc::Sender<Event>>, mut rx: mpsc::Receiver<Event>) -> Result<(), Box<dyn error::Error>> {
        let players = room.players.clone();
        let mut job_list = Vec::new();
        let mut job = Vec::new();
        let mut status = vec![Status::default(); players.lock().await.len()];

        let nicknames: Vec<String> = players.lock().await.iter().enumerate().map(|(idx, p)| {
            match p {
                Some(session) => session.nickname.clone(),
                None => format!("플레이어 {}", idx+1)
            }
        }).collect();

        let mut current_time = Time::Null;
        let timer = Arc::new(Timer::new());

        loop {
            if let Some(event) = rx.recv().await {
                match event {
                    Event::Start => {
                        let players = players.lock().await;

                        players.broadcast(Packet::from_data(method::START_GAME, vec![])).await;

                        let now_people = room.now_people();
                        if now_people < 4 || now_people > 12 {
                            return Err(Error::IncorrectPeopleCount.into());
                        }
                        println!("Game has been started!");
                        job_list = {
                            let job = Self::initialize_job(room.now_people()).await?;
                            let mut job_it = job.iter();

                            players.iter().map(|player| {
                                if let Some(_) = player {
                                    return Some(job_it.next().copied().unwrap_or(JobList::Citizen).clone());
                                }
                                else {
                                    return None;
                                }
                            })
                            .collect()
                        };
                        job = job_list.iter().map(|job| {
                            if let Some(job) = job {
                                Some(job.create_job())
                            }
                            else {
                                None
                            }
                        }).collect();

                        for (player, job) in players.iter().zip(job_list.iter()) {
                            if let (Some(player), Some(job)) = (player, job) {
                                let player = player.clone();
                                let job = job.clone();

                                tokio::spawn(async move {
                                    if let Err(e) = player.write_packet(Packet::from_data(method::JOB, vec![
                                        BinaryData::from_i32(job as i32)
                                    ])).await {
                                        eprintln!("Error while broadcasting job: {}", e);
                                    };
                                });
                            }
                        }

                        Self::send_spawn(&tx, Event::TimeChanged(Time::Night));
                    },
                    Event::TimeChanged(time) => {
                        println!("Time changed to {:?}!!", time);
                        current_time = time;

                        status.iter_mut().for_each(|s| s.reset(time));

                        let players = players.lock().await;
                        players.broadcast(Packet::from_data(
                            method::TIME_CHANGED,
                            vec![
                                BinaryData::from_i32(time as i32),
                                BinaryData::from_i32(TIME_LENGTH[time as usize]),
                            ]
                        )).await;

                        let next_time = match time {
                            Time::Night => Time::Day,
                            Time::Day => Time::Vote,
                            Time::Vote => Time::FinalObjection /* todo */,
                            Time::FinalObjection => Time::YesnoVote,
                            Time::YesnoVote => Time::Night,
                            _ => Time::Null
                        };

                        let timer = timer.clone();
                        let tx = tx.clone();
                        tokio::spawn(async move {
                            timer.run(TIME_LENGTH[time as usize]).await;
                            tx.send(Event::TimeChanged(next_time)).await.unwrap_or(());
                        });
                    },
                    Event::IncreaseTime(idx) => 'increase_time: {
                        if current_time != Time::Day {
                            break 'increase_time;
                        }
                        if status[idx].modified_time {
                            break 'increase_time;
                        }

                        status[idx].modified_time = true;
                        timer.increase(15).await;

                        let players = players.lock().await;
                        players.broadcast(Packet::from_data(method::INCREASE_TIME, vec![
                            BinaryData::from_string(nicknames[idx].clone())
                        ])).await;
                    },
                    Event::DecreaseTime(idx) => 'decrease_time: {
                        if current_time != Time::Day {
                            break 'decrease_time;
                        }
                        if status[idx].modified_time {
                            break 'decrease_time;
                        }

                        status[idx].modified_time = true;
                        timer.decrease(15).await;

                        let players = players.lock().await;
                        players.broadcast(Packet::from_data(method::DECREASE_TIME, vec![
                            BinaryData::from_string(nicknames[idx].clone())
                        ])).await;
                    },
                    Event::Chat(idx, msg) => {
                        let my_job = job[idx].as_ref().expect("Sender does not exist");
                        let chat_fn = my_job.chat(current_time, &status[idx]);

                        let players = players.lock().await;
                        let sender = &players[idx].as_ref().expect("Sender does not exist").nickname;
                        players.iter().zip(job.iter()).zip(status.iter()).for_each(|((p, j), s)| {
                            if let (Some(p), Some(j)) = (p, j) {
                                let chat_type = chat_fn(j, s);
                                if chat_type != ChatType::Null {
                                    let p = p.clone();
                                    let msg = msg.clone();
                                    let sender = sender.clone();

                                    tokio::spawn(async move {
                                        p.write_packet(Packet::from_data(method::CHAT, vec![
                                            BinaryData::from_i32(chat_type as i32),
                                            BinaryData::from_string(sender),
                                            BinaryData::from_string(msg)
                                        ])).await.unwrap_or(());
                                    });
                                }
                            }
                        });
                    },
                    Event::Hand(my_idx, target_idx) => {
                        if job[target_idx].is_none() {
                            continue;
                        }

                        let my_job = job[my_idx].as_ref().expect("Sender does not exist");
                        let target_job = job[target_idx].as_ref().expect("Target does not exist");
                        match my_job.option().hand_type {
                            HandType::NoHand => (),
                            HandType::FixedHand => {
                                let vec = my_job.hand(current_time, target_job, &status[target_idx], target_idx);
                                let tx = tx.clone();
                                tokio::spawn(async move {
                                    for e in vec {
                                        tx.send(e).await.unwrap_or(());
                                    }
                                });
                            },
                            HandType::MovingHand => {
                                if my_job.is_valid_hand(current_time, target_job, &status[target_idx], target_idx) {
                                    status[my_idx].hand = target_idx;

                                    let session = players.lock().await[my_idx].clone().expect("Session does not exist");
                                    tokio::spawn(async move {
                                        session.write_packet(Packet::from_data(method::HAND, vec![])).await.unwrap_or(())
                                    });
                                }
                            }
                        }
                    },
                    Event::Skill(skill_id, vec, receiver) => {
                        let players = players.lock().await;
                        if let Some(player) = players[receiver].clone() {
                            tokio::spawn(async move {
                                player.write_packet(Packet::from_data(method::SKILL,
                                    std::iter::once(BinaryData::from_i32(skill_id))
                                    .chain(vec.into_iter()
                                        .map(|s| BinaryData::from_string(s))
                                    )
                                    .collect()
                                )).await.unwrap_or(());
                            });
                        }
                    }
                }
            }
            else {
                println!("ClassicGame::event_loop finished!");
                return Ok(());
            }
        }
    }

    async fn initialize_job(people_cnt: usize) -> Result<Vec<JobList>, Box<dyn error::Error>> {
        //let mut rng = rand::thread_rng();
        let mut rng = MT19937::from_rng(thread_rng()).expect("MT19937 initialize error");
        let mut irs = Irs::default();

        let mut job_arr = JOB_ARR[0..people_cnt].to_vec();
        irs.shuffle(&mut job_arr, &mut rng)?;

        let special_cnt = SPECIAL_CNT[people_cnt];
        let mut special_arr = SPECIAL_ARR.to_vec();

        // 4인방에서 연인이 배정되지 않게 하는 코드
        if special_cnt == 1 {
            // special_arr는 항상 길이가 1 이상이기 때문에 expect()에서 절대로 실패하지 않는다
            special_arr[0] = *special_arr.last().expect("special_arr is empty!!");
            special_arr.pop();
        }

        irs.shuffle(&mut special_arr, &mut rng)?;
        special_arr.resize(special_cnt, JobList::default());

        // 연인이 한 명 배정됐다면 특수직업 한 명을 강제로 연인으로 바꿔버린다.
        if let Some(_) = special_arr.iter().find(|&&x| x == JobList::Couple) {
            let no_couple = special_arr.iter_mut().find(|x| **x != JobList::Couple).expect("there's no people who is not couple");
            *no_couple = JobList::Couple;

            irs.shuffle(&mut special_arr, &mut rng)?;
        }

        let mut assist_arr = ASSIST_ARR.to_vec();
        irs.shuffle(&mut assist_arr, &mut rng)?;

        let mut special_it = special_arr.iter();
        let mut assist_it = assist_arr.iter();

        let job_arr = job_arr.iter().map(|job| {
            match job {
                JobList::Special => special_it.next().expect("Special arr is too small"),
                JobList::Assist => assist_it.next().expect("Assist arr is too small"),
                other_job => other_job
            }.clone()
        }).collect();

        //println!("{:?}", job_arr);

        Ok(job_arr)
    }
}

#[async_trait]
impl Game for ClassicGame {
    async fn run(&self) -> Result<(), Box<dyn error::Error>> {
        self.tx.send(Event::Start).await?;

        Ok(())
    }
    async fn send(&self, event: Event) -> Result<(), Box<dyn error::Error>> {
        self.tx.send(event).await?;

        Ok(())
    }
}

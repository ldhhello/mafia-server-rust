use std::sync::Arc;
use tokio::sync::Mutex;
use crate::{game::{Game, GameType}, method, packet::{binarydata::BinaryData, Packet}, server::session::Session};
use std::error::Error;
use super::{error, manager::RoomManager};
use crate::filter::FILTER;

pub struct RoomOption {
    pub name: String,
    pub max_people: usize,
    pub password: Option<String>,
    pub game_type: GameType
}

// 방 정보 쿼리할 때 반환될 값
pub struct RoomInfo {
    pub id: i32,
    pub name: String,
    pub max_people: usize,
    pub now_people: usize,
    pub is_password: bool,
    pub is_gaming: bool
}

pub enum PlayerType {
    Empty = 0,
    Normal = 1,
    Moderator = 2
}

pub enum ChatType {
    Null = 0,
	Normal = 1,
	Dead = 2,
	Mafia = 3,
	Couple = 4,
	System = 5,
	Cult = 6
}

pub struct Room {
    id: i32,
    option: RoomOption,
    players: Arc<Mutex<Vec<Option<Arc<Session>>>>>,
    moderator: Mutex<usize>,
    manager: Arc<RoomManager>,
    game: Mutex<Option<Box<dyn Game + Send>>>,
    is_gaming: Mutex<bool>,
}

impl Room {
    pub fn new(manager: Arc<RoomManager>, id: i32, option: RoomOption) -> Room {
        let max_people = option.max_people;

        Room {
            id,
            option,
            players: Arc::new(Mutex::new(vec![None; max_people])),
            moderator: Mutex::new(0),
            manager,
            game: Mutex::new(None),
            is_gaming: Mutex::new(false),
        }
    }
    async fn now_people(&self) -> usize {
        return self.players.lock().await.iter()
            .filter(|player| player.is_some())
            .count();
    }
    pub async fn info(&self) -> RoomInfo {
        RoomInfo {
            id: self.id,
            name: self.option.name.clone(),
            max_people: self.option.max_people,
            is_password: self.option.password.is_some(),
            now_people: self.now_people().await,
            is_gaming: *self.is_gaming.lock().await,
        }
    }

    // broadcast 시에 발생하는 모든 I/O 오류는 무시한다.
    // 그러지 않으면 세션 하나 터진 것 때문에 전체 방에 broadcast가 제대로 안 되는 문제가 발생할 수 있음
    // 어차피 터진 세션은 Session::destruct() 에 의해 없어지기 때문에 여기서 따로 처리할 필요 없다.
    //
    // 또한 이 함수는 write 연산을 트리거하고 바로 반환된다.
    // 소켓 하나가 느린 것 때문에 전체 게임이 지연되면 문제가 생기기 때문
    //
    async fn broadcast(players: &Vec<Option<Arc<Session>>>, packet: Packet) {
        players.iter().for_each(|session| {
            if let Some(session) = session {
                let packet = packet.clone();
                let session = session.clone();
                tokio::spawn(async move {
                    if let Err(e) = session.write_packet(packet).await {
                        println!("Error while broadcasting: {}", e);
                    }
                });
            }
        });
    }
    pub async fn broadcast_player_list(&self) {
        let players = self.players.lock().await;

        let moderator = *self.moderator.lock().await;
        let vec = players.iter()
            .enumerate()
            .flat_map(|(idx, player)| {
                let (player_type, nickname) = match player.as_ref() {
                    None => (PlayerType::Empty, "".into()),
                    Some(p) => (
                        if idx == moderator { PlayerType::Moderator }
                        else { PlayerType::Normal },
                        p.nickname.clone()
                    )
                };
                return [
                    BinaryData::from_i32(player_type as i32),
                    BinaryData::from_string(nickname)
                ]
            })
            .collect();

        Self::broadcast(&players, Packet::from_data(method::PLAYER_LIST, vec)).await;
    }

    pub async fn join(&self, session: Arc<Session>) -> Result<(), error::Error> {
        let mut players = self.players.lock().await;
        let player = players.iter_mut().find(|player| player.is_none());

        if let Some(player) = player {
            *player = Some(session.clone());

            Self::broadcast(&players, Packet::from_data(method::PLAYER_JOINED, vec![
                BinaryData::from_string(session.nickname.clone())
            ])).await;
            return Ok(());
        }
        else {
            return Err(error::Error::RoomIsFull);
        }
    }
    pub async fn leave(self: &Arc<Room>, session: Arc<Session>) -> Result<(), error::Error> {
        let mut players = self.players.lock().await;
        let player = players.iter_mut().enumerate().find(|(_, player)| {
            if let Some(player) = player {
                return Arc::ptr_eq(&session, player);
            }
            else {
                return false;
            }
        });

        if let Some((idx, player)) = player {
            *player = None;
            
            let mut moderator = self.moderator.lock().await;
            if idx == *moderator {
                if let Some((idx, _)) = players.iter().enumerate()
                    .find(|(_, player)| player.is_some()) {
                        *moderator = idx;
                }

                let nickname = match &players[*moderator] {
                    Some(p) => p.nickname.clone(),
                    None => "".into()
                };
                Self::broadcast(&players, Packet::from_data(method::MODERATOR_CHANGED, vec![
                    BinaryData::from_string(nickname)
                ])).await;
            }

            Self::broadcast(&players, Packet::from_data(method::PLAYER_LEFT, vec![
                BinaryData::from_string(session.nickname.clone())
            ])).await;

            let this = self.clone();
            tokio::spawn(async move {
                if this.now_people().await == 0 {
                    this.manager.delete(this.id).await;
                }
            });

            return Ok(());
        }
        else {
            return Err(error::Error::AlreadyLeft);
        }
    }
    pub async fn chat(&self, session: Arc<Session>, message: String) -> Result<(), error::Error> {
        let message = FILTER.filter(message);

        let players = self.players.lock().await;

        Self::broadcast(&players, Packet::from_data(method::CHAT, vec![
            BinaryData::from_i32(ChatType::Normal as i32),
            BinaryData::from_string(session.nickname.clone()),
            BinaryData::from_string(message)
        ])).await;
        Ok(())
    }
    // index(): session의 픽번호를 반환한다 (0 based)
    pub async fn index(&self, session: Arc<Session>) -> Option<usize> {
        let Some((idx, _)) = self.players.lock().await.iter().enumerate()
            .find(|(_, player)| {
                if let Some(player) = player {
                    return Arc::ptr_eq(player, &session);
                }
                else {
                    return false;
                }
            })
        else {
            return None;
        };

        return Some(idx);
    }
    pub async fn is_moderator(&self, session: Arc<Session>) -> bool {
        let moderator = &self.players.lock().await[*self.moderator.lock().await];
        if let Some(moderator) = moderator {
            return Arc::ptr_eq(moderator, &session);
        }
        else {
            panic!("Moderator index does not exist");
        }
    }
    pub async fn start_game(&self) -> Result<(), error::Error> {
        if self.now_people().await < 4 {
            return Err(error::Error::PlayerNotEnough);
        }

        // 게임 시작
        // 추후 랭크 게임 등을 구현할 가능성도 있기 때문에 방장인지 확인은 Session에서 진행한다.
        println!("Triggered game start");

        let mut game = self.game.lock().await;
        if let Some(_) = *game {
            return Err(error::Error::AlreadyStarted);
        }

        let game_ = self.option.game_type.new(self.players.clone());

        if let Err(_) = game_.run().await {
            return Err(error::Error::CommunicationError);
        }

        *game = Some(game_);
        Ok(())
    }
}
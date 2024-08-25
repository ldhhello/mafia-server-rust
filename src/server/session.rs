use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::sync::Mutex;
use std::error::Error;
use crate::game::event::Event;
use crate::game::GameType;
use crate::packet::binarydata::BinaryData;
use crate::packet::Packet;
use crate::{method, room};
use crate::method::string;
use crate::room::manager::RoomManager;
use crate::room::room::{Room, RoomOption};

use super::{Stream, WritePacket, ReadPacket};

pub struct Session {
    read: Mutex<ReadHalf<Stream>>,
    write: Mutex<WriteHalf<Stream>>,
    pub nickname: String,
    room: Mutex<Option<Arc<Room>>>,
    room_manager: Arc<RoomManager>,
    index: Mutex<usize>
}

impl Session {
    pub fn new(socket: Stream, room_manager: Arc<RoomManager>) -> Self {
        let (read, write) = tokio::io::split(socket);

        return Self {
            read: Mutex::new(read), 
            write: Mutex::new(write),
            nickname: "".into(),
            room: Mutex::new(None),
            room_manager,
            index: Mutex::new(0),
        }
    }
    pub async fn write_packet(&self, packet: Packet) -> Result<(), Box<dyn Error>> {
        self.write.lock().await.write_packet(packet).await?;
        return Ok(());
    }
    // Session::read_packet은 PING 처리까지 해 줌
    async fn read_packet(&self) -> Result<Packet, Box<dyn Error>> {
        return loop {
            let packet = self.read.lock().await.read_packet().await?;

            if packet.get_method() == method::PING {
                self.write_packet(Packet::from_data(method::PONG, vec![])).await?;
                continue;
            }
    
            break Ok(packet);
        };
    }
    pub async fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        self.write_packet(Packet::from_data(method::HELLO, vec![
            BinaryData::from_string("Mafia_Server 2.0_Beta".into())
        ])).await?;

        let mut packet = self.read_packet().await?;

        if packet.get_method() != method::NICKNAME {
            return Err(Box::from("Invalid method call"));
        }

        self.nickname = packet.get_move(0).as_string()?;
        println!("Nickname: {}", self.nickname);

        Ok(())
    }
    pub async fn join_room(self: &Arc<Self>, room: Arc<Room>) -> Result<(), Box<dyn Error>> {
        match room.join(self.clone()).await {
            Ok(()) => {
                *self.index.lock().await = room.index(self.clone()).await.expect("Session index must exist");
            },
            Err(room::Error::RoomIsFull) => {
                self.write_packet(Packet::from_data(method::ERROR, vec![
                    BinaryData::from_i32(string::ROOM_IS_FULL)
                ])).await?;
            },
            Err(e) => return Err(Box::new(e))
        }

        *self.room.lock().await = Some(room.clone());

        let room_info = room.info().await;

        self.write_packet(Packet::from_data(method::JOIN_ROOM, vec![
            BinaryData::from_i32(room_info.id),
            BinaryData::from_string(room_info.name),
            BinaryData::from_i32(room_info.now_people as i32),
            BinaryData::from_i32(room_info.max_people as i32),
        ])).await?;
        room.broadcast_player_list().await;
        Ok(())
    }
    pub async fn leave_room(self: &Arc<Self>) -> Result<(), Box<dyn Error>> {
        let mut room = self.room.lock().await;
        
        if let Some(room) = room.as_ref() {
            room.leave(self.clone()).await?;
            room.broadcast_player_list().await;
        }

        *room = None;
        Ok(())
    }
    pub async fn run(self: &Arc<Self>) -> Result<(), Box<dyn Error>> {
        loop {
            let mut packet = self.read_packet().await?;

            match packet.get_method() {
                method::ROOM_LIST => {
                    let list = self.room_manager.list().await.into_iter()
                        .flat_map(|info| [
                            BinaryData::from_i32(info.id),
                            BinaryData::from_string(info.name),
                            BinaryData::from_i32(info.now_people as i32),
                            BinaryData::from_i32(info.max_people as i32),
                            BinaryData::from_i32(info.is_password as i32),
                        ])
                        .collect();
                    self.write_packet(Packet::from_data(method::ROOM_LIST, list)).await?;
                },
                method::CREATE_ROOM => {
                    let room_name = packet.get_move(0).as_string()?;
                    let max_people = packet.get(1).to_i32();
                    let is_password_room = packet.get(2).to_bool();
                    let password = packet.get_move(3).as_string()?;

                    if self.room.lock().await.is_some() {
                        continue;
                    }

                    if max_people < 4 || max_people > 12 {
                        self.write_packet(Packet::from_data(method::ERROR, vec![
                            BinaryData::from_i32(string::ROOM_SIZE_IS_INVALID)
                        ])).await?;
                        continue;
                    }

                    let options = RoomOption {
                        name: room_name,
                        max_people: max_people as usize,
                        password: if is_password_room { Some(password) } else { None },
                        game_type: GameType::ClassicGame
                    };

                    let room = self.room_manager.create(options).await;

                    self.join_room(room).await?;
                },
                method::JOIN_ROOM => {
                    if self.room.lock().await.is_some() {
                        continue;
                    }

                    let id = packet.get(0).to_i32();
                    match self.room_manager.get(id).await {
                        None => self.write_packet(Packet::from_data(method::ERROR, vec![
                            BinaryData::from_i32(string::ROOM_DOES_NOT_EXIST)
                        ])).await?,
                        Some(room) => self.join_room(room).await?
                    }
                },
                method::LEAVE_ROOM => {
                    if self.room.lock().await.is_none() {
                        continue;
                    }

                    // 이거 match 왜 안됨?..
                    if let Err(_) = self.leave_room().await {
                        continue;
                    }
                    self.write_packet(Packet::from_data(method::LEAVE_ROOM, vec![])).await?
                },
                method::CHAT => {
                    if let Some(room) = self.room.lock().await.clone() {
                        let message = packet.get_move(0).as_string()?;
                        room.chat(self.clone(), message).await?;
                    }
                },
                method::INDEX => {
                    if let Some(room) = self.room.lock().await.clone() {
                        let Some(idx) = room.index(self.clone()).await
                        else {
                            panic!("Session is in room but isn't included in room");
                        };
                        self.write_packet(Packet::from_data(method::INDEX, vec![
                            BinaryData::from_i32(idx as i32)
                        ])).await?;
                    }
                },
                method::START_GAME => {
                    if let Some(room) = self.room.lock().await.clone() {
                        if !room.is_moderator(self.clone()).await {
                            continue;
                        }
                        if let Err(room::Error::PlayerNotEnough) = room.start_game().await {
                            self.write_packet(Packet::from_data(method::ERROR, vec![
                                BinaryData::from_i32(method::string::PLAYER_NOT_ENOUGH)
                            ])).await?;
                        }
                    }
                },
                method::INCREASE_TIME => {
                    if let Some(room) = self.room.lock().await.clone() {
                        room.send(Event::IncreaseTime(*self.index.lock().await)).await.unwrap_or(());
                    }
                },
                method::DECREASE_TIME => {
                    if let Some(room) = self.room.lock().await.clone() {
                        room.send(Event::DecreaseTime(*self.index.lock().await)).await.unwrap_or(());
                    }
                }
                _ => ()
            }
        }
    }
    pub async fn destruct(self: Arc<Self>) -> Result<(), Box<dyn Error>> {
        // 방에서 나가기 등의 연산을 한다.
        self.leave_room().await.unwrap_or(());

        self.write.lock().await.shutdown().await?;
        Ok(())
    }
}
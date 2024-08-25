// player.rs : 플레이어들에게 일괄적으로 적용할 연산 같은 걸 관리한다
// broadcast, broadcast_if, broadcast_each
// memo, memo_each, memo_all

use std::sync::Arc;
use async_trait::async_trait;

use crate::{method, packet::{binarydata::BinaryData, Packet}, server::session::Session};

use super::room::{PlayerType, Room};

//type Players = Vec<Option<Arc<Session>>>;

#[async_trait]
pub trait Players {
    async fn broadcast(&self, packet: Packet);
    async fn broadcast_if<F>(&self, packet: Packet, condition: F)
        where F: (Fn(usize, Arc<Session>) -> bool) + std::marker::Send;
    async fn broadcast_each<F>(&self, packet: F)
        where F: (Fn(usize, Arc<Session>) -> Option<Packet>) + std::marker::Send;
}

#[async_trait]
impl Players for Vec<Option<Arc<Session>>> {
    async fn broadcast(&self, packet: Packet) {
        self.iter().for_each(|session| {
            if let Some(session) = session {
                let packet = packet.clone();
                let session = session.clone();
                tokio::spawn(async move {
                    if let Err(e) = session.write_packet(packet).await {
                        eprintln!("Error while broadcasting: {}", e);
                    }
                });
            }
        });
    }
    async fn broadcast_if<F>(&self, packet: Packet, condition: F)
    where F: (Fn(usize, Arc<Session>) -> bool) + std::marker::Send {
        self.iter().enumerate().for_each(|(idx, session)| {
            if let Some(session) = session {
                if !condition(idx, session.clone()) {
                    return;
                }

                let packet = packet.clone();
                let session = session.clone();
                tokio::spawn(async move {
                    if let Err(e) = session.write_packet(packet).await {
                        eprintln!("Error while broadcasting: {}", e);
                    }
                });
            }
        });
    }
    async fn broadcast_each<F>(&self, packet: F)
    where F: (Fn(usize, Arc<Session>) -> Option<Packet>) + std::marker::Send {
        self.iter().enumerate().for_each(|(idx, session)| {
            if let Some(session) = session {
                let packet = packet(idx, session.clone());

                if let Some(packet) = packet {
                    let session = session.clone();
                    tokio::spawn(async move {
                        if let Err(e) = session.write_packet(packet).await {
                            eprintln!("Error while broadcasting: {}", e);
                        }
                    });
                }
            }
        });
    }
}

impl Room {
    // broadcast 시에 발생하는 모든 I/O 오류는 무시한다.
    // 그러지 않으면 세션 하나 터진 것 때문에 전체 방에 broadcast가 제대로 안 되는 문제가 발생할 수 있음
    // 어차피 터진 세션은 Session::destruct() 에 의해 없어지기 때문에 여기서 따로 처리할 필요 없다.
    //
    // 또한 이 함수는 write 연산을 트리거하고 바로 반환된다.
    // 소켓 하나가 느린 것 때문에 전체 게임이 지연되면 문제가 생기기 때문
    //
    // pub async fn broadcast(players: &Vec<Option<Arc<Session>>>, packet: Packet) {
    //     players.iter().for_each(|session| {
    //         if let Some(session) = session {
    //             let packet = packet.clone();
    //             let session = session.clone();
    //             tokio::spawn(async move {
    //                 if let Err(e) = session.write_packet(packet).await {
    //                     eprintln!("Error while broadcasting: {}", e);
    //                 }
    //             });
    //         }
    //     });
    // }
}
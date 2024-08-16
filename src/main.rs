mod filter;
mod packet;
mod server;
mod method;
mod room;
mod game;
mod timer;

use native_tls::TlsStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::error::Error;
use std::sync::Arc;
use room::manager::RoomManager;

use server::session::Session;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("mafia-server-rust by Donghyun Lee");

    let manager = Arc::new(RoomManager::new());

    let addr: String = "0.0.0.0:6002".into();
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (socket, _) = listener.accept().await?;

        let manager = manager.clone();
        tokio::spawn(async move {
            let mut session = Session::new(socket, manager);
            if let Err(e) = session.initialize().await {
                eprintln!("Session Initialize Error: {}", e);
                // initialize 시에 오류가 난 경우 그대로 종료한다.
                // 들어가있는 방 같은 게 있을 리가 없기 때문에 side effect가 없음
                return;
            };
            let session = Arc::new(session);
            if let Err(e) = session.run().await {
                eprintln!("Session Run Error: {}", e);
            };

            if let Err(e) = session.destruct().await {
                eprintln!("Session Destruct Error: {}", e)
            }
        });
    }
}

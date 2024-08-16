pub mod session;

use tokio::net::TcpStream;
use crate::packet::Packet;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};

type Stream = TcpStream;

trait ReadPacket {
    async fn read_packet(&mut self) -> Result<Packet, Box<dyn Error>>;
}
trait WritePacket {
    async fn write_packet(&mut self, packet: Packet) -> Result<(), Box<dyn Error>>;
}

impl ReadPacket for ReadHalf<Stream> {
    async fn read_packet(&mut self) -> Result<Packet, Box<dyn Error>> {
        let len = self.read_u32().await?;

        if len > 100000 {
            return Err(Box::from("Packet len is too long"));
        }

        let mut vec = Vec::new();
        vec.resize((len-4) as usize, 0);
        self.read_exact(&mut vec).await?;
        let packet = Packet::from_binary(vec)?;

        Ok(packet)
    }
}
impl WritePacket for WriteHalf<Stream> {
    async fn write_packet(&mut self, packet: Packet) -> Result<(), Box<dyn Error>> {
        let data = packet.to_binary(true);
        self.write_all(&data[..]).await?;
        Ok(())
    }
}
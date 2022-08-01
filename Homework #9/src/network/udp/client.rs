use crate::network::udp::protocol;
use std::error::Error;
use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct UdpClient {
    socket: UdpSocket,
}

impl UdpClient {
    pub async fn new(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(address).await?;
        Ok(Self { socket })
    }
    pub async fn send_message(
        &self,
        address: impl ToSocketAddrs,
        message: f32,
    ) -> Result<(), Box<dyn Error>> {
        protocol::send_f32(&self.socket, address, message).await?;
        Ok(())
    }
}

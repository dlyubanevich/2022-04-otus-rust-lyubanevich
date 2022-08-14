use std::error::Error;

use crate::network::udp::protocol;
use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct UdpServer {
    socket: UdpSocket,
}

impl UdpServer {
    pub async fn new(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(address).await?;
        Ok(Self { socket })
    }
    pub async fn get_message(&self) -> Result<f32, Box<dyn Error>> {
        protocol::receive_f32(&self.socket).await
    }
}

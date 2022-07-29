use std::{
    error::Error,
    net::{ToSocketAddrs, UdpSocket},
    time::Duration,
};

use crate::network::udp::protocol;

pub struct UdpServer {
    socket: UdpSocket,
}

impl UdpServer {
    pub fn new(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(address)?;
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;
        Ok(Self { socket })
    }
    pub fn get_message(&self) -> Result<f32, Box<dyn Error>> {
        protocol::receive_f32(&self.socket)
    }
}

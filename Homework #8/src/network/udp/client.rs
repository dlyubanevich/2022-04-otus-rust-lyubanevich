use std::{
    error::Error,
    net::{ToSocketAddrs, UdpSocket},
};

use crate::network::udp::protocol;

pub struct UdpClient {
    socket: UdpSocket,
}

impl UdpClient {
    pub fn new(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(address)?;
        Ok(Self { socket })
    }
    pub fn send_message(
        &self,
        address: impl ToSocketAddrs,
        message: f32,
    ) -> Result<(), Box<dyn Error>> {
        protocol::send_f32(&self.socket, address, message)?;
        Ok(())
    }
}

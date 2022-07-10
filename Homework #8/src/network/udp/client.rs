use std::{
    error::Error,
    net::{ToSocketAddrs, UdpSocket},
};

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
        let buffer = f32::to_be_bytes(message);
        self.socket.send_to(&buffer, address)?;
        Ok(())
    }
}

use std::{
    error::Error,
    net::{ToSocketAddrs, UdpSocket},
    time::Duration,
};

pub struct UdpServer {
    socket: UdpSocket,
}

impl UdpServer {
    pub fn new(address: impl ToSocketAddrs) -> Result<Self, Box<dyn Error>> {
        let socket = UdpSocket::bind(address)?;
        socket.set_read_timeout(Some(Duration::from_secs(1)))?;
        Ok(Self { socket })
    }
    pub fn get_message(&self) -> Result<[u8; 4], Box<dyn Error>> {
        let mut buffer = [0; 4];
        self.socket.recv(&mut buffer)?;
        Ok(buffer)
    }
}

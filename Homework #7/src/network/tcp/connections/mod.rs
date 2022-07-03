use crate::network::tcp::protocol;
use crate::network::tcp::types::{ReceiveResult, SendResult};
use std::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
    pub fn receive_request(&mut self) -> ReceiveResult {
        protocol::receive_string(&mut self.stream)
    }

    pub fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        protocol::send_string(&mut self.stream, response)
    }
}

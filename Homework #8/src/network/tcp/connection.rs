use crate::network::common::types::{ReceiveResult, SendResult};
use crate::network::tcp::protocol;
use std::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
    pub fn receive_request(&mut self) -> ReceiveResult<String> {
        protocol::receive_string(&mut self.stream)
    }

    pub fn send_response<Resp: AsRef<str>>(&mut self, response: Resp) -> SendResult {
        protocol::send_string(&mut self.stream, response)
    }
}

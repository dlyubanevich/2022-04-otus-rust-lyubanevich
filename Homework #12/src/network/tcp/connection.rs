use crate::network::common::types::{ReceiveResult, SendResult};
use crate::network::tcp::protocol;
use tokio::net::TcpStream;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self { stream }
    }
    pub async fn receive_request(&self) -> ReceiveResult<String> {
        protocol::receive_string(&self.stream).await
    }

    pub async fn send_response<Resp: AsRef<str>>(&self, response: Resp) -> SendResult {
        protocol::send_string(&self.stream, response).await
    }
}

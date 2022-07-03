use crate::network::tcp::protocol;
use crate::network::tcp::types::{ConnectionResult, RequestResult};
use std::net::{TcpStream, ToSocketAddrs};

pub struct TcpClient {
    stream: TcpStream,
}
impl TcpClient {
    pub fn connect<Addrs>(addrs: Addrs) -> ConnectionResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        Ok(Self { stream })
    }

    pub fn send_request<R: AsRef<str>>(&mut self, request: R) -> RequestResult {
        protocol::send_string(&mut self.stream, request)?;
        let response = protocol::receive_string(&mut self.stream)?;
        Ok(response)
    }
}

use crate::network::common::types::{ConnectionResult, RequestResult};
use crate::network::tcp::protocol;
use tokio::net::{TcpStream, ToSocketAddrs};

pub struct TcpClient {
    stream: TcpStream,
}
impl TcpClient {
    pub async fn connect<Addrs>(addrs: Addrs) -> ConnectionResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs).await?;
        Ok(Self { stream })
    }

    pub async fn send_request<R: AsRef<str>>(&mut self, request: R) -> RequestResult<String> {
        protocol::send_string(&self.stream, request).await?;
        let response = protocol::receive_string(&self.stream).await?;
        Ok(response)
    }
}

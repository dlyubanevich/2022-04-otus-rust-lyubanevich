use crate::network::common::types::{BindResult, ConnectionResult};
use crate::network::tcp::connection::Connection;
use tokio::net::{TcpListener, ToSocketAddrs};

pub struct TcpServer {
    tcp: TcpListener,
}

impl TcpServer {
    pub async fn bind<Addrs>(addrs: Addrs) -> BindResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs).await?;
        Ok(Self { tcp })
    }

    pub async fn accept(&self) -> ConnectionResult<Connection> {
        let (stream, _) = self.tcp.accept().await?;
        Ok(Connection::new(stream))
    }
}

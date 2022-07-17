use crate::network::common::errors::ConnectionError;
use crate::network::common::types::{BindResult, ConnectionResult};
use crate::network::tcp::connection::Connection;
use std::net::{TcpListener, ToSocketAddrs};

pub struct TcpServer {
    tcp: TcpListener,
}

impl TcpServer {
    pub fn bind<Addrs>(addrs: Addrs) -> BindResult<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let tcp = TcpListener::bind(addrs)?;
        Ok(Self { tcp })
    }
    /// Blocking iterator for incoming connections.
    pub fn incoming(&self) -> impl Iterator<Item = ConnectionResult<Connection>> + '_ {
        self.tcp.incoming().map(|s| match s {
            Ok(stream) => Ok(Connection::new(stream)),
            Err(error) => Err(ConnectionError::Io(error)),
        })
    }
}

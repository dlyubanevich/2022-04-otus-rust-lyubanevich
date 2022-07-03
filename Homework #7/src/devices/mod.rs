mod clients;
mod socket;
mod thermometer;
mod traits;

pub use clients::socket::SocketTcpClient;
pub use socket::Socket;
pub use thermometer::Thermometer;

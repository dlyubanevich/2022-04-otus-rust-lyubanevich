use std::io;

use crate::network::common::errors::ReceiveError;
use crate::network::common::types::{ReceiveResult, SendResult};
use tokio::net::TcpStream;

pub async fn send_string<D: AsRef<str>>(stream: &TcpStream, data: D) -> SendResult {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    write_all(stream, &len_bytes).await?;
    write_all(stream, bytes).await?;
    Ok(())
}

pub async fn receive_string(stream: &TcpStream) -> ReceiveResult<String> {
    let mut buf = [0; 4];
    read_exact(stream, &mut buf).await?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    read_exact(stream, &mut buf).await?;
    String::from_utf8(buf).map_err(|_| ReceiveError::BadEncoding)
}

async fn write_all(stream: &TcpStream, buffer: &[u8]) -> io::Result<()> {
    let mut written: usize = 0;
    while written < buffer.len() {
        stream.writable().await?;
        match stream.try_write(&buffer[written..]) {
            Ok(0) => break,
            Ok(n) => written += n,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error),
        }
    }

    Ok(())
}

async fn read_exact(stream: &TcpStream, buffer: &mut [u8]) -> io::Result<()> {
    let mut readed: usize = 0;
    while readed < buffer.len() {
        stream.readable().await?;
        match stream.try_read(&mut buffer[readed..]) {
            Ok(0) => break,
            Ok(n) => readed += n,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error),
        }
    }

    Ok(())
}

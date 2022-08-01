use crate::network::common::types::SendResult;
use tokio::net::{ToSocketAddrs, UdpSocket};

pub async fn send_f32(socket: &UdpSocket, address: impl ToSocketAddrs, data: f32) -> SendResult {
    let bytes = data.to_be_bytes();
    let mut sended_len: usize = 0;
    let required_len = bytes.len();
    while sended_len < bytes.len() {
        let buffer_slice = bytes.get(sended_len..required_len).unwrap();
        sended_len += socket.send_to(buffer_slice, &address).await?;
    }
    Ok(())
}

pub async fn receive_f32(socket: &UdpSocket) -> Result<f32, Box<dyn std::error::Error>> {
    let mut buffer = [0; 4];
    let mut received_len: usize = 0;
    let required_len = buffer.len();
    while received_len < buffer.len() {
        let buffer_slice = buffer.get_mut(received_len..required_len).unwrap();
        received_len += socket.recv(buffer_slice).await?;
    }
    Ok(f32::from_be_bytes(buffer))
}

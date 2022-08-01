use crate::devices::traits::DeviceStatus;
use crate::devices::Socket;

pub struct SocketTcpClient {
    socket: Socket,
}
impl SocketTcpClient {
    pub fn new(socket: Socket) -> Self {
        Self { socket }
    }

    pub fn handle_command(&mut self, command: String) -> String {
        match command.as_str() {
            "switch" => {
                if self.socket.is_on() {
                    self.socket.switch_off();
                    "The socket is switch off".to_string()
                } else {
                    self.socket.switch_on();
                    "The socket is switch on".to_string()
                }
            }
            "status" => self.socket.get_status(),
            other => format!("Bad command {}", other),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_switch_socket() {
        let mut client = SocketTcpClient {
            socket: Socket::new(""),
        };
        assert!(client.socket.is_on());
        client.handle_command("switch".to_string());
        assert!(!client.socket.is_on());
    }
}

use application::SocketHandler;

pub fn main() {
    let mut socket = SocketHandler::default();

    socket.get_state();
    socket.switch_state();
    socket.get_state();
}

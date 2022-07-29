use smart_home::{
    devices::{Socket, SocketTcpClient},
    network::{Connection, TcpServer},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket_client = SocketTcpClient::new(Socket::new("Socket"));
    let receiver = TcpServer::bind("127.0.0.1:8080")?;
    for connection in receiver.incoming() {
        let connection = match connection {
            Ok(connection) => connection,
            Err(error) => {
                eprintln!("Can't establish connection: {}", error);
                continue;
            }
        };
        handle_connection(connection, &mut socket_client)?;
        break;
    }
    Ok(())
}

fn handle_connection(
    mut connection: Connection,
    socket_client: &mut SocketTcpClient,
) -> Result<(), Box<dyn std::error::Error>> {
    while let Ok(command) = connection.receive_request() {
        let response = socket_client.handle_command(command);
        connection.send_response(response)?;
    }
    Ok(())
}

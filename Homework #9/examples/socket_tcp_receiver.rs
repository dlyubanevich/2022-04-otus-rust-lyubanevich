use smart_home::{
    devices::{Socket, SocketTcpClient},
    network::{Connection, TcpServer},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut socket_client = SocketTcpClient::new(Socket::new("Socket"));
    let receiver = TcpServer::bind("127.0.0.1:8080").await?;
    loop {
        let connection = match receiver.accept().await {
            Ok(connection) => connection,
            Err(error) => {
                eprintln!("Can't establish connection: {}", error);
                continue;
            }
        };
        handle_connection(connection, &mut socket_client).await?;
        break;
    }
    Ok(())
}

async fn handle_connection(
    connection: Connection,
    socket_client: &mut SocketTcpClient,
) -> Result<(), Box<dyn std::error::Error>> {
    while let Ok(command) = connection.receive_request().await {
        let response = socket_client.handle_command(command);
        connection.send_response(response).await?;
    }
    Ok(())
}

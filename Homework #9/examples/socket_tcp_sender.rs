use smart_home::network::TcpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TcpClient::connect("127.0.0.1:8080").await?;
    let response = client.send_request("switch").await?;
    println!("response of switch command: {}", response);
    let response = client.send_request("status").await?;
    println!("response of status command: {}", response);
    Ok(())
}

use smart_home::network::TcpClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TcpClient::connect("127.0.0.1:8080")?;
    let response = client.send_request("switch")?;
    println!("response of switch command: {}", response);
    let response = client.send_request("status")?;
    println!("response of status command: {}", response);
    Ok(())
}

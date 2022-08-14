use std::{thread, time::Duration};

use smart_home::devices::{Thermometer, ThermometerUdpClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind_address = "127.0.0.1:34254";
    let thermometr = Thermometer::new("Thermo");
    let thermometr_client = ThermometerUdpClient::new(thermometr, bind_address).await?;
    for _ in 0..10 {
        thread::sleep(Duration::from_secs(1));
        let temperature = thermometr_client.get_temperature();
        println!("The temperature is {temperature}");
    }
    Ok(())
}

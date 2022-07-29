use std::{
    thread,
    time::{Duration, Instant},
};

use smart_home::network::UdpClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bind_address = "127.0.0.1:34255";
    let receiver_address = "127.0.0.1:34254";
    let socket_client = UdpClient::new(bind_address)?;
    let generator = TemperatureGenerator::default();
    for _ in 0..12 {
        thread::sleep(Duration::from_secs(1));
        let message = generator.generate();
        if let Err(error) = socket_client.send_message(receiver_address, message) {
            println!("can't send temperature: {error}")
        }
    }

    Ok(())
}

struct TemperatureGenerator {
    started: Instant,
}

impl Default for TemperatureGenerator {
    fn default() -> Self {
        Self {
            started: Instant::now(),
        }
    }
}

impl TemperatureGenerator {
    pub fn generate(&self) -> f32 {
        let delay = Instant::now() - self.started;
        20.0 + (delay.as_secs_f32() / 2.0).sin()
    }
}

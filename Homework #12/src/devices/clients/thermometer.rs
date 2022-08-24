use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use crate::{devices::Thermometer, network::UdpServer};
use tokio::net::ToSocketAddrs;

pub struct ThermometerUdpClient {
    thermometer: Arc<Mutex<Thermometer>>,
    state: Arc<Mutex<State>>,
}

impl ThermometerUdpClient {
    pub async fn new(
        thermometer: Thermometer,
        address: impl ToSocketAddrs,
    ) -> Result<Self, Box<dyn Error>> {
        let thermometer = Arc::new(Mutex::new(thermometer));
        let state = Arc::new(Mutex::new(State::Running));
        let socket = UdpServer::new(address).await?;
        Self::run_measuring(socket, Arc::clone(&thermometer), Arc::clone(&state)).await;

        Ok(Self { thermometer, state })
    }

    async fn run_measuring(
        socket: UdpServer,
        thermometer: Arc<Mutex<Thermometer>>,
        state: Arc<Mutex<State>>,
    ) {
        tokio::spawn(async move {
            loop {
                if let State::Stopped = *state.lock().unwrap() {
                    return;
                }

                match socket.get_message().await {
                    Ok(value) => thermometer.lock().unwrap().set_temperature(value),
                    Err(error) => println!("Can't receive message: {error}"),
                }
            }
        });
    }
    pub fn get_temperature(&self) -> f32 {
        self.thermometer.lock().unwrap().get_temperature()
    }
}
impl Drop for ThermometerUdpClient {
    fn drop(&mut self) {
        *self.state.lock().unwrap() = State::Stopped;
    }
}

enum State {
    Running,
    Stopped,
}

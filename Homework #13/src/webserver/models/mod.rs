use serde_derive::{Deserialize, Serialize};

use crate::core::SmartHomeErrors;
use crate::core::SmartHomeProvider;
use crate::provider::InfoProvider;

use super::traits::SmartHomeRepository;

pub struct RequestHandler<S, D>
where
    S: SmartHomeProvider,
    D: InfoProvider,
{
    smarthome_provider: S,
    info_provider: D,
}
impl<S: SmartHomeProvider, D: InfoProvider> RequestHandler<S, D> {
    pub fn new(smarthome_provider: S, info_provider: D) -> Self {
        RequestHandler {
            smarthome_provider,
            info_provider,
        }
    }
}

impl<S: SmartHomeProvider, D: InfoProvider> SmartHomeRepository for RequestHandler<S, D> {
    fn get_rooms(&self) -> Vec<Room> {
        self.smarthome_provider
            .get_rooms()
            .into_iter()
            .map(|room| Room {
                id: room.id,
                name: room.name,
            })
            .collect()
    }

    fn get_devices(&self, room_id: u32) -> Vec<Device> {
        self.smarthome_provider
            .get_devices(room_id)
            .into_iter()
            .map(|device| Device {
                id: device.id,
                name: device.name.to_string(),
            })
            .collect()
    }

    fn add_room(&mut self, room_name: String) -> Result<(), SmartHomeErrors> {
        self.smarthome_provider.add_room(&room_name);
        Ok(())
    }

    fn add_device(&mut self, room_id: u32, device_name: String) -> Result<(), SmartHomeErrors> {
        todo!()
    }

    fn delete_room(&mut self, room_id: u32) -> Result<(), SmartHomeErrors> {
        self.smarthome_provider.delete_room(room_id)
    }

    fn delete_device(&mut self, room_id: u32, device_id: u32) -> Result<(), SmartHomeErrors> {
        self.smarthome_provider.delete_device(room_id, device_id)
    }

    fn create_report(&self) -> Report {
        let text = self.smarthome_provider.create_report(&self.info_provider);
        Report { text }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Room {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    pub text: String,
}

use serde_derive::{Deserialize, Serialize};

use crate::core::{SmartHomeErrors, SmartHomeService};

pub struct RequestHandler {
    service: SmartHomeService,
}
impl RequestHandler {
    pub fn new(service: SmartHomeService) -> Self {
        RequestHandler { service }
    }
}

impl RequestHandler {
    pub async fn get_rooms(&self) -> Vec<Room> {
        self.service
            .get_rooms()
            .await
            .into_iter()
            .map(|room| Room { name: room.name })
            .collect()
    }

    pub async fn get_devices(&self, room_name: String) -> Vec<Device> {
        self.service
            .get_devices(&room_name)
            .await
            .into_iter()
            .map(|device| Device { name: device.name })
            .collect()
    }

    pub async fn add_room(&mut self, room_name: String) -> Result<(), SmartHomeErrors> {
        match self.service.add_room(&room_name).await {
            Ok(_) => Ok(()),
            Err(error) => Err(error),
        }
    }

    pub async fn add_device(
        &mut self,
        room_name: String,
        device_name: String,
    ) -> Result<(), SmartHomeErrors> {
        self.service.add_device(&room_name, &device_name).await
    }

    pub async fn delete_room(&mut self, room_name: String) -> Result<(), SmartHomeErrors> {
        self.service.delete_room(&room_name).await
    }

    pub async fn delete_device(
        &mut self,
        room_name: String,
        device_name: String,
    ) -> Result<(), SmartHomeErrors> {
        self.service.delete_device(&room_name, &device_name).await
    }

    pub async fn create_report(&self) -> Report {
        let text = self.service.create_report().await;
        Report { text }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Room {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Report {
    pub text: String,
}

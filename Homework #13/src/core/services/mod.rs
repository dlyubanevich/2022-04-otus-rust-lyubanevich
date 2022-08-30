use std::collections::HashMap;

use crate::{
    core::{SmartHome, SmartHomeErrors},
    database::{
        models::{DeviceDTO, RoomDTO},
        SqliteRepository,
    },
    provider::DeviceInfoProvider,
};

pub struct SmartHomeService {
    smarthome: SmartHome,
    info_provider: DeviceInfoProvider,
    repository: SqliteRepository,
    rooms: HashMap<String, u32>,
}

impl SmartHomeService {
    pub fn new(
        smarthome: SmartHome,
        info_provider: DeviceInfoProvider,
        repository: SqliteRepository,
    ) -> Self {
        SmartHomeService {
            smarthome,
            info_provider,
            repository,
            rooms: Default::default(),
        }
    }

    pub async fn add_room(&mut self, room_name: &str) -> Result<u32, SmartHomeErrors> {
        if self.smarthome.room_exiest(room_name) {
            return Err(SmartHomeErrors::RoomCreateError(
                room_name.to_string(),
                "Room is already exiest".to_string(),
            ));
        }
        let result = self.repository.add_room(room_name).await;
        match result {
            Ok(id) => {
                self.smarthome.add_room(room_name);
                self.rooms.insert(room_name.to_string(), id);
                Ok(id)
            }
            Err(error) => Err(SmartHomeErrors::RoomCreateError(
                room_name.to_string(),
                error.to_string(),
            )),
        }
    }

    pub async fn delete_room(&mut self, room_name: &str) -> Result<(), SmartHomeErrors> {
        if !self.smarthome.room_exiest(room_name) {
            return Err(SmartHomeErrors::RoomNotFound(room_name.to_string()));
        }
        let result = self.repository.delete_room(room_name).await;
        match result {
            Ok(_) => {
                self.rooms.remove(room_name);
                self.smarthome.delete_room(room_name)
            }
            Err(error) => Err(SmartHomeErrors::RoomDeleteError(
                room_name.to_string(),
                error.to_string(),
            )),
        }
    }

    pub async fn add_device(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> Result<(), SmartHomeErrors> {
        if self.smarthome.device_exiest(room_name, device_name) {
            return Err(SmartHomeErrors::DeviceCreateError(
                room_name.to_string(),
                "Device is already exiest".to_string(),
            ));
        }
        let room_id = *self.rooms.get(room_name).unwrap();
        let result = self.repository.add_device(room_id, device_name).await;

        match result {
            Ok(_) => {
                self.smarthome.add_device(room_name, device_name);
                Ok(())
            }
            Err(error) => Err(SmartHomeErrors::DeviceCreateError(
                device_name.to_string(),
                error.to_string(),
            )),
        }
    }

    pub async fn delete_device(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> Result<(), SmartHomeErrors> {
        if !self.smarthome.device_exiest(room_name, device_name) {
            return Err(SmartHomeErrors::DeviceNotFound(
                device_name.to_string(),
                room_name.to_string(),
            ));
        }
        let room_id = *self.rooms.get(room_name).unwrap();
        let result = self.repository.delete_device(room_id, device_name).await;
        match result {
            Ok(_) => self.smarthome.delete_device(room_name, device_name),
            Err(error) => Err(SmartHomeErrors::DeviceDeleteError(
                room_name.to_string(),
                error.to_string(),
            )),
        }
    }

    pub async fn get_rooms(&self) -> Vec<RoomDTO> {
        self.smarthome
            .get_rooms()
            .iter()
            .map(|room| RoomDTO {
                name: room.get_name().to_string(),
            })
            .collect()
    }

    pub async fn get_devices(&self, room_name: &str) -> Vec<DeviceDTO> {
        let result = self.smarthome.get_devices(room_name);
        match result {
            Some(devices) => devices
                .iter()
                .map(|device_name| DeviceDTO {
                    name: device_name.to_string(),
                })
                .collect(),
            None => vec![],
        }
    }

    pub async fn create_report(&self) -> String {
        self.smarthome.create_report(&self.info_provider)
    }
}

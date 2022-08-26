use crate::core::errors::SmartHomeErrors;
use crate::core::traits::SmartHomeProvider;
use crate::database::DatabaseRepository;
use std::collections::HashMap;

use crate::core::{Device, Room};
use crate::provider::InfoProvider;

const SEPARATOR: &str = "\n";

pub struct SmartHome<R: DatabaseRepository> {
    name: String,
    repository: R,
}

impl<R: DatabaseRepository> SmartHome<R> {
    pub fn bulder(name: &str) -> SmartHomeBuilder<R> {
        SmartHomeBuilder {
            name: name.to_string(),
            repository: None,
        }
    }
}

pub struct SmartHomeBuilder<R: DatabaseRepository> {
    name: String,
    repository: Option<R>,
}
impl<R: DatabaseRepository> SmartHomeBuilder<R> {
    pub fn with_repository(mut self, repository: R) -> Self {
        self.repository = Some(repository);
        self
    }
    pub fn build(self) -> SmartHome<R> {
        SmartHome {
            name: self.name,
            repository: self.repository.unwrap(),
        }
    }
}

impl<R: DatabaseRepository> SmartHomeProvider for SmartHome<R> {
    fn add_room(&mut self, room_name: &str) -> u32 {
        let result = self.repository.add_room(room_name);
        result.unwrap()
    }

    fn delete_room(&mut self, room_id: u32) -> Result<(), SmartHomeErrors> {
        self.repository.delete_room(room_id);
        Ok(())
    }
    //TODO unwrap
    fn add_device(&mut self, room_id: u32, device_name: &str) -> u32 {
        let result = self.repository.add_device(room_id, device_name);
        result.unwrap()
    }

    fn delete_device(&mut self, room_id: u32, device_id: u32) -> Result<(), SmartHomeErrors> {
        self.repository.delete_device(room_id, device_id);
        Ok(())
    }

    fn get_rooms(&self) -> Vec<Room> {
        self.repository
            .get_rooms()
            .into_iter()
            .map(|dto| Room::new(dto.id, &dto.name))
            .collect()
    }

    fn get_devices(&self, room_id: u32) -> Vec<Device> {
        self.repository
            .get_devices(room_id)
            .into_iter()
            .map(|dto| Device::new(dto.id, &dto.name))
            .collect()
    }

    fn create_report<T: InfoProvider>(&self, info_provider: &T) -> String {
        let mut report = Vec::new();
        let header = format!("Smart home [{}]:", &self.name);
        report.push(header);

        for room in self.get_rooms() {
            let room_name = &room.name;
            let devices = self.get_devices(room.id);
            for device in devices {
                let result = info_provider.get_device_report(room_name, &device.name);
                report.push(result);
            }
        }

        report.join(SEPARATOR)
    }
}

//TODO Po umolchaniyu ispolzovat etot repository
#[derive(Default)]
struct HashMapRepository {
    room_storage: HashMap<String, String>,
    device_storage: HashMap<String, String>,
}

impl DatabaseRepository for HashMapRepository {
    fn room_exiest(&self, room_name: &str) -> bool {
        todo!()
    }

    fn add_room(
        &mut self,
        room_name: &str,
    ) -> Result<u32, crate::database::errors::DatabaseErrors> {
        todo!()
    }

    fn delete_room(&self, room_id: u32) -> Result<(), crate::database::errors::DatabaseErrors> {
        todo!()
    }

    fn get_rooms(&self) -> Vec<crate::database::models::RoomDTO> {
        todo!()
    }

    fn device_exiest(&self, room_id: u32, device_name: &str) -> bool {
        todo!()
    }

    fn add_device(
        &mut self,
        room_id: u32,
        device_name: &str,
    ) -> Result<u32, crate::database::errors::DatabaseErrors> {
        todo!()
    }

    fn delete_device(
        &mut self,
        room_id: u32,
        device_id: u32,
    ) -> Result<(), crate::database::errors::DatabaseErrors> {
        todo!()
    }

    fn get_devices(&self, room_id: u32) -> Vec<crate::database::models::DeviceDTO> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn should_add_and_delete_rooms() {
    //         let home_name = "My home";
    //         let room_name = "Bedroom";

    //         let mut smart_home = SmartHome::bulder(home_name).build();
    //         smart_home.add_room(room_name);
    //         assert_eq!(smart_home.get_rooms().len(), 1);
    //         let result = smart_home.delete_room(room_name);
    //         assert!(result.is_ok());
    //         let result = smart_home.delete_room(room_name);
    //         assert!(result.is_err());
    //     }

    //     #[test]
    //     fn should_add_and_delete_devices() {
    //         let home_name = "My home";
    //         let device_name = "Socket";
    //         let room_name = "Bedroom";

    //         let mut smart_home = SmartHome::bulder(home_name).build();
    //         smart_home.add_device(room_name, device_name);
    //         let result = smart_home.delete_device(room_name, device_name);
    //         assert!(result.is_ok());
    //         let result = smart_home.delete_device(room_name, device_name);
    //         assert!(result.is_err());
    //     }
}

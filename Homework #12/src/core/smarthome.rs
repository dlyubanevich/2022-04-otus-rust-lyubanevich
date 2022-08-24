use crate::core::errors::SmartHomeErrors;
use std::collections::HashMap;

use crate::core::room::Room;
use crate::provider::InfoProvider;

const SEPARATOR: &str = "\n";

pub struct SmartHome {
    name: String,
    rooms: HashMap<String, Room>,
}

impl SmartHome {
    pub fn bulder() -> SmartHomeBuilder {
        SmartHomeBuilder {
            name: String::new(),
        }
    }
    pub fn add_room(&mut self, room_name: &str) {
        if !self.rooms.contains_key(room_name) {
            self.rooms
                .insert(room_name.to_string(), Room::new(room_name));
        }
    }
    pub fn delete_room(&mut self, room_name: &str) -> Result<(), SmartHomeErrors> {
        if self.rooms.remove(room_name).is_none() {
            return Err(SmartHomeErrors::RoomNotFound(room_name.to_string()));
        }
        Ok(())
    }
    pub fn add_device(&mut self, room_name: &str, device_name: &str) {
        if let Some(room) = self.rooms.get_mut(room_name) {
            room.add_device(device_name);
        } else {
            let mut room = Room::new(room_name);
            room.add_device(device_name);
            self.rooms.insert(room_name.to_string(), room);
        }
    }
    pub fn delete_device(
        &mut self,
        room_name: &str,
        device_name: &str,
    ) -> Result<(), SmartHomeErrors> {
        if let Some(device_is_deleted) = self
            .rooms
            .get_mut(room_name)
            .map(|room| room.delete_device(device_name))
        {
            if !device_is_deleted {
                return Err(SmartHomeErrors::DeviceNotFound(
                    room_name.to_string(),
                    device_name.to_string(),
                ));
            }
        }
        Ok(())
    }
    pub fn get_rooms(&self) -> Vec<&Room> {
        self.rooms.values().collect()
    }
    pub fn get_devices(&self, room_name: &str) -> Option<Vec<&String>> {
        let room = self.rooms.get(room_name);
        room.map(|room| room.get_devices())
    }
    pub fn create_report<T: InfoProvider>(&self, info_provider: &T) -> String {
        let mut report = Vec::new();
        let header = format!("Smart home [{}]:", &self.name);
        report.push(header);

        for room in self.get_rooms() {
            let room_name = room.get_name();
            let devices = room.get_devices();
            for device_name in devices {
                let result = info_provider.get_device_report(room_name, device_name);
                report.push(result);
            }
        }

        report.join(SEPARATOR)
    }
}

pub struct SmartHomeBuilder {
    name: String,
}
impl SmartHomeBuilder {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name.push_str(name);
        self
    }
    pub fn build(self) -> SmartHome {
        SmartHome {
            name: self.name,
            rooms: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_and_delete_rooms() {
        let home_name = "My home";
        let room_name = "Bedroom";

        let mut smart_home = SmartHome::bulder().with_name(home_name).build();
        smart_home.add_room(room_name);
        assert_eq!(smart_home.get_rooms().len(), 1);
        let result = smart_home.delete_room(room_name);
        assert!(result.is_ok());
        let result = smart_home.delete_room(room_name);
        assert!(result.is_err());
    }

    #[test]
    fn should_add_and_delete_devices() {
        let home_name = "My home";
        let device_name = "Socket";
        let room_name = "Bedroom";

        let mut smart_home = SmartHome::bulder().with_name(home_name).build();
        smart_home.add_device(room_name, device_name);
        let result = smart_home.delete_device(room_name, device_name);
        assert!(result.is_ok());
        let result = smart_home.delete_device(room_name, device_name);
        assert!(result.is_err());
    }
}

use std::collections::HashMap;

use crate::core::room::Room;
use crate::provider::InfoProvider;

const SEPARATOR: &str = "\n";

pub struct SmartHome {
    name: String,
    rooms: HashMap<String, Room>,
}

impl SmartHome {
    pub fn new(name: &str) -> Self {
        SmartHome {
            name: name.to_string(),
            rooms: HashMap::new(),
        }
    }
    pub fn add_room(&mut self, room_name: &str) {
        if self.rooms.contains_key(room_name) {
            panic!("There is already added room with name: [{room_name}]");
        }
        self.rooms
            .insert(room_name.to_string(), Room::new(room_name));
    }
    pub fn add_device(&mut self, room_name: &str, device_name: &str) {
        let room = self.rooms.get_mut(room_name);
        match room {
            Some(room) => room.add_device(device_name),
            None => {
                let mut room = Room::new(room_name);
                room.add_device(device_name);
                self.rooms.insert(room_name.to_string(), room);
            }
        };
    }
    pub fn get_rooms(&self) -> Vec<&Room> {
        self.rooms.values().collect()
    }
    pub fn get_devices(&self, room_name: &str) -> &[String] {
        let room = self.rooms.get(room_name);
        match room {
            Some(room) => room.get_devices(),
            _ => panic!("There is already added room with name: [{room_name}]"),
        }
    }
    pub fn create_report<T: InfoProvider>(&self, info_provider: T) -> String {
        let mut report = Vec::new();
        let home_name = &self.name;
        let header = format!("Smart home [{home_name}]:");
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

#[cfg(test)]
mod tests {
    use super::*;

    struct Provider {}
    impl InfoProvider for Provider {
        fn get_device_report(&self, room: &str, device: &str) -> String {
            format!("Room: [{room}], Device: [{device}]")
        }
    }

    #[test]
    fn should_create_report() {
        let home_name = "My home";
        let device_name = "Socket";
        let room_name = "Bedroom";

        let mut smart_home = SmartHome::new(home_name);
        smart_home.add_room(room_name);
        smart_home.add_device(room_name, device_name);

        let result = smart_home.create_report(Provider {});
        let expected =
            format!("Smart home [{home_name}]:\nRoom: [{room_name}], Device: [{device_name}]");
        assert_eq!(result, expected)
    }
}

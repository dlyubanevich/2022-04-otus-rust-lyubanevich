use crate::core::room::Room;
use crate::provider::InfoProvider;

const SEPARATOR: &str = "\n";

pub struct SmartHome {
    name: String,
    rooms: Vec<Room>,
}

impl SmartHome {
    pub fn new(name: &str) -> Self {
        SmartHome {
            name: name.to_string(),
            rooms: Vec::new(),
        }
    }
    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }
    pub fn get_rooms(&self) -> &[Room] {
        self.rooms.as_slice()
    }
    pub fn get_devices<'a>(&self, room: &'a Room) -> &'a [String] {
        room.get_devices()
    }
    pub fn create_report<T: InfoProvider>(&self, info_provider: T) -> String {
        let mut report = Vec::new();
        let home_name = &self.name;
        let header = format!("Smart home [{home_name}]:");
        report.push(header);

        for room in self.get_rooms() {
            let room_name = room.get_name();
            for device_name in self.get_devices(room) {
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
        let mut bedroom = Room::new(room_name);
        bedroom.add_device(device_name);

        let mut smart_home = SmartHome::new(home_name);
        smart_home.add_room(bedroom);

        let result = smart_home.create_report(Provider {});
        let expected =
            format!("Smart home [{home_name}]:\nRoom: [{room_name}], Device: [{device_name}]");
        assert_eq!(result, expected)
    }
}

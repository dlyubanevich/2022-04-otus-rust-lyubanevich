use std::collections::HashSet;

pub struct Room {
    name: String,
    devices: HashSet<String>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Room {
            name: name.to_string(),
            devices: HashSet::new(),
        }
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn add_device(&mut self, name: &str) {
        self.devices.insert(name.to_string());
    }
    pub fn delete_device(&mut self, name: &str) -> bool {
        self.devices.remove(name)
    }
    pub fn get_devices(&self) -> Vec<&String> {
        self.devices.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_collect_unique_device_name() {
        let room_name = "Room";
        let device_name = "Thermometer";
        let mut room = Room::new(room_name);
        room.add_device(device_name);
        room.add_device(device_name);
        assert_eq!(room.get_devices().len(), 1);
    }
}

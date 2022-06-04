pub struct Room {
    name: String,
    devices: Vec<String>,
}

impl Room {
    pub fn new(name: &str) -> Self {
        Room {
            name: name.to_string(),
            devices: Vec::new(),
        }
    }
    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
    pub fn add_device(&mut self, name: &str) {
        if self.device_have_been_added(name) {
            panic!("There is already added device with name: [{name}]");
        }
        self.devices.push(name.to_string());
    }
    pub fn get_devices(&self) -> &[String] {
        self.devices.as_slice()
    }
    fn device_have_been_added(&self, name: &str) -> bool {
        self.devices.iter().any(|device| device.as_str() == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "There is already added device with")]
    fn should_panic_when_trying_add_already_added_device() {
        let room_name = "Room";
        let device_name = "Thermometer";
        let mut room = Room::new(room_name);
        room.add_device(device_name);
        room.add_device(device_name);
    }
}

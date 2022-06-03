
pub struct Room {
    name: String,
    devices: Vec<String>
}

impl Room {
    pub fn new(name: &str) -> Self {
        Room {
            name: name.to_string(),
            devices: Vec::new(),
        }
    }
    pub fn add_device(&mut self, name: &str) {
        self.devices.push(name.to_string());
    }
    pub fn get_devices(&self) -> &[String] {
        self.devices.as_slice()
    }
}
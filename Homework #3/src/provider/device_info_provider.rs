use crate::devices::DeviceStatus;
use crate::devices::Socket;
use crate::devices::Thermometer;
use crate::provider::traits::InfoProvider;

pub struct DeviceInfoProvider<'a> {
    devices: Vec<Device<'a>>,
}

impl<'a> Default for DeviceInfoProvider<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> DeviceInfoProvider<'a> {
    pub fn new() -> Self {
        DeviceInfoProvider {
            devices: Vec::new(),
        }
    }
    pub fn add_device(&mut self, owner: &str, name: &str, device_type: DeviceType<'a>) {
        if self.device_have_been_added(owner, name) {
            panic!(
                "There is already added device with owner: [{}] and name: [{}]",
                owner, name
            );
        }
        let device = Device {
            owner: owner.to_string(),
            name: name.to_string(),
            device_type,
        };
        self.devices.push(device);
    }
    fn device_have_been_added(&self, owner: &str, name: &str) -> bool {
        self.devices
            .iter()
            .any(|device| device.owner == owner && device.name == name)
    }
}

impl<'a> InfoProvider for DeviceInfoProvider<'a> {
    fn get_device_report(&self, room_name: &str, device_name: &str) -> String {
        let result = self
            .devices
            .iter()
            .filter(|device| device.owner == room_name && device.name == device_name)
            .map(|device| match device.device_type {
                DeviceType::Socket(socket) => socket.get_status(),
                DeviceType::Thermometer(thermometer) => thermometer.get_status(),
            })
            .next();

        match result {
            Some(status) => {
                format!("Room: [{room_name}], Device: [{device_name}], Status:{status}")
            }
            None => format!("Room: [{room_name}], Device: [{device_name}], Status: NOT AVAILABLE"),
        }
    }
}

struct Device<'a> {
    owner: String,
    name: String,
    device_type: DeviceType<'a>,
}

pub enum DeviceType<'a> {
    Socket(&'a Socket),
    Thermometer(&'a Thermometer),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "There is already added device with")]
    fn should_panic_when_trying_add_already_added_device() {
        let thermometer = Thermometer::new("Thermometer");
        let mut provider = DeviceInfoProvider::new();
        provider.add_device(
            "Bedroom",
            "Thermometer",
            DeviceType::Thermometer(&thermometer),
        );
        provider.add_device(
            "Bedroom",
            "Thermometer",
            DeviceType::Thermometer(&thermometer),
        );
    }

    #[test]
    fn should_add_device() {
        let device_name = "Socket";
        let socket = Socket::new(device_name);
        let mut provider = DeviceInfoProvider::new();
        assert_eq!(0, provider.devices.len());
        provider.add_device("Bedroom", device_name, DeviceType::Socket(&socket));
        assert_eq!(1, provider.devices.len());
    }
}

use crate::devices::Socket;
use crate::devices::Thermometer;
use crate::provider::errors::ProviderErrors;
use crate::provider::traits::InfoProvider;

pub struct DeviceInfoProvider {
    devices: Vec<Device>,
}

impl Default for DeviceInfoProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceInfoProvider {
    pub fn new() -> Self {
        DeviceInfoProvider {
            devices: Vec::new(),
        }
    }
    pub fn add_device(
        &mut self,
        owner: &str,
        name: &str,
        device_type: DeviceType,
    ) -> Result<(), ProviderErrors> {
        if self.device_have_been_added(owner, name) {
            return Err(ProviderErrors::NotUniqueDeviceByOwner(
                owner.to_string(),
                name.to_string(),
            ));
        }
        let device = Device {
            owner: owner.to_string(),
            name: name.to_string(),
            device_type,
        };
        self.devices.push(device);
        Ok(())
    }
    fn device_have_been_added(&self, owner: &str, name: &str) -> bool {
        self.devices
            .iter()
            .any(|device| device.owner == owner && device.name == name)
    }
}

impl InfoProvider for DeviceInfoProvider {
    fn get_device_report(&self, room_name: &str, device_name: &str) -> String {
        let result = self
            .devices
            .iter()
            .filter(|device| device.owner == room_name && device.name == device_name)
            .map(|device| device.get_status())
            .next();

        match result {
            Some(status) => {
                format!("Room: [{room_name}], Device: [{device_name}], Status:{status}")
            }
            None => format!("Room: [{room_name}], Device: [{device_name}], Status: NOT AVAILABLE"),
        }
    }
}

struct Device {
    owner: String,
    name: String,
    device_type: DeviceType,
}

impl Device {
    fn get_status(&self) -> String {
        match &self.device_type {
            DeviceType::Socket(socket) => socket.to_string(),
            DeviceType::Thermometer(thermometer) => thermometer.to_string(),
        }
    }
}

pub enum DeviceType {
    Socket(Socket),
    Thermometer(Thermometer),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_err_when_trying_add_already_added_device() {
        let device_name = "Thermometer";
        let mut provider = DeviceInfoProvider::new();
        let result = provider.add_device(
            "Bedroom",
            "Thermometer",
            DeviceType::Thermometer(Thermometer::new(device_name)),
        );
        assert!(result.is_ok());
        let result = provider.add_device(
            "Bedroom",
            "Thermometer",
            DeviceType::Thermometer(Thermometer::new(device_name)),
        );
        assert!(result.is_err());
    }

    #[test]
    fn should_add_device() {
        let device_name = "Socket";
        let socket = Socket::new(device_name);
        let mut provider = DeviceInfoProvider::new();
        assert_eq!(0, provider.devices.len());
        let result = provider.add_device("Bedroom", device_name, DeviceType::Socket(socket));
        assert!(result.is_ok());
        assert_eq!(1, provider.devices.len());
    }
}

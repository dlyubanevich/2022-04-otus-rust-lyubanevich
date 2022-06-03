use crate::devices::socket::Socket;
use crate::devices::thermometer::Thermometer;
use crate::info_provider::traits::InfoProvider;

pub struct DeviceInfoProvider<'a>{
    devices: Vec<Device<'a>>
}

impl<'a> DeviceInfoProvider<'a> {
    pub fn new()-> Self{
        DeviceInfoProvider{
            devices: Vec::new(),
        }    
    }
    pub fn add_device(&mut self, owner: &str, name: &str, device_type: DeviceType<'a>) {
        let device = Device{
            owner: owner.to_string(),
            name: name.to_string(),
            device_type: device_type,
        };
        self.devices.push(device);
    }
}

impl<'a> InfoProvider for DeviceInfoProvider<'a> {
    fn get_device_report(&self, room: &str, device: &str) -> String {
        todo!("")  
    }
}

struct Device<'a> {
    owner: String,
    name: String,
    device_type: DeviceType<'a>,
}

pub enum DeviceType<'a>{
    Socket(&'a Socket),
    Thermometer(&'a Thermometer),
}

use crate::core::errors::SmartHomeErrors;
use crate::core::{Device, Room};
use crate::provider::InfoProvider;

pub trait SmartHomeProvider {
    fn add_room(&mut self, room_name: &str) -> u32;
    fn delete_room(&mut self, room_id: u32) -> Result<(), SmartHomeErrors>;
    fn add_device(&mut self, room_id: u32, device_name: &str) -> u32;
    fn delete_device(&mut self, room_id: u32, device_id: u32) -> Result<(), SmartHomeErrors>;
    fn get_rooms(&self) -> Vec<Room>;
    fn get_devices(&self, room_id: u32) -> Vec<Device>;
    fn create_report<T: InfoProvider>(&self, info_provider: &T) -> String;
}

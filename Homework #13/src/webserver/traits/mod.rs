use crate::core::SmartHomeErrors;
use crate::webserver::models::{Device, Report, Room};

pub trait SmartHomeRepository {
    fn get_rooms(&self) -> Vec<Room>;
    fn get_devices(&self, room_id: u32) -> Vec<Device>;
    fn add_room(&mut self, room_name: String) -> Result<(), SmartHomeErrors>;
    fn add_device(&mut self, room_id: u32, device_name: String) -> Result<(), SmartHomeErrors>;
    fn delete_room(&mut self, room_id: u32) -> Result<(), SmartHomeErrors>;
    fn delete_device(&mut self, room_id: u32, device_id: u32) -> Result<(), SmartHomeErrors>;
    fn create_report(&self) -> Report;
}

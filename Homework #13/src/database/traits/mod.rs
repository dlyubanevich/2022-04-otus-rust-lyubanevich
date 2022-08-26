use crate::database::errors::DatabaseErrors;

use super::models::{DeviceDTO, RoomDTO};

pub trait DatabaseRepository {
    fn room_exiest(&self, room_name: &str) -> bool;
    fn add_room(&mut self, room_name: &str) -> Result<u32, DatabaseErrors>;
    fn delete_room(&self, room_id: u32) -> Result<(), DatabaseErrors>;
    fn get_rooms(&self) -> Vec<RoomDTO>;
    fn device_exiest(&self, room_id: u32, device_name: &str) -> bool;
    fn add_device(&mut self, room_id: u32, device_name: &str) -> Result<u32, DatabaseErrors>;
    fn delete_device(&mut self, room_id: u32, device_id: u32) -> Result<(), DatabaseErrors>;
    fn get_devices(&self, room_id: u32) -> Vec<DeviceDTO>;
}

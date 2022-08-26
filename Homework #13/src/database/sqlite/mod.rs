use crate::database::DatabaseRepository;

pub struct SqliteRepository {}
impl DatabaseRepository for SqliteRepository {
    fn room_exiest(&self, room_name: &str) -> bool {
        todo!()
    }

    fn add_room(&mut self, room_name: &str) -> Result<u32, super::errors::DatabaseErrors> {
        todo!()
    }

    fn delete_room(&self, room_id: u32) -> Result<(), super::errors::DatabaseErrors> {
        todo!()
    }

    fn get_rooms(&self) -> Vec<super::models::RoomDTO> {
        todo!()
    }

    fn device_exiest(&self, room_id: u32, device_name: &str) -> bool {
        todo!()
    }

    fn add_device(
        &mut self,
        room_id: u32,
        device_name: &str,
    ) -> Result<u32, super::errors::DatabaseErrors> {
        todo!()
    }

    fn delete_device(
        &mut self,
        room_id: u32,
        device_id: u32,
    ) -> Result<(), super::errors::DatabaseErrors> {
        todo!()
    }

    fn get_devices(&self, room_id: u32) -> Vec<super::models::DeviceDTO> {
        todo!()
    }
}

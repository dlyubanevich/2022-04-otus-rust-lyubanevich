use crate::smarthome::room::Room;

pub struct SmartHome {
    name: String,
    rooms: Vec<Room>,
}

impl SmartHome {
    pub fn new(name: &str) -> Self {
        SmartHome{
            name: name.to_string(),
            rooms: Vec::new(),
        }   
    }
    pub fn add_room(&mut self, room: Room) {
        self.rooms.push(room);
    }
    pub fn create_report<T: >(&self, info_provider: T) -> String {
        String::from("")
    }
    pub fn get_rooms(&self) -> &[Room]{
        self.rooms.as_slice()
    }
}
use std::fmt::Formatter;

#[derive(Debug)]
pub enum SmartHomeErrors {
    RoomNotFound(String),
    DeviceNotFound(String, String),
}

impl std::fmt::Display for SmartHomeErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SmartHomeErrors::RoomNotFound(room_name) => {
                write!(f, "There is no room with name: [{}]", room_name)
            }
            SmartHomeErrors::DeviceNotFound(room_name, device_name) => {
                write!(
                    f,
                    "There is no device: [{}] in room with name: [{}]",
                    device_name, room_name
                )
            }
        }
    }
}
impl std::error::Error for SmartHomeErrors {}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmartHomeErrors {
    #[error("here is no room with name: {0}")]
    RoomNotFound(String),
    #[error("There is no device: [{0}] in room with name: [{1}]")]
    DeviceNotFound(String, String),
}

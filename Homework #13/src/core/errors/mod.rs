use thiserror::Error;

#[derive(Debug, Error)]
pub enum SmartHomeErrors {
    #[error("There is no room with name [{0}]")]
    RoomNotFound(String),
    #[error("Error on creating room [{0}]: [{1}]")]
    RoomCreateError(String, String),
    #[error("Error on delete room [{0}]: [{1}]")]
    RoomDeleteError(String, String),
    #[error("There is no device: [{0}] in room with name: [{1}]")]
    DeviceNotFound(String, String),
    #[error("Error on adding device [{0}]: [{1}]")]
    DeviceCreateError(String, String),
    #[error("Error on delete device [{0}]: [{1}]")]
    DeviceDeleteError(String, String),
}

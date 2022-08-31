use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseErrors {
    #[error("Connection error: [{0}]")]
    ConnectionError(String),
    #[error("Request error: [{0}]")]
    RequestError(String),
    #[error("There is no room with id [{0}]")]
    RoomNotFound(u32),
    #[error("There is no device by id [{0}] in room with id [{1}]")]
    DeviceNotFound(u32, u32),
    #[error("Room with name [{0}] is already exiest!")]
    RoomIsAlreadyExiest(String),
    #[error("Device with name [{1}] is already exiest in room with id [{0}]!")]
    DeviceIsAlreadyExiest(u32, String),
}

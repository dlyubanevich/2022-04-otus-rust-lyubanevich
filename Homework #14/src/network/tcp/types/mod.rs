use crate::network::tcp::errors::{
    BindError, ConnectionError, ReceiveError, RequestError, SendError,
};

pub type SendResult = Result<(), SendError>;
pub type ReceiveResult = Result<String, ReceiveError>;
pub type ConnectionResult<T> = Result<T, ConnectionError>;
pub type BindResult<T> = Result<T, BindError>;
pub type RequestResult = Result<String, RequestError>;

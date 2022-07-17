use crate::network::common::errors::{
    BindError, ConnectionError, ReceiveError, RequestError, SendError,
};

pub type SendResult = Result<(), SendError>;
pub type ReceiveResult<T> = Result<T, ReceiveError>;
pub type ConnectionResult<T> = Result<T, ConnectionError>;
pub type BindResult<T> = Result<T, BindError>;
pub type RequestResult<T> = Result<T, RequestError>;

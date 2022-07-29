use crate::network::common::errors::ReceiveError;
use crate::network::common::types::{ReceiveResult, SendResult};
use std::io::{Read, Write};

pub fn send_string<W: Write, D: AsRef<str>>(mut writer: W, data: D) -> SendResult {
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes)?;
    writer.write_all(bytes)?;
    Ok(())
}

pub fn receive_string<R: Read>(mut reader: R) -> ReceiveResult<String> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| ReceiveError::BadEncoding)
}

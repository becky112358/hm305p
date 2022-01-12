use std::io;

use thiserror::Error;

use crate::common::MESSAGE_LENGTH;

#[derive(Debug, Error)]
pub enum Hm305pError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    SerialPort(#[from] serialport::Error),
    #[error("Power supply CRC is invalid")]
    InvalidCrc,
    #[error("Unexpected response from power supply: {0:?}")]
    UnexpectedResponse([u8; MESSAGE_LENGTH]),
}

impl From<Hm305pError> for io::Error {
    fn from(err: Hm305pError) -> io::Error {
        match err {
            Hm305pError::Io(e) => e,
            _ => io::Error::new(io::ErrorKind::Other, err),
        }
    }
}

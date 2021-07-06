use std::io;

#[derive(Debug)]
pub enum Hm305pError {
    IoError(io::Error),
    SerialPortError(serialport::Error),
    LocalError(Error),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub description: String,
}

#[derive(Debug)]
pub enum ErrorKind {
    InvalidCrc,
    UnexpectedResponse,
}

impl Hm305pError {
    pub fn new(kind: ErrorKind, description: &str) -> Self {
        Hm305pError::LocalError(Error {
            kind,
            description: description.to_string(),
        })
    }
}

impl From<io::Error> for Hm305pError {
    fn from(error: io::Error) -> Self {
        Hm305pError::IoError(error)
    }
}

impl From<serialport::Error> for Hm305pError {
    fn from(error: serialport::Error) -> Self {
        Hm305pError::SerialPortError(error)
    }
}

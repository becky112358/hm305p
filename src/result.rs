use std::io;

pub enum Hm305pError {
    IoError(io::Error),
    SerialPortError(serialport::Error),
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

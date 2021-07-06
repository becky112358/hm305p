use crate::common::{
    u16_get_u8_high, u16_get_u8_low, INDEX_ADDRESS, INDEX_READ_WRITE, MESSAGE_LENGTH,
    READ_RESPONSE_LENGTH, VALUE_ADDRESS, VALUE_READ, VALUE_WRITE
};
use crate::crc::compute;
use crate::result::{Hm305pError, ErrorKind};

pub fn init_read() -> [u8; MESSAGE_LENGTH] {
    let mut message = [0; MESSAGE_LENGTH];
    message[INDEX_ADDRESS] = VALUE_ADDRESS;
    message[INDEX_READ_WRITE] = VALUE_READ;
    message[4] = 0x00;
    message[5] = 0x01;

    message
}

pub fn init_write() -> [u8; MESSAGE_LENGTH] {
    let mut message = [0; MESSAGE_LENGTH];
    message[INDEX_ADDRESS] = VALUE_ADDRESS;
    message[INDEX_READ_WRITE] = VALUE_WRITE;

    message
}

pub fn verify_read(response: [u8; MESSAGE_LENGTH]) -> Result<(), Hm305pError> {
    if response[INDEX_ADDRESS] != VALUE_ADDRESS
        || response[INDEX_READ_WRITE] != VALUE_READ
        || response[2] != 2 {
        return Err(Hm305pError::new(ErrorKind::UnexpectedResponse, "Unexpected response from power supply"));
    }

    let crc = compute(&response, READ_RESPONSE_LENGTH);

    if response[5] != u16_get_u8_low(crc) || response[6] != u16_get_u8_high(crc) {
        return Err(Hm305pError::new(ErrorKind::InvalidCrc, "Power supply CRC is invalid"));
    }

    Ok(())
}

#[cfg(test)]
#[path = "./test_message.rs"]
mod test_message;

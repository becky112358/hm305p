
use crate::common::MESSAGE_LENGTH;

fn crc_fill(message: &mut [u8; MESSAGE_LENGTH]) {

}

#[cfg(test)]
#[path = "./test_crc.rs"]
mod test_crc;

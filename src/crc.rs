
use crate::common::{u16_get_u8_high, u16_get_u8_low, MESSAGE_LENGTH};

const CRC_LENGTH: usize = 2;

pub fn fill(message: &mut [u8; MESSAGE_LENGTH]) {
    let mut crc = 0xffff;

    for i in 0..(message.len() - CRC_LENGTH) {
        crc ^= message[i] as u16;

        for _ in 0..8 {
            if (crc & 0x0001) != 0 {
                crc >>= 1;
                crc ^= 0xa001;
            } else {
                crc >>= 1;
            }
        }
    }

    message[6] = u16_get_u8_low(crc);
    message[7] = u16_get_u8_high(crc);
}

#[cfg(test)]
#[path = "./test_crc.rs"]
mod test_crc;

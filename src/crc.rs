use crate::common::{u16_get_u8_high, u16_get_u8_low, MESSAGE_LENGTH};

const CRC_LENGTH: usize = 2;

pub fn fill(message: &mut [u8; MESSAGE_LENGTH]) {
    let crc = compute(message, MESSAGE_LENGTH);

    message[6] = u16_get_u8_low(crc);
    message[7] = u16_get_u8_high(crc);
}

pub fn compute(message: &[u8; MESSAGE_LENGTH], length: usize) -> u16 {
    let mut crc = 0xffff;

    for item in message.iter().take(length - CRC_LENGTH) {
        crc ^= *item as u16;

        for _ in 0..8 {
            if (crc & 0x0001) != 0 {
                crc >>= 1;
                crc ^= 0xa001;
            } else {
                crc >>= 1;
            }
        }
    }

    crc
}

#[cfg(test)]
#[path = "./test_crc.rs"]
mod test_crc;

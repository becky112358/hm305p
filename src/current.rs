use crate::common::{u16_get_u8_high, u16_get_u8_low, u8_high_low_get_u16, Index, MESSAGE_LENGTH};

pub fn get(response: [u8; MESSAGE_LENGTH]) -> u16 {
    u8_high_low_get_u16(response[3], response[4])
}

pub fn set(current_ma: u16, message: &mut [u8; MESSAGE_LENGTH]) {
    message[Index::SetValueHigh as usize] = u16_get_u8_high(current_ma);
    message[Index::SetValueLow as usize] = u16_get_u8_low(current_ma);
}

#[cfg(test)]
#[path = "./test_current.rs"]
mod test_current;

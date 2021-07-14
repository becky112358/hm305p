
use crate::common::{
    u8_high_low_get_u16, u16_get_u8_high, u16_get_u8_low, INDEX_SET_VALUE_HIGH, INDEX_SET_VALUE_LOW, MESSAGE_LENGTH,
};


pub fn get(response: [u8; MESSAGE_LENGTH]) -> u16 {
    u8_high_low_get_u16(response[3], response[4])
}

pub fn set(current_ma: u16, message: &mut [u8; MESSAGE_LENGTH]) {
    message[INDEX_SET_VALUE_HIGH] = u16_get_u8_high(current_ma);
    message[INDEX_SET_VALUE_LOW] = u16_get_u8_low(current_ma);
}

#[cfg(test)]
#[path = "./test_current.rs"]
mod test_current;

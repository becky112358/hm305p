
use crate::common::{
    u16_get_u8_high, u16_get_u8_low, INDEX_CURRENT_HIGH, INDEX_CURRENT_LOW, MESSAGE_LENGTH,
};


pub fn set(current_ma: u16, message: &mut [u8; MESSAGE_LENGTH]) {
    message[INDEX_CURRENT_HIGH] = u16_get_u8_high(current_ma);
    message[INDEX_CURRENT_LOW] = u16_get_u8_low(current_ma);
}

#[cfg(test)]
#[path = "./test_current.rs"]
mod test_current;

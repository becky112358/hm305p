
pub const MESSAGE_LENGTH: usize = 8;

pub fn u16_get_u8_high(input: u16) -> u8 {
    (input >> 8) as u8
}

pub fn u16_get_u8_low(input: u16) -> u8 {
    (input & 0x00ff) as u8
}

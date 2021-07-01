pub const MESSAGE_LENGTH: usize = 8;

pub const INDEX_ADDRESS: usize = 0;
pub const INDEX_READ_WRITE: usize = 1;
pub const INDEX_CONTROL_COMMAND_0: usize = 2;
pub const INDEX_CONTROL_COMMAND_1: usize = 3;
pub const INDEX_CURRENT_HIGH: usize = 4;
pub const INDEX_CURRENT_LOW: usize = 5;
pub const INDEX_VOLTAGE_HIGH: usize = 4;
pub const INDEX_VOLTAGE_LOW: usize = 5;

pub const VALUE_ADDRESS: u8 = 0x01;
pub const VALUE_READ: u8 = 0x03;
pub const VALUE_WRITE: u8 = 0x06;
pub const VALUE_SET_VOLTAGE_0: u8 = 0x00;
pub const VALUE_SET_VOLTAGE_1: u8 = 0x30;
pub const VALUE_SET_CURRENT_0: u8 = 0x00;
pub const VALUE_SET_CURRENT_1: u8 = 0x31;

pub fn u16_get_u8_high(input: u16) -> u8 {
    (input >> 8) as u8
}

pub fn u16_get_u8_low(input: u16) -> u8 {
    (input & 0x00ff) as u8
}

#[cfg(test)]
#[path = "./test_common.rs"]
mod test_common;

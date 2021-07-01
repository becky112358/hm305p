
use crate::common::{u16_get_u8_high, u16_get_u8_low, MESSAGE_LENGTH};

const VOLTAGE_INDEX_HIGH: usize = 4;
const VOLTAGE_INDEX_LOW: usize = 5;

pub fn set(voltage_mv: u16, message: &mut [u8; MESSAGE_LENGTH]) {
    let command_voltage_value_mv_x10 = voltage_mv / 10;

    message[VOLTAGE_INDEX_LOW] = u16_get_u8_low(command_voltage_value_mv_x10);
    message[VOLTAGE_INDEX_HIGH] = u16_get_u8_high(command_voltage_value_mv_x10);
}

#[cfg(test)]
#[path = "./test_voltage.rs"]
mod test_voltage;

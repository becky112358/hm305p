use crate::common::{
    u8_high_low_get_u16, u16_get_u8_high, u16_get_u8_low, INDEX_SET_VALUE_HIGH,
    INDEX_SET_VALUE_LOW, MESSAGE_LENGTH,
};

pub fn get(response: [u8; MESSAGE_LENGTH]) -> u16 {
    let voltage_value_mv_by10 = u8_high_low_get_u16(response[3], response[4]);

    voltage_value_mv_by10 * 10
}

pub fn set(voltage_mv: u16, message: &mut [u8; MESSAGE_LENGTH]) {
    let command_voltage_value_mv_by10 = voltage_mv / 10;

    message[INDEX_SET_VALUE_HIGH] = u16_get_u8_high(command_voltage_value_mv_by10);
    message[INDEX_SET_VALUE_LOW] = u16_get_u8_low(command_voltage_value_mv_by10);
}

#[cfg(test)]
#[path = "./test_voltage.rs"]
mod test_voltage;

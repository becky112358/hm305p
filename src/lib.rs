mod common;
use crate::common::{
    INDEX_ADDRESS, INDEX_CONTROL_COMMAND_0, INDEX_CONTROL_COMMAND_1, INDEX_READ_WRITE,
    MESSAGE_LENGTH, VALUE_ADDRESS, VALUE_SET_VOLTAGE_0, VALUE_SET_VOLTAGE_1, VALUE_WRITE,
};
mod crc;
mod port;
mod voltage;

pub fn set_voltage(voltage_mv: u16) {
    let mut port = port::connect().unwrap();
    let mut message = [0; MESSAGE_LENGTH];
    message[INDEX_ADDRESS] = VALUE_ADDRESS;
    message[INDEX_READ_WRITE] = VALUE_WRITE;
    message[INDEX_CONTROL_COMMAND_0] = VALUE_SET_VOLTAGE_0;
    message[INDEX_CONTROL_COMMAND_1] = VALUE_SET_VOLTAGE_1;
    voltage::set(voltage_mv, &mut message);
    crc::fill(&mut message);
    port.write(&message).unwrap();
}

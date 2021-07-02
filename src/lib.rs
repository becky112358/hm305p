mod common;
use crate::common::{
    INDEX_ADDRESS, INDEX_CONTROL_COMMAND_0, INDEX_CONTROL_COMMAND_1, INDEX_READ_WRITE,
    MESSAGE_LENGTH, VALUE_ADDRESS, VALUE_SET_CURRENT_0, VALUE_SET_CURRENT_1,
    VALUE_SET_VOLTAGE_0, VALUE_SET_VOLTAGE_1, VALUE_WRITE,
};
mod crc;
mod current;
mod port;
mod result;
pub use result::Hm305pError;
mod voltage;

pub fn set_current(current_ma: u16) -> Result<(), Hm305pError> {
    let mut port = port::connect()?;
    let mut message = [0; MESSAGE_LENGTH];
    message[INDEX_ADDRESS] = VALUE_ADDRESS;
    message[INDEX_READ_WRITE] = VALUE_WRITE;
    message[INDEX_CONTROL_COMMAND_0] = VALUE_SET_CURRENT_0;
    message[INDEX_CONTROL_COMMAND_1] = VALUE_SET_CURRENT_1;
    current::set(current_ma, &mut message);
    crc::fill(&mut message);
    port.write(&message)?;

    Ok(())
}

pub fn set_voltage(voltage_mv: u16) -> Result<(), Hm305pError> {
    let mut port = port::connect()?;
    let mut message = [0; MESSAGE_LENGTH];
    message[INDEX_ADDRESS] = VALUE_ADDRESS;
    message[INDEX_READ_WRITE] = VALUE_WRITE;
    message[INDEX_CONTROL_COMMAND_0] = VALUE_SET_VOLTAGE_0;
    message[INDEX_CONTROL_COMMAND_1] = VALUE_SET_VOLTAGE_1;
    voltage::set(voltage_mv, &mut message);
    crc::fill(&mut message);
    port.write(&message)?;

    Ok(())
}

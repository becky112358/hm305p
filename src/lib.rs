mod common;
use crate::common::{
    INDEX_CONTROL_COMMAND_0, INDEX_CONTROL_COMMAND_1, VALUE_GET_VOLTAGE_0, VALUE_GET_VOLTAGE_1,
    VALUE_SET_CURRENT_0, VALUE_SET_CURRENT_1, VALUE_SET_VOLTAGE_0, VALUE_SET_VOLTAGE_1,
};
mod crc;
mod current;
mod message;
mod port;
mod result;
pub use result::Hm305pError;
mod voltage;

pub fn get_voltage_mv() -> Result<u16, Hm305pError> {
    let mut port = port::connect()?;
    let mut message = message::init_read();
    message[INDEX_CONTROL_COMMAND_0] = VALUE_GET_VOLTAGE_0;
    message[INDEX_CONTROL_COMMAND_1] = VALUE_GET_VOLTAGE_1;
    crc::fill(&mut message);
    port.write(&message)?;
    let response = port::read(&mut port)?;
    message::verify_read(response)?;
    let voltage_mv = voltage::get(response);

    Ok(voltage_mv)
}

pub fn set_current(current_ma: u16) -> Result<(), Hm305pError> {
    let mut port = port::connect()?;
    let mut message = message::init_write();
    message[INDEX_CONTROL_COMMAND_0] = VALUE_SET_CURRENT_0;
    message[INDEX_CONTROL_COMMAND_1] = VALUE_SET_CURRENT_1;
    current::set(current_ma, &mut message);
    crc::fill(&mut message);
    port.write(&message)?;
    let response = port::read(&mut port)?;
    message::verify_write(response)?;

    Ok(())
}

pub fn set_voltage(voltage_mv: u16) -> Result<(), Hm305pError> {
    let mut port = port::connect()?;
    let mut message = message::init_write();
    message[INDEX_CONTROL_COMMAND_0] = VALUE_SET_VOLTAGE_0;
    message[INDEX_CONTROL_COMMAND_1] = VALUE_SET_VOLTAGE_1;
    voltage::set(voltage_mv, &mut message);
    crc::fill(&mut message);
    port.write(&message)?;
    let response = port::read(&mut port)?;
    message::verify_write(response)?;

    Ok(())
}

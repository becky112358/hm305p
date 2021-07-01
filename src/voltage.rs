
use crate::common::MESSAGE_LENGTH;

pub fn set(voltage_mv: u16, message: &mut [u8; MESSAGE_LENGTH]) {

}

#[cfg(test)]
#[path = "./test_voltage.rs"]
mod test_voltage;

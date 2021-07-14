mod common;
use crate::common::{Action, Request};
mod crc;
mod current;
mod message;
mod port;
mod result;
pub use result::Hm305pError;
mod voltage;

pub fn get_current_ma() -> Result<u16, Hm305pError> {
    message::send_and_receive(Request::Read(Action::CurrentmA))
}

pub fn get_voltage_mv() -> Result<u16, Hm305pError> {
    message::send_and_receive(Request::Read(Action::VoltagemV))
}

pub fn set_current(current_ma: u16) -> Result<(), Hm305pError> {
    let _ = message::send_and_receive(Request::Write((Action::CurrentmA, current_ma)))?;

    Ok(())
}

pub fn set_voltage(voltage_mv: u16) -> Result<(), Hm305pError> {
    let _ = message::send_and_receive(Request::Write((Action::VoltagemV, voltage_mv)))?;

    Ok(())
}

pub fn switch_on() -> Result<(), Hm305pError> {
    let _ = message::send_and_receive(Request::Write((Action::OnOff, 0x0001)))?;

    Ok(())
}

pub fn switch_off() -> Result<(), Hm305pError> {
    let _ = message::send_and_receive(Request::Write((Action::OnOff, 0x0000)))?;

    Ok(())
}

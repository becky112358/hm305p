mod common;
use crate::common::{Action, Request};
mod crc;
mod current;
mod message;
mod port;
mod result;
pub use result::Hm305pError;
mod voltage;

/// Get the instantaneous current consumption, in mA
pub fn get_current_ma() -> Result<u16, Hm305pError> {
    message::send_and_receive(Request::Read(Action::CurrentmA))
}

/// Get the instantaneous voltage, in mV
/// # Accuracy
/// 10mV
pub fn get_voltage_mv() -> Result<u16, Hm305pError> {
    message::send_and_receive(Request::Read(Action::VoltagemV))
}

/// Set the current limit, in mA
pub fn set_current_ma(current_ma: u16) -> Result<(), Hm305pError> {
    let _ = message::send_and_receive(Request::Write((Action::CurrentmA, current_ma)))?;

    Ok(())
}

/// Set the voltage, in mV
/// # Accuracy
/// 10mV, rounded down to the nearest 10mV
/// # Note
/// The driver reports success before completing the task, and can take a few seconds to
/// complete the task.
pub fn set_voltage_mv(voltage_mv: u16) -> Result<(), Hm305pError> {
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

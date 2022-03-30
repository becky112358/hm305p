use crate::common::{
    u16_get_u8_high, u16_get_u8_low, Action, Index, Request, MESSAGE_LENGTH, READ_RESPONSE_LENGTH,
    VALUE_ADDRESS, VALUE_READ, VALUE_WRITE,
};
use crate::crc;
use crate::current;
use crate::port;
use crate::result::Hm305pError;
use crate::voltage;

pub fn send_and_receive(request: Request) -> Result<u16, Hm305pError> {
    let mut port = port::connect()?;
    let mut message = [0; MESSAGE_LENGTH];
    message[Index::Address as usize] = VALUE_ADDRESS;
    message[Index::ReadWrite as usize] = match request {
        Request::Read(_) => VALUE_READ,
        Request::Write(_) => VALUE_WRITE,
    };

    match request {
        Request::Read(Action::CurrentmA) => {
            message[Index::ControlCommand0 as usize] = 0x00;
            message[Index::ControlCommand1 as usize] = 0x11;
        }
        Request::Read(Action::VoltagemV) => {
            message[Index::ControlCommand0 as usize] = 0x00;
            message[Index::ControlCommand1 as usize] = 0x10;
        }
        Request::Write((Action::CurrentmA, _)) => {
            message[Index::ControlCommand0 as usize] = 0x00;
            message[Index::ControlCommand1 as usize] = 0x31;
        }
        Request::Write((Action::VoltagemV, _)) => {
            message[Index::ControlCommand0 as usize] = 0x00;
            message[Index::ControlCommand1 as usize] = 0x30;
        }
        Request::Write((Action::OnOff, _)) => {
            message[Index::ControlCommand0 as usize] = 0x00;
            message[Index::ControlCommand1 as usize] = 0x01;
        }
        _ => unimplemented!("Option has not yet been implemented"),
    }

    match request {
        Request::Read(_) => {
            message[4] = 0x00;
            message[5] = 0x01;
        }
        Request::Write((Action::CurrentmA, value)) => current::set(value, &mut message),
        Request::Write((Action::OnOff, value)) => {
            message[Index::SetValueLow as usize] = value as u8
        }
        Request::Write((Action::VoltagemV, value)) => voltage::set(value, &mut message),
    }

    crc::fill(&mut message);
    port.write_all(&message)?;
    let response = port::read(&mut port)?;

    match request {
        Request::Read(_) => verify_read(response)?,
        Request::Write(_) => verify_write(response)?,
    }

    match request {
        Request::Read(Action::CurrentmA) => {
            let current_ma = current::get(response);
            Ok(current_ma)
        }
        Request::Read(Action::VoltagemV) => {
            let voltage_mv = voltage::get(response);
            Ok(voltage_mv)
        }
        Request::Read(_) => unimplemented!("Option has not yet been implemented"),
        Request::Write(_) => Ok(0),
    }
}

fn verify_read(response: [u8; MESSAGE_LENGTH]) -> Result<(), Hm305pError> {
    if response[Index::Address as usize] != VALUE_ADDRESS
        || response[Index::ReadWrite as usize] != VALUE_READ
        || response[2] != 2
    {
        return Err(Hm305pError::UnexpectedResponse(response));
    }

    let crc = crc::compute(&response, READ_RESPONSE_LENGTH);

    if response[5] != u16_get_u8_low(crc) || response[6] != u16_get_u8_high(crc) {
        return Err(Hm305pError::InvalidCrc);
    }

    Ok(())
}

fn verify_write(response: [u8; MESSAGE_LENGTH]) -> Result<(), Hm305pError> {
    if response[Index::Address as usize] != VALUE_ADDRESS
        || response[Index::ReadWrite as usize] != VALUE_WRITE
        || response[2] != 0
    {
        return Err(Hm305pError::UnexpectedResponse(response));
    }

    let crc = crc::compute(&response, MESSAGE_LENGTH);

    if response[6] != u16_get_u8_low(crc) || response[7] != u16_get_u8_high(crc) {
        return Err(Hm305pError::InvalidCrc);
    }

    Ok(())
}

#[cfg(test)]
#[path = "./test_message.rs"]
mod test_message;

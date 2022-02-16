use std::time::Duration;

use serialport::{Error, ErrorKind, Result, SerialPort, SerialPortType};

use crate::common::MESSAGE_LENGTH;

const MANUFACTURER_0: &str = "wch.cn";
const MANUFACTURER_1: &str = "QinHeng Electronics";
const PRODUCT: &str = "CH340";

const BAUD_RATE: u32 = 9600;
const TIMEOUT_S: u64 = 1;

pub fn connect() -> Result<Box<dyn SerialPort>> {
    let mut result = Error::new(ErrorKind::NoDevice, "Cannot find power supply");

    let ports_visible = serialport::available_ports()?;

    for port_visible in ports_visible {
        match &port_visible.port_type {
            SerialPortType::UsbPort(usb_port_info) => {
                match (&usb_port_info.manufacturer, &usb_port_info.product) {
                    (Some(manufacturer), Some(product)) => {
                        if !manufacturer.eq(MANUFACTURER_0) && !manufacturer.eq(MANUFACTURER_1) {
                            continue;
                        }
                        if !product.contains(PRODUCT) {
                            continue;
                        }
                    }
                    _ => continue,
                }
            }
            _ => continue,
        }

        match serialport::new(&port_visible.port_name, BAUD_RATE)
            .timeout(Duration::from_secs(TIMEOUT_S))
            .open()
        {
            Ok(port) => return Ok(port),
            Err(err) => result = err,
        }
    }

    Err(result)
}

pub fn read(port: &mut Box<dyn SerialPort>) -> Result<[u8; MESSAGE_LENGTH]> {
    let mut output = [0; MESSAGE_LENGTH];
    let _ = port.read(&mut output)?;

    Ok(output)
}

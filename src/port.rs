use std::time::Duration;

use serialport::{ClearBuffer, Error, ErrorKind, Result, SerialPort, SerialPortType};

use crate::common::MESSAGE_LENGTH;

const MANUFACTURER_0: &str = "wch.cn";
const MANUFACTURER_1: &str = "QinHeng Electronics";

const PRODUCT: &str = "CH340";

// see http://www.linux-usb.org/usb.ids
#[cfg(target_os = "linux")]
const VENDOR_ID: u16 = 0x1A86;
#[cfg(target_os = "linux")]
const PRODUCT_ID_1: u16 = 0x7522;
#[cfg(target_os = "linux")]
const PRODUCT_ID_2: u16 = 0x7523;

const BAUD_RATE: u32 = 9600;
const TIMEOUT_S: u64 = 1;

pub fn connect() -> Result<Box<dyn SerialPort>> {
    let mut result = Error::new(ErrorKind::NoDevice, "Cannot find power supply");

    let ports_visible = serialport::available_ports()?;
    for port_visible in ports_visible {
        match &port_visible.port_type {
            SerialPortType::UsbPort(usb_port_info) => {
                #[cfg(target_os = "linux")]
                if usb_port_info.vid != VENDOR_ID
                    && usb_port_info.pid != PRODUCT_ID_1
                    && usb_port_info.pid != PRODUCT_ID_2
                {
                    continue;
                }

                #[cfg(not(target_os = "linux"))]
                match (&usb_port_info.manufacturer, &usb_port_info.product) {
                    (Some(manufacturer), Some(product)) => {
                        if !manufacturer.eq(MANUFACTURER_0)
                            && !manufacturer.eq(MANUFACTURER_1)
                            && !product.contains(PRODUCT)
                        {
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
            Ok(port) => {
                port.clear(ClearBuffer::All)?;
                return Ok(port)
            },
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

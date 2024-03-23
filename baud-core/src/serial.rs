use serialport::{available_ports, SerialPortType};
use std::error::Error;

pub struct SerialPortInfo {
    pub name: String,  // System name for the port
    pub port_type: PortType,
}

pub enum PortType {
    Usb(UsbPortInfo),
    Bluetooth,
    Pci,
    Unknown,
}

pub struct UsbPortInfo {
    pub vid: String,
    pub pid: String,
    pub serial_number: Option<String>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
}

pub fn list_available_ports() -> Result<Vec<SerialPortInfo>, Box<dyn Error>> {
    let ports = available_ports()?;
    let mut serial_ports = Vec::new();

    for port in ports {
        let port_type = match port.port_type {
            SerialPortType::UsbPort(usb_info) => {
                PortType::Usb(UsbPortInfo {
                    vid: format!("{:04x}", usb_info.vid),
                    pid: format!("{:04x}", usb_info.pid), 
                    serial_number: usb_info.serial_number,
                    manufacturer: usb_info.manufacturer,
                    product: usb_info.product,
                })
            }
            SerialPortType::PciPort => PortType::Pci,
            SerialPortType::BluetoothPort => PortType::Bluetooth,
            SerialPortType::Unknown => PortType::Unknown,
        };

        serial_ports.push(SerialPortInfo {
            name: port.port_name,
            port_type,
        });
    }

    Ok(serial_ports)
}

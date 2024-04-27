use clap:: ArgMatches;
use baud_core::serial::{PortType, list_available_ports};
use baud_core::connection::SerialConnection;
use tokio::runtime;

// TODO: 1. Make list_ports differentiate between ports that refer to the same physical port. Don't
// repeat unless the user specifies the --all flag.
// TODO: 2. Test writing to serial port and see how to make it better.
// TODO: 3. Convert data bytes to decimal format and display it in the output without any
// additional text.
// TODO: 4. Add a header with colour when a serial port connection is opened.
// TODO: 5. Add a hacker mode that displays matrix rain when a serial port connection is opened.
// TODO: 6. Add a work mode that displays random data to impreess the boss

/// Handles the command line arguments and executes the appropriate command.
pub fn handle_matches(matches: ArgMatches) {
    match matches.subcommand() {
        Some(("list", _sub_matches)) => list_ports(),
        Some(("open", sub_matches)) => open_port(sub_matches),
        None => println!("No command provided"),
        _ => println!("No command provided"),
    }
}

/// Opens a serial port and reads data from it.
fn open_port(sub_matches: &ArgMatches) {
    let port_name = sub_matches.get_one::<String>("port").expect("Port name is required");
    let baud_rate: u32 = sub_matches.get_one::<String>("baud")
        .expect("Default of 9600 used since baud rate is required")
        .parse()
        .expect("Baud rate must be a number");
    let rt = runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut connection = SerialConnection::connect(port_name, baud_rate).await.unwrap();
        loop {
            let data = connection.read_data().await.unwrap();
            if !data.is_empty() {
                println!("Received data: {:?}", data);
            }
        }
    });
}

/// Lists available serial ports.
fn list_ports() {
    let ports = list_available_ports().unwrap();
    for port in ports {
        match port.port_type {
            PortType::Usb(usb_info) => {
                println!("Name: {}", port.name);
                println!("Type: USB");
                println!("VID: {}", usb_info.vid);
                println!("PID: {}", usb_info.pid);
                if let Some(serial_number) = usb_info.serial_number {
                    println!("Serial Number: {}", serial_number);
                }
                if let Some(manufacturer) = usb_info.manufacturer {
                    println!("Manufacturer: {}", manufacturer);
                }
                if let Some(product) = usb_info.product {
                    println!("Product: {}", product);
                }
            }
            PortType::Bluetooth => {
                println!("Name: {}", port.name);
                println!("Type: Bluetooth");
            }
            PortType::Pci => {
                println!("Name: {}", port.name);
                println!("Type: PCI");
            }
            PortType::Unknown => {
                println!("Name: {}", port.name);
                println!("Type: Unknown");
            }
        }
    }
}

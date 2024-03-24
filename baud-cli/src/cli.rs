use clap::{Command, Arg, ArgAction};

pub fn build_cli() -> Command {
    Command::new("Baud")
        .version("0.1.0")
        .author("Milind Sharma")
        .about("Serial Communication So Smooth, Even your Data Will Be Impressed!")
        .subcommand(Command::new("list")
            .about("Lists all available serial ports"))
            .arg(Arg::new("all")
                .short('a')
                .long("all")
                .help("List all available ports, including those that refer to the same physical port"))
        .subcommand(Command::new("open")
            .about("Connects to a serial port")
            .arg(Arg::new("port")
                .short('p')
                .long("port")
                .action(ArgAction::Set)
                .value_name("PORT")
                .help("The name of the serial port to connect to")
                .required(false))
            .arg(Arg::new("baud")
                .short('b')
                .long("baud")
                .action(ArgAction::Set)
                .value_name("BAUDRATE")
                .help("The baud rate to use when connecting")
                .required(false)
                .default_value("9600")),
        ) 
}

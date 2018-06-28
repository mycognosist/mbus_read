// mbus_read : commandline tool for executing a Modbus RTU
// read_holding_registers command

// import clap, a library for parsing commandline arguments
extern crate clap;

use clap::{Arg, App};

// import tokio-modbus libraries
extern crate futures;
extern crate tokio_core;
extern crate tokio_modbus;
extern crate tokio_serial;

use tokio_core::reactor::Core;
use futures::future::Future;
use tokio_serial::{BaudRate, Serial, SerialPortSettings};
use tokio_modbus::*;

fn main() {
    let matches = App::new("mbus_read")
        .version("0.1.0")
        .author("Andrew Reid <gnomad@cryptolab.net>")
        .about("Execute Modbus RTU read_holding_registers command")
        .arg(Arg::with_name("MODBUS_ADDRESS")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("modbus address to read from"))
        .arg(Arg::with_name("STARTING_REGISTER")
                 .required(true)
                 .takes_value(true)
                 .index(2)
                 .help("register at which to begin reading"))
        .arg(Arg::with_name("NUM_OF_REGISTERS")
                 .required(true)
                 .takes_value(true)
                 .index(3)
                 .help("number of registers to read"))
        .get_matches();
    // Assign variables based on input parameters
    let modbus_address = matches.value_of("MODBUS_ADDRESS").unwrap();
    let starting_register = matches.value_of("STARTING_REGISTER").unwrap();
    let num_of_registers = matches.value_of("NUM_OF_REGISTERS").unwrap();
    // Print values to consider for error-checking (dev feature)
    println!("Modbus address: {}", modbus_address);
    println!("Starting register: {}", starting_register);
    println!("Number of registers: {}", num_of_registers);
    println!("------------------------------");
    // Parse strings to unsigned integers (required for tokio-modbus)
    let modbus_addr: u8 = modbus_address
        .trim()
        .parse()
        .expect("Failed to parse");
    
    let start_register: u16 = starting_register.parse().unwrap();
    let registers: u16 = num_of_registers.parse().unwrap();

// Function to perform Modbus RTU read_holding_registers command
// pub fn read_registers() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let tty_path = "/dev/serial0";
    //let server_addr = 0x01;

    let mut settings = SerialPortSettings::default();
    settings.baud_rate = BaudRate::Baud19200;
    let mut port = Serial::from_path(tty_path, &settings, &handle).unwrap();
    port.set_exclusive(false).unwrap();

    let task = Client::connect_rtu(port, modbus_addr, &handle).and_then(|client| {
        println!("Reading registers");
        client
            //.read_holding_registers(0x082B, 2)
            .read_holding_registers(start_register, registers)
            .and_then(move |res| {
                println!("Register values: {:?}", res);
                Ok(())
            })
    });

    core.run(task).unwrap();
}

extern crate encoding;

use std::{env, error};
use std::io::prelude::*;
use std::net::TcpStream;
use encoding::{Encoding, EncoderTrap};
use encoding::all::ASCII;

const HOST: &'static str = "127.0.0.1:5555";

fn send_one_command(command: &str) -> Result<String, Box<error::Error + Send + Sync>> {
    let mut command_bytes = try!(ASCII.encode(command, EncoderTrap::Strict).map_err(|x| x.into_owned()));
    command_bytes.push('\r' as u8);

    let mut stream = try!(TcpStream::connect(HOST));
    try!(stream.write_all(&command_bytes));

    let mut response = String::new();
    let mut limited = stream.take(1024);
    try!(limited.read_to_string(&mut response));

    Ok(response)
}

fn main() {
    let command = match env::args().nth(1) {
        Some(cmd) => cmd,
        None => {
            let my_name = env::args().nth(0).unwrap();
            panic!("Usage: {} [command]", my_name)
        }
    };

    match send_one_command(&command) {
        Ok(response) => println!("{}", response),
        Err(err) => println!("An error occurred: {}", err),
    }
}
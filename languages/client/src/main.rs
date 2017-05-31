extern crate encoding;

use std::{env, error};
use std::io::prelude::*;
use std::net::TcpStream;
use encoding::{Encoding, EncoderTrap};
use encoding::all::ASCII;
use std::thread;
use std::io;
use std::io::prelude::*;

const HOST: &'static str = "127.0.0.1:5555";

fn send_one_command(command: &str) -> Result<String, Box<error::Error + Send + Sync>> {
    let mut command_bytes = try!(ASCII.encode(command, EncoderTrap::Strict).map_err(|x| x.into_owned()));
    command_bytes.push('\r' as u8); // ending escape sequence

    //let mut stream = try!(TcpStream::connect(HOST));
    let mut stream = TcpStream::connect("127.0.0.1:5555")
                                .expect("Couldn't connect to the server...");

    //move stream to nonblocking before writing
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    //try!(stream.write_all(&command_bytes));
    stream.write_all(&command_bytes);

    let mut response = String::new();
    let mut limited = stream.take(1024); // blocking!
    limited.read_to_string(&mut response);

    Ok(response)
}

fn main() {

    // program seems to be blocking on receive (stream.take)
    // moved the program to take std::input and send to the server
    // instead of command line argument to investigate blocking

    println!("Type command:");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let command = line.unwrap();
        println!("message: {}", command);

        match send_one_command(&command){
            Ok(response) => println!("response: {}", response),
            Err(err) => println!("An error occurred: {}", err),
        }
    }

    // command line argument check
    /*let command = match env::args().nth(1) {
        Some(cmd) => cmd,
        None => {
            let my_name = env::args().nth(0).unwrap();
            panic!("Usage: {} [command]", my_name)
        }
    };*/
}

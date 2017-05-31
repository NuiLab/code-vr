extern crate encoding;

use std::{error}; //{env, error}
use std::io::prelude::*;
use std::net::TcpStream;
use encoding::{Encoding, EncoderTrap};
use encoding::all::ASCII;
use std::thread::sleep;
use std::io;
use std::time::Duration;
//use std::io::prelude::*;

//const HOST: &'static str = "127.0.0.1:5555";

fn send_one_command(command: &str, mut stream: &TcpStream) -> Result<String, Box<error::Error + Send + Sync>> {
    let mut command_bytes = try!(ASCII.encode(command, EncoderTrap::Strict).map_err(|x| x.into_owned()));
    command_bytes.push('\r' as u8); // ending escape sequence    

    stream.write_all(&command_bytes).unwrap();

    sleep(Duration::from_millis(2)); //disgusting solution; program should wait until stream it ready.
    let mut response = String::new();
    let mut limited = stream.take(1024);
    limited.read_to_string(&mut response); //unwrap() causes it to panic

    if response.is_empty()
    {
        println!("response is empty")
    }

    Ok(response)
}

fn main() {

    //setup connection:
    //let mut stream = TcpStream::connect("127.0.0.1:5555")
    let stream = TcpStream::connect("127.0.0.1:5555") // try!(TcpStream::connect(HOST));
                               .expect("Couldn't connect to the server...");

    //make stream nonblocking
    stream.set_nonblocking(true).expect("set_nonblocking call failed");

    loop {
        println!("Type command:");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let command = line.unwrap();
            println!("message: {}", command);

            match send_one_command(&command, &stream){
                Ok(response) => println!("response: {}", response),
                Err(err) => println!("An error occurred: {}", err),
            }
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

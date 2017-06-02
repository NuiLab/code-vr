extern crate encoding;

use std::{error}; //{env, error}
use std::io::prelude::*;
use std::net::TcpStream;
use encoding::{Encoding, EncoderTrap};
use encoding::all::ASCII;
use std::thread::sleep;
use std::io;
use std::time::Duration;
use std::str;
//use std::io::prelude::*;

//const HOST: &'static str = "127.0.0.1:5555";

fn check_file(command: &str, mut stream: &TcpStream) -> Result<String, Box<error::Error + Send + Sync>> {
    //prepare buffer
    let mut command_bytes = try!(ASCII.encode(command, EncoderTrap::Strict).map_err(|x| x.into_owned()));
    command_bytes.push('\r' as u8);            //ending escape sequence    

    let _ = stream.write_all(&command_bytes).unwrap(); //send

    stream.set_read_timeout(None).expect("set_read_timeout call failed");
    assert_eq!(stream.read_timeout().unwrap(), None);

    //let response = String::new();
    //let mut buf = [0; 8];
    let mut buf: Vec<u8> = Vec::new();
    loop {
        let mut result = stream.read(&mut buf);
        match result {
            Ok(n) => {
                println!("Received {} bytes", n);
                buf.push(n);
                if n < 8 { break };
            },
            _ => {},
        }
    }

    //convert vec to string and print it out
    let response = String::from_utf8(buf).unwrap();
    println!("response: {}", response);

    /*
    //sleep(Duration::from_millis(10));          //disgusting solution;
                                               //program should wait until stream is ready

    let mut response = String::new();
    let mut buffer = [0; 1024];                //1024 bytes buffer

    //set a timeout
    let mut limited = stream.read(&mut [0; 1024]);       //read from stream (receive) (this blocks)
    //limited.read_to_string(&mut response);     //unwrap() causes this to panic
    */

    Ok(response)
}

fn main() {
    //setup connection:
    //let mut stream = TcpStream::connect("127.0.0.1:5555")
    let stream = TcpStream::connect("127.0.0.1:5555") // try!(TcpStream::connect(HOST));
                            .expect("Couldn't connect to the server...");

    //make stream nonblocking
    //stream.set_nonblocking(true).expect("set_nonblocking call failed");

    loop {
        println!("Type command:");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let command = line.unwrap();
            println!("message: {}", command);

            match check_file(&command, &stream){
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

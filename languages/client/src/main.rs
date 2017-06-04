#![feature(peek)] 
extern crate encoding;
extern crate byteorder;

use byteorder::{ByteOrder, BigEndian, LittleEndian};
use std::{error}; //{env, error}
use std::io::prelude::*;
use std::net::TcpStream;
use encoding::{Encoding, EncoderTrap};
use encoding::all::ASCII;
use std::io;
use std::str;
use std::mem;
//use std::thread::sleep;
//use std::io::prelude::*;
//use std::time::Duration;

fn check_file(command: &str, mut stream: &mut TcpStream) -> Result<String, Box<error::Error + Send + Sync>> {

    /*
    ****************
    *  REQUEST AST *
    ****************
    */


    //get string size (in bytes)
    let mut string_size = command.len();
    string_size = string_size + 1;
    println!("sending {} bytes", string_size);

    let mut string_size_str = string_size.to_string();

    //prepare buffer to send size
    let mut string_size_bytes =  try!(ASCII.encode(&string_size_str, EncoderTrap::Strict).map_err(|x| x.into_owned()));

    //prepare buffer to send message itself
    let mut command_bytes = try!(ASCII.encode(command, EncoderTrap::Strict).map_err(|x| x.into_owned()));
    command_bytes.push('\r' as u8); //ending escape sequence

    let mut response = String::new();

    //send message size:
    stream.write_all(&string_size_bytes).unwrap();

    //send file path
    stream.write_all(&command_bytes).unwrap();

    //receive message length:
    let mut buf = [0u8; 8]; //make it bigger
    stream.read(&mut buf).unwrap();

    //interpret the buffer contents into a string slice
    //let mut cl = buf.clone();
    let mut msg_len_slice: &str = str::from_utf8(&mut buf).unwrap(); //string slice

    //convert string slice to string type
    let mut msg_len_str = msg_len_slice.to_string();

    //clean string

    /*
    server might send message size smaller than buffer,
    which is usually the case when the server is sending
    the message size:
    buffer:     _ _ _ _ _ _ _ (bytes)
    message:    1 2 _ _ _ _ _ (bytes)
    (empty characters trail the meaningful characters)
    if this is the case, we shrink the string using .truncate()
    */

    let mut numeric_chars = 0;
    for c in msg_len_str.chars() {
        if c.is_numeric() == true { numeric_chars = numeric_chars+1;}
    }

    //shrink:
    msg_len_str.truncate(numeric_chars);

    //let msg_len = LittleEndian::read_u64(&mut cl);
    println!("receiving {} bytes", msg_len_str);

    //receive actual message:
    let mut remainingData = msg_len_str.parse::<i32>().unwrap();
    let mut r = [0u8; 8]; //buffer
    //println!("integer: {}", remainingData);

    let mut accumulator: String = String::new();

    while remainingData != 0
    {
        //println!("{} bytes remaining", remainingData);
        //if remainingData >= 8 // slab >= 8 byte buffer
        //{
            let mut slab = stream.read(&mut r);
            match slab {
                Ok(n) => {                    
                    let r_str = str::from_utf8(&mut r).unwrap();
                    accumulator.push_str(r_str);
                    println!("wrote {} bytes", n);
                    remainingData = remainingData - n as i32;
                }
                _ => {},
            }
        //}

    }

    //println!("{}", accumulator);


    //let s_ref = <TcpStream as Read>::by_ref(&mut stream);
    //match s_ref.take(msg_len).read(&mut r) {

    /*
    let s_ref = <TcpStream as Read>::by_ref(&mut stream);
    match s_ref.take(msg_len).read(&mut r) {
        Ok(0) => {
            println!("0 bytes read");
        },
        Ok(n) => {
            println!("{} bytes read", n);
            let response = std::str::from_utf8(&r[..]).unwrap();
            println!("{} bytes read", response);
        },
        Err(e) => {
            panic!("{} panic!", e);
        }
    }*/
    
    //raw_bytes.read_to_string(&mut response);          //convert string

    //response = str::from_utf8(raw_bytes).unwrap();

    //let conv = String::from_utf8(&raw_bytes.to_vec()).unwrap();
    //let mut limited = stream.take(8);
    //let mut limited = stream.read_to_string(&mut response);
    //let _ = limited.read_to_string(&mut response);

    response = accumulator;
    Ok(response)
}

fn main() {
    //setup connection:
    //let mut stream = TcpStream::connect("127.0.0.1:5555")
    let mut stream = TcpStream::connect("127.0.0.1:5555") // try!(TcpStream::connect(HOST));
                                .expect("Couldn't connect to the server...");

    //make stream nonblocking
    //stream.set_nonblocking(true).expect("set_nonblocking call failed");

    loop {
        println!("Type command:");
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let command = line.unwrap();
            println!("message: {}", command);

            match check_file(&command, &mut stream){
                Ok(response) => println!("response: {}", response),
                Err(err) => println!("An error occurred: {}", err),
            }
        }
    }
}

/* mob client
fn main()
{
    let mut stream = TcpStream::connect("127.0.0.1:5555").unwrap();

    let mut x = 0;
    while x < 10
    {
        let msg = "hello"; //11 or 12
        let mut buf = [0u8; 8];
        //println!("Sending over message length of {}", msg.len());
        //write the message length to buffer
        //BigEndian::write_u64(&mut buf, msg.len() as u64);
        //stream.write_all(buf.as_ref()).unwrap();

        //send message
        stream.write_all(msg.as_ref()).unwrap();

        //read message length
        let mut buf = [0u8; 8];
        stream.read(&mut buf).unwrap();
        let msg_len = BigEndian::read_u64(&mut buf);
        println!("Reading message length of {}", msg_len);

        //read message itself
        let mut r = [0u8; 256];
        let s_ref = <TcpStream as Read>::by_ref(&mut stream);

        match s_ref.take(msg_len).read(&mut r) {
            Ok(0) => {
                println!("0 bytes read");
            },
            Ok(n) => {
                println!("{} bytes read", n);
                let s = std::str::from_utf8(&r[..]).unwrap();
                println!("{} bytes read", s);
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
        x = x+1;
    }
}*/
extern crate encoding;

use std::{error}; //{env, error}
use std::net::TcpStream;
use encoding::{Encoding, EncoderTrap};
use encoding::all::ASCII;
use std::io;
use std::str;
use std::io::prelude::*;
use std::io::BufWriter;
use std::fs::File;

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

    let string_size_str = string_size.to_string();

    //encode buffer to send size
    let mut string_size_bytes =  try!(ASCII.encode(&string_size_str, EncoderTrap::Strict).map_err(|x| x.into_owned()));
    string_size_bytes.push('\r' as u8);

    //prepare buffer to send message itself
    let mut command_bytes = try!(ASCII.encode(command, EncoderTrap::Strict).map_err(|x| x.into_owned()));
    command_bytes.push('\r' as u8); //ending escape sequence

    //send message size:
    stream.write_all(&string_size_bytes).unwrap();

    //receive message size ACK:
    let mut ack_buf = [0u8; 8];
    stream.read(&mut ack_buf).unwrap();
    let ack_slice: &str = str::from_utf8(&mut ack_buf).unwrap(); //string slice
    let ack_str = ack_slice.to_string(); //convert slice to string
    println!("{}: server ackd", ack_str);


    //send file path
    stream.write_all(&command_bytes).unwrap();

    //receive message length:
    let mut buf = [0u8; 8]; //make it bigger if necessary
    stream.read(&mut buf).unwrap();

    //interpret the buffer contents into a string slice
    //let mut cl = buf.clone();
    let msg_len_slice: &str = str::from_utf8(&mut buf).unwrap(); //string slice
    let mut msg_len_str = msg_len_slice.to_string(); //convert slice to string

    /*
    CLEAN STRING:
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

    println!("receiving {} bytes", msg_len_str);

    let mut remaining_data = msg_len_str.parse::<i32>().unwrap();
    let mut accumulator: String = String::new();
    let mut r = [0u8; 8]; //8 byte buffer

    if remaining_data > 260 //big message; write to file
    {

        //format file name
        // ../files/file1.py
        let start = command.rfind('/').unwrap() as usize;
        let end = command.rfind('.').unwrap() as usize;
        let mut file_name = String::from(&command[start+1..end]);
        file_name.push_str(".json");
        println!("{}", &command);
        println!("{}", file_name);

        //create a file
        let mut file_buffer = BufWriter::new(File::create(file_name)?);

        while remaining_data != 0
        {
            if remaining_data >= 8 //slab >= 8 byte buffer
            {
                let slab = stream.read(&mut r);
                match slab {
                    Ok(n) => {                    
                        file_buffer.write(&mut r)?;
                        file_buffer.flush()?;
                        println!("wrote {} bytes to file", n);
                        remaining_data = remaining_data - n as i32;
                    }
                    _ => {},
                }
            }
            else //slab < 8 byte buffer
            {
                let array_limit = (remaining_data as i32) - 1;
                let slab = stream.read(&mut r);
                match slab {
                    Ok(n) => {
                        let mut r_slice = &r[0..(array_limit as usize + 1)]; // fixes underreading
                                                                             // caused by not using
                                                                             // subprocess call  on
                                                                             // the server server
                        file_buffer.write(&mut r_slice)?;
                        file_buffer.flush()?;
                        println!("wrote {} bytes to file", n);
                        remaining_data = remaining_data - n as i32;
                    }
                    _ => {},
                }
            }
        }
        accumulator.push_str("response written to file");
    }
    else{ //small message; receive as string
        while remaining_data != 0
        {
            if remaining_data >= 8 //slab >= 8 byte buffer
            {
                let slab = stream.read(&mut r);
                match slab {
                    Ok(n) => {                    
                        let r_slice = str::from_utf8(&mut r).unwrap(); //string slice
                        accumulator.push_str(r_slice);
                        println!("wrote {} bytes", n);
                        remaining_data = remaining_data - n as i32;
                    }
                    _ => {},
                }
            }
            /*
            option 1) receive and read a smaller buffer
            option 2) receive and read same buffer; truncate it to the smaller slab size

            since we cannot instantiate an array with a non-constant:
                e.g.: let mut r = [0u8; remainingData];
            it is better to just put the byte in the 8 byte buffer, and shrink it with
            .truncate() method before pushing to the String
            */
            else //slab < 8 byte buffer
            {
                let slab = stream.read(&mut r);
                match slab {
                    Ok(n) => {
                        let s_slice = str::from_utf8(&mut r).unwrap(); //string slice
                        let mut s_str = s_slice.to_string(); //convert slice to string
                        s_str.truncate(n);
                        accumulator.push_str(&s_str);
                        println!("wrote {} bytes", n);
                        remaining_data = remaining_data - n as i32;
                    }
                    _ => {},
                }
            }
        }
    }

    let response = accumulator;
    Ok(response)
}

fn main() {
    //setup connection:
    //let mut stream = TcpStream::connect("127.0.0.1:5555")
    let mut stream = TcpStream::connect("127.0.0.1:5555") // try!(TcpStream::connect(HOST));
                                .expect("Couldn't connect to the server...");
                                
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
#![feature(peek)]
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
let stream = TcpStream::connect("127.0.0.1:22181")
                       .expect("couldn't bind to address");
let mut buf = [0; 10];
let len = stream.peek(&mut buf).expect("peek failed");
}

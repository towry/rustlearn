
// server mod

use std::io::prelude::*;
use std::net::TcpStream;


pub fn run() {
    let stream = TcpStream::connect("127.0.0.1:8080");
    
    if stream.is_err() {
        println!("error: {:?}", stream);
        return;
    }

    let mut unwraped = stream.unwrap();
    
    let _ = unwraped.write(&[1]);
    let _ = unwraped.read(&mut [0; 128]);
}

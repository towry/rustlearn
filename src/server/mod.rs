
// server mod

use std::io::prelude::*;
use std::net::TcpStream;

#[allow(dead_code)]
static MESSAGE: &'static str = "
GET / HTTP/1.1
Host: 127.0.0.1:8080
Accept: text/*
Content-Type: text/text
Content-Length: 7

a=3&b=4
";

pub fn test_request() {
    let stream = TcpStream::connect("127.0.0.1:8080");
    
    if stream.is_err() {
        println!("error: {:?}", stream);
        return;
    } else {
        println!("connected");
    }

    let mut unwraped = stream.unwrap();

    let mut buffer = [0; 128];
    
    let _ = unwraped.write(MESSAGE.as_bytes());
    let what = unwraped.read(&mut buffer);
    if what.is_ok() {
        println!("{:?}", String::from_utf8_lossy(&buffer));
    }
}


pub fn run() {
    println!("there is nothing to see.");
}

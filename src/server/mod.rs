
// server mod
/**
 * demo for how browser works.
 * you need start a server with port 8080.
 */

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
    // connect to a local server.
    let stream = TcpStream::connect("127.0.0.1:8080");
    
    if stream.is_err() {
        println!("error: {:?}", stream);
        return;
    } else {
        println!("connected");
    }

    let mut unwraped = stream.unwrap();

    let mut buffer = vec![0; 10]; 
    
    // send something to the local server.
    let _ = unwraped.write(MESSAGE.as_bytes());
    // receive response from local server.
    let what = unwraped.read_to_end(&mut buffer);
    if what.is_ok() {
        // print out the response content.
        println!("{:?}", String::from_utf8_lossy(&buffer));
    }
}


pub fn run() {
    test_request();
}

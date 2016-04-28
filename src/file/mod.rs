
use std::io;
use std::io::prelude::*;
use std::fs::File;

pub fn test_read() -> Result<(), io::Error> {
    let mut f = try!(File::open("./data/foo.txt"));
    let mut buffer = vec![0; 10];

    try!(f.read_to_end(&mut buffer));
    println!("{}", String::from_utf8_lossy(&buffer));
    Ok(())
}

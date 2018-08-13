use std::io::prelude::*;
use std::net::TcpStream;

use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() {

    let mut client = TcpStream::connect("127.0.0.1:19999").unwrap();

    let _ = client.write(&[1]);
    let _ = client.read(&mut [0; 128]);

    let path = Path::new("test.txt");
    let display = path.display();

    let mut file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_)    => print!("{} contains\n{}", display, s),
    }

    println!("Hello, world!");
}

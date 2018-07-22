use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let fpath = Path::new("./test.txt");
    let fdisp = fpath.display();

    let mut file = match File::open(&fpath){
        Err(why) => panic!("could not open {}: {}", fdisp, why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(_) => panic!("cannot read {}", fdisp),
        Ok(_) => print!("{} contains {}", fdisp, s),
    };

    println!("Hello, world!");
}

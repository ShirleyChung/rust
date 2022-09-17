//use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() {
    let filename: &str;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        filename = &args[1];
    } else {
        filename = "./test.txt";
    }
    let fpath = Path::new(filename);
    let fdisp = fpath.display();

    let mut file = match File::open(&fpath){
        Err(why) => panic!("could not open {}: {}", fdisp, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s){
        Err(_) => panic!("cannot read {}", fdisp),
        Ok(_) => print!("{} contains {}", fdisp, s),
    };

    println!("Hello, world!");
}

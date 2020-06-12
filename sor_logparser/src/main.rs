use structopt::StructOpt;
use std::io::*;
use std::io::{BufRead, BufReader};
//use std::io::prelude::*;
use std::fs::File;

#[derive(StructOpt)]
struct Options {
	filepath: String // Log檔路徑
}

fn main() -> Result<()> {
	let options    = Options::from_args();

	let f = File::open(options.filepath)?;
	let reader= BufReader::new(f);
	
	for line in reader.lines() {
		match line {
			Ok(ok_line)=> println!("{:?}", ok_line),
			_ => continue,
		}
	}
	Ok(())
}

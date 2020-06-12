use structopt::StructOpt;
use std::io::*;
//use std::io::prelude::*;
use std::fs::File;

#[derive(StructOpt)]
struct Options {
	filepath: String // Log檔路徑
}

fn main() -> Result<()> {
	let options    = Options::from_args();
	let mut buffer = Vec::new();

	let mut f = File::open(options.filepath)?;
	f.read_to_end(&mut buffer)?;
	
	println!("{:?}", buffer);
	
	Ok(())
}

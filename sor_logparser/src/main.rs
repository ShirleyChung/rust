use structopt::StructOpt;
use std::io::*;
use std::io::{BufRead, BufReader};
//use std::io::prelude::*;
use std::fs::File;

#[derive(StructOpt)]
struct Options {
	filepath: String // Log檔路徑
}

struct Parser {
	providers: Vec<String>,
}

impl Parser{
	fn parse_line(&mut self, line: &str) {
		self.providers.push(line.to_string());
	}
}

fn main() -> Result<()> {
	let options    = Options::from_args();

	let f           = File::open(options.filepath)?;
	let reader     = BufReader::new(f);
	let mut parser = Parser{ providers: Vec::<String>::new() };
	
	for line in reader.lines() {
		match line {
			Ok(ok_line)=> parser.parse_line(&ok_line),
			_ => continue,
		}
	}
	Ok(())
}

use structopt::StructOpt;
use std::io::*;
use std::io::{BufRead, BufReader};
//use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

#[derive(StructOpt)]
struct Options {
	filepath: String // Log檔路徑
}

/// 儲存解析後的陣列
struct Parser {
	providers: Vec<String>,
	reqRecs: HashMap<String, Vec<String>>,
	ordRecs: HashMap<String, Vec<String>>,
}

/// 陣列的操作函式
impl Parser{
	fn new()->Parser {
		Parser{ 
			providers: Vec::<String>::new(),
			reqRecs: HashMap::<String, Vec<String>>::new(),
			ordRecs: HashMap::<String, Vec<String>>::new() 
			}
	}
	fn parse_line(&mut self, line: &str) {
		self.providers.push(line.to_string());
	}
}

/// 第一參數指定檔案
/// 將其讀入陣列似便解析
fn main() -> Result<()> {
	let options    = Options::from_args();

	let f           = File::open(options.filepath)?;
	let reader     = BufReader::new(f);
	let mut parser = Parser::new();
	
	for line in reader.lines() {
		match line {
			Ok(ok_line)=> parser.parse_line(&ok_line),
			_ => continue,
		}
	}
	
	for line in parser.providers {
		let toks = line.split('\x01');
		for (i, tok) in toks.enumerate() {
			println!("{}, {:?}", i, tok);
		}
		
	}
	
	Ok(())
}


use structopt::StructOpt;
use std::io::*;
use std::io::{BufRead, BufReader};
//use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::fmt;

#[derive(StructOpt)]
struct Options {
	filepath: String // Log檔路徑
}

/// 儲存解析後的陣列
struct Parser {
	providers: HashMap<String, Vec<String>>,
	req_recs:  HashMap<String, Vec<String>>,
	ord_recs:  HashMap<String, Vec<String>>,
}

/// 陣列的操作函式
impl Parser{
	fn new()->Parser {
		Parser{ 
			providers: HashMap::<String, Vec<String>>::new(),
			req_recs:   HashMap::<String, Vec<String>>::new(),
			ord_recs:   HashMap::<String, Vec<String>>::new() 
			}
	}
	fn parse_line(&mut self, line: &str) {
		let toks : Vec<String> = line.to_string().split('\x01').map(|s| s.to_string()).collect();
		if toks.len() > 2 {
			if toks[1] == "-" {
				self.providers.insert((&toks[0]).to_string(), toks);
			}
			else if toks[0] == "Req" {
				self.req_recs.insert((&toks[1]).to_string(), toks);
			}
			else if toks[0] == "Ord" {
				self.ord_recs.insert((&toks[1]).to_string(), toks);
			}
		}
	}
}

impl fmt::Display for Parser {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "prividers: {}, reqs: {}, ords: {}", self.providers.len(), self.req_recs.len(), self.ord_recs.len())
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
	
	println!("parser: {}", parser);
	
	Ok(())
}


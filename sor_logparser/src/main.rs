use structopt::StructOpt;
use std::io::*;
use std::io::{BufRead, BufReader};
//use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::fmt;

#[derive(StructOpt)]
struct Options {
	filepath: String, // Log檔路徑
	#[structopt(short="o", long="ordno", default_value = "")]
	ordno   : String,
}

struct Field {
	field_name: String,
	field_index:u32,
}

/// 儲存解析後的陣列
struct Parser {
	providers: HashMap<String, Vec<Field>>,
	req_recs:  HashMap<String, Vec<String>>,
	ord_recs:  HashMap<String, Vec<String>>,
}

/// 陣列的操作函式
impl Parser{
	fn new()->Parser {
		Parser{ 
			providers:  HashMap::<String, Vec<Field>>::new(),
			req_recs:   HashMap::<String, Vec<String>>::new(),
			ord_recs:   HashMap::<String, Vec<String>>::new() 
			}
	}
	/// 解析每一行的內容, 並儲存到HashMap
	fn parse_line(&mut self, line: &str) {
		let toks : Vec<String> = line.to_string().split('\x01').map(|s| s.to_string()).collect();
		if toks.len() > 3 {
			let key = &toks[1];
			if key == "-" {
				// Provider的名稱, 例如 TwsNew
				let provider_name = (&toks[2]).to_string();
				if !self.providers.contains_key(&provider_name) {
					// 建立Provider的陣列
					let mut fields = Vec::<Field>::new();
					// 插入每個Provider的Field
					for (idx, name) in toks.iter().enumerate() {
						fields.push(Field{field_name: name.to_string(), field_index: idx as u32});
					}
					self.providers.insert(provider_name, fields);
				}
			}
			else if toks[0] == "Req" {
				self.req_recs.insert((&toks[1]).to_string(), toks);
			}
			else if toks[0] == "Ord" {
				self.ord_recs.insert((&toks[1]).to_string(), toks);
			}
		}
	}
	/// 以委託書號為key, 找出req-ord
	fn find_ordno(&self, _ordno: &str) -> String {
		String::from("not found")
	}
}

/// 使Parse類別能以println列印出來
impl fmt::Display for Parser {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "prividers: {}, reqs: {}, ords: {}", self.providers.len(), self.req_recs.len(), self.ord_recs.len())
	}
}

/// 第一參數指定檔案
/// 將其讀入陣列以便解析
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
	
	if !options.ordno.is_empty() {
		println!("ord {} => {}", options.ordno, parser.find_ordno(&options.ordno));
	}
	
	Ok(())
}


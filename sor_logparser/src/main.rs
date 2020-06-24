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
	#[structopt(short="t", long="table", default_value = "FrfOrd")]
	table   : String,
}

struct Field {
	field_name: String,
	field_index:u32,
}

/// 儲存解析後的陣列
struct Parser {
	rec_tables: HashMap<String, HashMap<String, Field>>,
	req_recs:   HashMap<String, Vec<String>>,
	ord_recs:   HashMap<String, Vec<String>>,
}

/// 陣列的操作函式
impl Parser{
	fn new()->Parser {
		Parser{ 
			rec_tables:  HashMap::<String, HashMap<String, Field>>::new(),
			req_recs:    HashMap::<String, Vec<String>>::new(),
			ord_recs:    HashMap::<String, Vec<String>>::new() 
			}
	}
	/// 解析每一行的內容, 並儲存到HashMap
	fn parse_line(&mut self, line: &str) {
		let toks : Vec<String> = line.to_string().split('\x01').map(|s| s.to_string()).collect();
		if toks.len() > 3 {
			let key = &toks[1];
			if key == "-" {
				// Provider table的名稱, 例如 TwsNew
				let table_name = (&toks[2]).to_string();
				if !self.rec_tables.contains_key(&table_name) {
					// 建立Provider的陣列
					let mut fields = HashMap::<String, Field>::new();
					// 插入每個Provider的Field
					for (idx, name) in toks.iter().enumerate() {
						fields.insert(name.to_string(), Field{field_name: name.to_string(), field_index: idx as u32});
					}
					self.rec_tables.insert(table_name, fields);
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
	
	/// 以index, 找出rec中相等於target的rec
	fn find_ord(&self, key_index: usize, target: &str) {
		for (key, ord_vec) in &self.ord_recs {
			if ord_vec[key_index] == target.to_string() {
				println!("found key = {}, {:?}", key, ord_vec)
			}
		}
	}
	
	/// 以委託書號為key, 找出req-ord
	fn find_ordno(&self, table: &str, ordno: &str) -> String {
		let result = String::from("end");
		match self.rec_tables.get(table) {
			Some(provider_vec) => self.find_ord(provider_vec["OrdNo"].field_index as usize, ordno),
			_ => println!("{} NotFound", table),
		}
		
		result
	}
}

/// 使Parse類別能以println列印出來
impl fmt::Display for Parser {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "prividers: {}, reqs: {}, ords: {}", self.rec_tables.len(), self.req_recs.len(), self.ord_recs.len())
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
		println!("ord {} => {}", options.ordno, parser.find_ordno(&options.table, &options.ordno));
	}
	
	Ok(())
}


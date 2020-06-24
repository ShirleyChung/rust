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
	#[structopt(short="f", long="field", default_value = "")]
	/// 指定TableName:FieldName:SearchValue; 例如 -f TwsNew:SorRID:100001
	field   : String,
}

struct Field {
//	field_name  :String,
	field_index :usize,
}

struct Req {
	req_recs: Vec<String>,
	line    : String,
	log     : String,
}

struct Ord {
	ord_recs: Vec<String>,
	line    : String,
	log     : String,
}

/// 儲存解析後的陣列
struct Parser {
	rec_tables: HashMap<String, HashMap<String, Field>>,
	req_recs:   HashMap<String, Req>,
	ord_recs:   HashMap<String, Ord>,
}

/// 陣列的操作函式
impl Parser{
	fn new()->Parser {
		Parser{ 
			rec_tables:  HashMap::<String, HashMap<String, Field>>::new(),
			req_recs:    HashMap::<String, Req>::new(),
			ord_recs:    HashMap::<String, Ord>::new() 
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
						fields.insert(name.to_string(), Field{/*field_name: name.to_string(),*/ field_index: idx});
					}
					self.rec_tables.insert(table_name, fields);
				}
			}
			else if toks[0] == "Req" {
				self.req_recs.insert((&toks[1]).to_string(), Req{req_recs: toks, line: line.to_string(), log: String::new()});
			}
			else if toks[0] == "Ord" {
				self.ord_recs.insert((&toks[1]).to_string(), Ord{ord_recs: toks, line: line.to_string(), log: String::new()});
			}
		}
	}
	
	/// 以index, 找出rec中相等於target的rec
	fn find_rec(&self, key_index: usize, target: &str) {
		for (key, ords) in &self.ord_recs {
			if ords.ord_recs[key_index] == target.to_string() {
				println!("found key = {}, {}, {}", key, ords.line, ords.log)
			}
		}
	}
	
	/// 以委託書號為key, 找出req-ord
	fn find_by_field(&self, table: &str, field_name: &str, search_target: &str) -> String {
		let result = String::from("end");

		match self.rec_tables.get(table) {
			Some(provider_vec) => {
				match provider_vec.get(field_name){
					Some(field) => self.find_rec(field.field_index, search_target),
					_=> println!("{} doesn't exist", field_name),
				}
				},
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
	
	let field_vec :Vec<String> = options.field.split(':').map(|s| s.to_string()).collect();
	if field_vec.len() < 3 {
		println!("please specify -f TableName:FieldName:Value");
		return Ok(())
	}
	else {
		println!("ord {} => {}", options.field, parser.find_by_field(&field_vec[0], &field_vec[1], &field_vec[2]));
	}
	
	Ok(())
}


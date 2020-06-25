use structopt::StructOpt;
use std::io::*;
use std::io::{BufRead, BufReader};
//use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::fmt;

/// 第一步，程式的參數接收
#[derive(StructOpt)]
struct Options {
	/// 要解析的SorReqOrd.log
	filepath: String, // Log檔路徑
	/// 指定TableName:FieldName:SearchValue; 例如 -f TwsNew:SorRID:100001
	#[structopt(short="f", long="field", default_value = "")]
	field   : String,
}

/// 第二步，定義儲存的資料結構
struct Rec {
	recs_vec: Vec<String>,
	line    : String,
	log     : String,
}

struct ReqData {
	tables: HashMap<String, HashMap<String, usize>>,
	recs:   HashMap<String, Rec>,
}

impl ReqData {
	fn new() -> ReqData {
		ReqData {
			tables: HashMap::<String, HashMap<String, usize>>::new(),
			recs  : HashMap::<String, Rec>::new(),
		}
	}
	fn insert_data(&mut self, toks: &Vec<String>, line: &str ) {
		let key = &toks[1];		
		if key == "-" { // 沒有key值的，是表格
			// Provider table的名稱, 例如 TwsNew
			let table_name = (&toks[2]).to_string();
			if !self.tables.contains_key(&table_name) {
				// 建立Provider的陣列
				let mut fields = HashMap::<String, usize>::new();
				// 插入每個Provider的Field
				for (idx, name) in toks.iter().enumerate() {
					fields.insert(name.to_string(), idx);
				}
				self.tables.insert(table_name, fields);
			}
		}
		else {  // 依key將記錄儲存到hashmap中
			self.recs.insert((&toks[1]).to_string(), Rec{recs_vec: toks.to_vec(), line: line.to_string(), log: String::new()});
		}
	}
	/// 以index, 找出rec中相等於target的rec
	fn find_rec(&self, recs: &HashMap<String, Rec>, key_index: usize, target: &str) {
		for (key, rec) in recs {
			if rec.recs_vec[key_index] == target.to_string() {
				println!("found key = {}, {}, {}", key, rec.line, rec.log)
			}
		}
	}

	fn check_req_data(&self, table_name: &str, field_name: &str, search_target: &str) {
		match self.tables.get(table_name) {
			Some(table) => { // 表中，該欄位的index
				match table.get(field_name){
					Some(idx) => self.find_rec(&self.recs, *idx, search_target),
					_=> println!("{} doesn't exist", field_name),
				}
			},
			_ => println!("{} NotFound", table_name),
		}	
	}
}

/// 儲存解析後的陣列
struct Parser {
	req_recs :ReqData,
	ord_recs :ReqData, 
}

/// 陣列的操作函式
impl Parser {
	fn new()->Parser {
		Parser{ 
			req_recs: ReqData::new(),
			ord_recs: ReqData::new(), 
			}
	}

	/// 解析每一行的內容, 並儲存到HashMap
	fn parse_line(&mut self, line: &str) {
		let toks : Vec<String> = line.to_string().split('\x01').map(|s| s.to_string()).collect();
		if toks.len() > 3 {
			if toks[0] == "Req" {
				self.req_recs.insert_data(&toks, line);
			}
			else if toks[0] == "Ord" {
				self.ord_recs.insert_data(&toks, line);
			}
		}
	}
	
	/// 輸入 表名/欄位名/值 來尋找目標
	fn find_by_field(&mut self, table_name: &str, field_name: &str, search_target: &str) {
		// 先找看看 Req表
		self.req_recs.check_req_data(table_name, field_name, search_target);
		// 再找看看 Ord表
		self.ord_recs.check_req_data(table_name, field_name, search_target);
	}
}

/// 使Parse類別能以println列印出來
impl fmt::Display for Parser {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "req prividers: {}, ord providers:{}, reqs: {}, ords: {}", 
			self.req_recs.tables.len(), self.ord_recs.tables.len(), 
			self.req_recs.recs.len(),   self.ord_recs.recs.len())
	}
}

/// 第一參數指定檔案
/// 將其讀入陣列以便解析
fn main() -> Result<()> {
	let options    = Options::from_args();

	let f           = File::open(options.filepath)?;
	let reader     = BufReader::new(f);
	let mut parser = Parser::new();
	
	// 依每行解析
	for line in reader.lines() {
		match line {
			Ok(ok_line)=> parser.parse_line(&ok_line),
			_ => continue,
		}
	}
	
	// 解析完了, 顯示解析結果
	println!("parser: {}", parser);
	
	// 搜尋指定的目標
	let field_vec :Vec<String> = options.field.split(':').map(|s| s.to_string()).collect();
	if field_vec.len() < 3 {
		println!("please specify -f TableName:FieldName:Value");
		return Ok(())
	}
	else {
		parser.find_by_field(&field_vec[0], &field_vec[1], &field_vec[2]);
	}
	
	Ok(())
}


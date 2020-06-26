use structopt::StructOpt;
use std::io::*;
use std::io::{BufRead, BufReader};
//use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::LinkedList;
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
	reqs_vec: Vec<String>,
	line    : String,
	log     : String,
}

impl Rec {
	fn print(&self) {
		println!("{}", self.line);
	}
}



struct TableRec {
	index: HashMap<String, usize>,
	recs : Vec<String>,
}

impl TableRec {
	fn new() -> TableRec {
		TableRec {
			index: HashMap::<String, usize>::new(),
			recs : Vec::<String>::new(),
		}
	}
}

type ReqRecMap   = HashMap<String, Rec>;            // ReqKey-Rec
type OrdRecMap   = HashMap<String, LinkedList<Rec>>;// OrdKey-Rec

struct OrderRec {
	tables : HashMap<String, TableRec>, // table_name-table fields
	reqs   : ReqRecMap,
	ords   : OrdRecMap,
	req2ord: HashMap<String, String>,   // req對應到的ord
}

impl OrderRec {
	fn new() -> OrderRec {
		OrderRec {
			tables: HashMap::<String, TableRec>::new(), // 表格名-欄位-index
			reqs  : ReqRecMap::new(),                   // reqKey-一筆Req
			ords  : OrdRecMap::new(),                   // ordKey-一筆Ord
			req2ord: HashMap::<String, String>::new(),
		}
	}
	fn insert_rec(&mut self, toks: Vec<String>, line: &str ) {
		let key = &toks[1];
		let hdr = &toks[0];	
		if key == "-" { // 沒有key值的，是表格
			let table_name = &toks[2];	
			let mut tabrec = self.tables.entry(table_name.to_string()).or_insert(TableRec::new());
			for (idx, name) in toks.iter().enumerate() { // 插入每個Provider的Field
				tabrec.index.insert(name.to_string(), idx);
			}
			tabrec.recs = toks.to_vec();
		}
		else if "Req" == hdr {  // 依key將記錄儲存到hashmap中
			self.reqs.insert(key.to_string(), Rec{reqs_vec: toks.to_vec(), line: line.to_string(), log: String::new()});
		}
		else if "Ord" == hdr {	
			let rec = Rec{reqs_vec: toks.to_vec(), line: line.to_string(), log: String::new()};
			self.ords.entry(key.to_string()).or_insert(LinkedList::<Rec>::new()).push_back(rec);
			// 檢查Req-Ord對應是否有覆蓋的情況
			let reqkey = &toks[4];
			match self.req2ord.get(reqkey) {
				Some(ordkey) => {
					if ordkey != key {
						println!("There is MISS-MAPPING req-ord: req:{} ord:{}", reqkey, ordkey);
					}
				},
				_ => (),
			}
			self.req2ord.insert(reqkey.to_string(), key.to_string());
		}
		else {
			println!("unknow toks");
		}
	}
	fn print_ord(&self, key: &str) {
		match self.ords.get(key) {
			Some(list) => {
				let mut reqord_list = LinkedList::<&Rec>::new();
				let mut reqkey: String = String::from("");
				for ord in list {
					let ord_reqkey = &ord.reqs_vec[4];
					if &reqkey != ord_reqkey {
						match self.reqs.get(ord_reqkey) {
							Some(rec) => reqord_list.push_back(rec),
							_=> println!("req {} not found", ord_reqkey),
						}
						reqkey = ord_reqkey.to_string();
					}
					reqord_list.push_back(ord);
				}
				for rec in reqord_list {
					rec.print();
				}
			},
			_=> println!("order {} not exist", key),
		};
	}
	/// 以index, 找出ords中相等於target的rec
	fn find_req(&self, key_index: usize, target: &str) {
		println!("find req {}, index={} ords:{}", target, key_index, self.ords.len());
		for (key, rec) in &self.reqs  {
			if  rec.reqs_vec.len() < 3 {
				continue;
			}
			if rec.reqs_vec.len() > key_index {
				if rec.reqs_vec[key_index] == target.to_string() {
					println!("found key = {}, {}, {}", key, rec.line, rec.log)
				}
			} else {
				println!("line:{}, \n fields miss match.", rec.line);
			}
		}
	}
	/// 以index, 找出reqs中相等於target的rec
	fn find_ord(&self, key_index: usize, target: &str) {
		for (key, list) in &self.ords {
			match list.back() {
				Some(rec) => {
					if  rec.reqs_vec.len() < 3 {
						continue;
					}
					if rec.reqs_vec.len() > key_index {
						if rec.reqs_vec[key_index] == target.to_string() {
							self.print_ord(&rec.reqs_vec[1]);
						}
					} else {
						println!("line:{}, \n fields miss match.", rec.line);
					}
				},
				_=>println!("{} is empty", key),
			}
		}
	}

	fn check_req_data(&self, table_name: &str, field_name: &str, search_target: &str) {
		println!("checking {}, {}", field_name, search_target);
		match self.tables.get(table_name) {
			Some(tabrec) => { 
				match tabrec.index.get(field_name) {
					Some(idx) => {
						if tabrec.recs[0] == "Req" {
							self.find_req(*idx, search_target);
						}
						else if tabrec.recs[0] == "Ord" {
							self.find_ord(*idx, search_target);							
						}
						else {
							println!("cannot find {}, {}", field_name, search_target);
						}
					},
					_=> println!("field {} not found", field_name),
				}
			},
			_=> println!("{} doesn't exist", field_name),
		}	
	}
}

/// 儲存解析後的陣列
struct Parser {
	ord_rec : OrderRec,
}

/// 陣列的操作函式
impl Parser {
	fn new()->Parser {
		Parser{ 
			ord_rec: OrderRec::new(),
		}
	}

	/// 解析每一行的內容, 並儲存到HashMap
	fn parse_line(&mut self, line: &str) {
		let toks : Vec<String> = line.to_string().split('\x01').map(|s| s.to_string()).collect();
		if toks.len() > 3 {
			self.ord_rec.insert_rec(toks, line);
		}
	}
	
	/// 輸入 表名/欄位名/值 來尋找目標
	fn find_by_field(&mut self, table_name: &str, field_name: &str, search_target: &str) {
		// 先找看看 Req表
		self.ord_rec.check_req_data(table_name, field_name, search_target);

	}
}

/// 使Parse類別能以println列印出來
impl fmt::Display for Parser {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "tables: {} reqs: {} ords:{}", 
			self.ord_rec.tables.len(), self.ord_rec.reqs.len(), self.ord_rec.ords.len())
	}
}

/// 第一參數指定檔案
/// 將其讀入陣列以便解析
fn main() -> Result<()> {
	let options    = Options::from_args();

	let f          = File::open(options.filepath)?;
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


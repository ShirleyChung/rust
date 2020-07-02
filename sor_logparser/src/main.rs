use structopt::StructOpt;
use std::io::*;
use std::io::{BufRead, BufReader};
//use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fmt;
use encoding::{Encoding, DecoderTrap};
use encoding::all::BIG5_2003;
use chrono::prelude::*;

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
		if self.reqs_vec.len() > 3 {
			let ts_toks : Vec<String> = self.reqs_vec[3].split('.').map(|s| s.to_string()).collect();
			if ts_toks.len() > 1 {
				let u_secs = ts_toks[0].parse::<i64>().unwrap();
				let u_ms   = ts_toks[1].parse::<i64>().unwrap();
				let datetime: DateTime<Local> = Local.timestamp(u_secs, 0);
				let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
    			println!("{} {}", newdate, u_ms);
			}
		}
		println!("{}{}", self.line, self.log);
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
	fn insert_rec(&mut self, toks: Vec<String>, line: &str, log: &str) -> (&'static str, String) {
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
			self.reqs.insert(key.to_string(), Rec{reqs_vec: toks.to_vec(), line: line.to_string(), log: log.to_string()});
			return ("Req", key.to_string())
		}
		else if "Ord" == hdr {	
			let rec = Rec{reqs_vec: toks.to_vec(), line: line.to_string(), log: log.to_string()};
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
			return ("Ord", key.to_string())
		}
		else {
			//println!("unknow toks");
		}
		("", "".to_string())
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
					};
					reqord_list.push_back(ord);
				}
				println!("ordlist:{}", list.len());
				for rec in reqord_list {
					rec.print();
				}
			},
			_=> println!("order {} not exist", key),
		};
	}
	/// 以index, 找出ords中相等於target的rec
	fn find_req(&self, table_name: &str, key_index: usize, target: &str) {
		println!("find req {}, index={} ords:{}", target, key_index, self.ords.len());
		for (key, rec) in &self.reqs  {
			if  rec.reqs_vec.len() < 3 || rec.reqs_vec[2] != table_name {
				continue;
			}
			if rec.reqs_vec.len() > key_index {
				if rec.reqs_vec[key_index] == target.to_string() {
					self.print_ord(&self.req2ord[key])
				}
			} else {
				println!("line:{}, \n fields miss match.", rec.line);
			}
		}
	}
	/// 以index, 找出reqs中相等於target的rec
	fn find_ord(&self, table_name: &str, key_index: usize, target: &str) {
		for (key, list) in &self.ords {
			match list.back() {
				Some(rec) => {
					if  rec.reqs_vec.len() < 3 || rec.reqs_vec[2] != table_name {
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
							self.find_req(table_name, *idx, search_target);
						}
						else if tabrec.recs[0] == "Ord" {
							self.find_ord(table_name, *idx, search_target);							
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
	prevkey : (&'static str, String)
}

/// 陣列的操作函式
impl Parser {
	fn new()->Parser {
		Parser{ 
			ord_rec: OrderRec::new(),
			prevkey: ("", "".to_string()),
		}
	}

	/// 解析每一行的內容, 並儲存到HashMap
	fn parse_line(&mut self, line: &str, log: &str) {
		let toks : Vec<String> = line.to_string().split('\x01').map(|s| s.to_string()).collect();

		if toks.len() > 3 {
			self.prevkey = self.ord_rec.insert_rec(toks, line, log);
		} else {
			//println!("log line: {}", line);
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
/*
fn read_data_log(reader: &mut BufReader, parser: &mut Parser) -> Result<()> {
	let mut line_buf   = Vec::<u8>::new();
	// 讀第一行
	if reader.read_until(b'\n', &mut line_buf).is_ok() {
		let mut line = String::new();
		if BIG5_2003.decode_to(&mut line_buf, DecoderTrap::Ignore, &mut line).is_ok() {
			line_buf.clear();
			// 讀第二行
			if reader.read_until(b'\n', &mut line_buf).is_ok() {
				let mut log = String::new();
				if BIG5_2003.decode_to(&mut line_buf, DecoderTrap::Ignore, &mut log).is_ok() {
					// 第二行是LOG
					if log.as_bytes()[0] == ':' as u8 {
						parser.parse_line(&line, &log);
					} else { // 否則是另一筆ReqOrd
						parser.parse_line(&line, "");
						parser.parse_line(&log, "");					
					}
				} else { // 第二行Decode失敗, 仍要parse第一行
					parser.parse_line(&line, "");
				}			
				return Ok(()) // 繼續往下讀
			}
			// 讀不到第二行, 代表已到EOF
			parser.parse_line(&line, "");
		}		
	}
	Err(());
}
*/
/// 第一參數指定檔案
/// 將其讀入陣列以便解析
fn main() -> Result<()> {
	let options    = Options::from_args();

	let f           = File::open(options.filepath)?;
	let mut reader = BufReader::new(f);
	let mut parser = Parser::new();

/*
	// 依每行解析
	let mut line_buf   = Vec::<u8>::new();
	while let Ok(sz_line) = reader.read_until(b'\n', &mut line_buf) {
		if sz_line > 0 {
			let mut line = String::new();
			if BIG5_2003.decode_to(&mut line_buf, DecoderTrap::Ignore, &mut line).is_ok() {
				parser.parse_line(&line, "");
			}
			line_buf.clear();
		} else {
			break;
		}
	}
*/
	let mut line_buf   = Vec::<u8>::new();
	loop {
		// 讀第一行
		line_buf.clear();
		if reader.read_until(b'\n', &mut line_buf).is_ok() {
			let mut line = String::new();
			if BIG5_2003.decode_to(&mut line_buf, DecoderTrap::Ignore, &mut line).is_ok() && line.len() > 0 {
				// 讀第二行
				line_buf.clear();
				if reader.read_until(b'\n', &mut line_buf).is_ok() {
					let mut log = String::new();
					if BIG5_2003.decode_to(&mut line_buf, DecoderTrap::Ignore, &mut log).is_ok() && log.len() > 0 {
						// 第二行是LOG
						if log.as_bytes()[0] == ':' as u8 {
							parser.parse_line(&line, &log);
						} else { // 否則是另一筆ReqOrd
							parser.parse_line(&line, "");
							parser.parse_line(&log, "");					
						}
					} else { // 第二行Decode失敗, 仍要parse第一行
						parser.parse_line(&line, "-=LOG decode failed=-");
					}			
					continue; // 繼續往下讀
				}
				// 讀不到第二行, 代表已到EOF
				parser.parse_line(&line, "");
			}
		}
		break;
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


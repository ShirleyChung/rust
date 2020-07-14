use std::collections::HashMap;
use std::collections::LinkedList;
use std::fmt;
use chrono::prelude::*;

// 每一筆資料由 string array組成每一個欄位，原資料ReqOrd, 以及相關的log
pub struct Rec {
	reqs_vec: Vec<String>,
	line    : String,
	log     : String,
}

fn get_ordst(st: i32) -> String {
	match st {
		6 => "委託傳送中".to_string(),
		7 => "委託已傳送".to_string(),
		90=> "委託成功".to_string(),
		99 => "委託失敗".to_string(),
		101 => "交易所已接受".to_string(),
		110 => "部份成交".to_string(),
		111 => "全部成交".to_string(),
		120 => "交易所取消".to_string(),
		_ => "".to_string(),
	}
}

// Rec具有print的操作, 可將timestamp印出
impl Rec {
	pub fn is_req(&self) -> bool {
		&self.reqs_vec[0] == "Req"
	}
	pub fn get_timestamp(&self) -> String {
		let mut dt = String::new();
		if self.reqs_vec.len() > 3 {
			let ts_toks : Vec<String> = self.reqs_vec[3].split('.').map(|s| s.to_string()).collect();
			if ts_toks.len() > 1 {
				let u_secs = ts_toks[0].parse::<i64>().unwrap();
				let datetime: DateTime<Local> = Local.timestamp(u_secs, 0);
				dt = datetime.format("%Y-%m-%d %H:%M:%S:").to_string() + &ts_toks[1];
			}
		}
		dt
	}
	pub fn print(&self) {
		if self.reqs_vec.len() > 5 {
			if self.is_req() {
				let type_key = &self.reqs_vec[4][..];
				let ord_type: &str = match type_key
				{ "1" => "新單", "2" => "改量", "3" => "改價", "4" => "刪單", _=> "" };
				println!("\n{} ({})", self.get_timestamp(), ord_type);
			} 
			else {
				let ordst = self.reqs_vec[6].parse::<i32>().unwrap();
				println!("{} =>{}", self.get_timestamp(), get_ordst(ordst));
			}
		}
		println!("{}\n{}", self.line, self.log);
	}
}

pub struct TableRec {
	index: HashMap<String, usize>,
	recs : Vec<String>,
}

impl TableRec {
	pub fn new() -> TableRec {
		TableRec {
			index: HashMap::<String, usize>::new(),
			recs : Vec::<String>::new(),
		}
	}
	pub fn print(&self) {
		for rec in &self.recs {
			print!("{} ", rec);
		}
		println!("");
	}
}

type ReqRecMap   = HashMap<String, Rec>;            // ReqKey-Rec
type OrdRecMap   = HashMap<String, LinkedList<Rec>>;// OrdKey-Rec

pub struct OrderRec {
	tables : HashMap<String, TableRec>, // table_name-table fields
	reqs   : ReqRecMap,
	ords   : OrdRecMap,
	req2ord: HashMap<String, String>,   // req對應到的ord
}

pub struct OrdInfo {
	rid   : String,
	ordno : String,
	status: String,
}

impl OrdInfo {
	pub fn new() -> OrdInfo {
		OrdInfo {
			rid   : String::new(),
			ordno : String::new(),
			status: String::new(),
		}
	}
	pub fn to_string(&self) -> String {
		String::from("流水號:") + &self.rid + " 委託書號:" + &self.ordno + " 最後狀態:" + &self.status
	}
}

// 3. ReqOrd資料的輸入與輸出
impl OrderRec {
	pub fn new() -> OrderRec {
		OrderRec {
			tables: HashMap::<String, TableRec>::new(), // 表格名-欄位-index
			reqs  : ReqRecMap::new(),                   // reqKey-一筆Req
			ords  : OrdRecMap::new(),                   // ordKey-一筆Ord
			req2ord: HashMap::<String, String>::new(),
		}
	}
	pub fn insert_rec(&mut self, toks: Vec<String>, line: &str, log: &str) -> (&'static str, String) {
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
	/// 取得 指定Ord key 的 ReqOrd 的 LinkedList
	pub fn get_target_ordlist(&self, key: &str) -> LinkedList<&Rec> {
		let mut reqord_list = LinkedList::<&Rec>::new();
		match self.ords.get(key) {
			Some(list) => {
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
			},
			_=> (),
		};
		reqord_list
	}
	/// 取得該記錄中，指定欄位的值
	fn get_value(&self, rec: &Rec, field_name: &str) -> String {
		match self.tables.get(&rec.reqs_vec[2]) {
			Some(tabrec) => { 
				match tabrec.index.get(field_name) {
					Some(idx) => {
						return rec.reqs_vec[*idx].to_string();
					},
					_=> return String::new(),
				};
			},
			_=> return String::new(),
		};
	}
	/// 取得該筆LinkedList的彙總說明
	fn get_ord_summary(&self, list: &LinkedList<&Rec>) -> OrdInfo {
		let mut info = OrdInfo::new();
		let mut ordst :i32 = 0;
		//let mut reqst :i32 = 0;
		for rec in list {
			if rec.reqs_vec[0] == "Req" && rec.reqs_vec[4] == "1" { // 若是新單要求，則取流水號
				info.rid = self.get_value(rec, "SorRID");
				//reqst = rec.reqs_vec[6].parse::<i32>().unwrap();
			}
			if rec.reqs_vec[0] == "Ord" {
				info.ordno = self.get_value(rec, "OrdNo");
				let rec_ordst = rec.reqs_vec[7].parse::<i32>().unwrap();
				if rec_ordst > ordst {
					ordst = rec_ordst;
				}
			}
		}
		info.status = get_ordst(ordst);
		info
	}
	/// 印出指定 Ord Key 的 彙總以及 所有Log; 每筆Log會有timestamp
	pub fn print_ord(&self, key: &str) {
		let list = &self.get_target_ordlist(key);
		println!("{}", self.get_ord_summary(list).to_string() );
		for rec in list {
			rec.print();
		}
	}
	/// 以index, 找出ords中相等於target的rec
	pub fn find_req(&self, table_name: &str, key_index: usize, target: &str) {
		println!("find req {}, index={} ords:{}", target, key_index, self.ords.len());
		let mut found = false;
		for (key, rec) in &self.reqs  {
			if  rec.reqs_vec.len() < 3 || rec.reqs_vec[2] != table_name {
				continue;
			}
			if rec.reqs_vec.len() > key_index {
				if rec.reqs_vec[key_index] == target.to_string() {
					self.print_ord(&self.req2ord[key]);
					found = true;
				}
			} else {
				println!("line:{}, \n fields miss match.", rec.line);
			}
		};
		if !found {
			println!("{} not found", target);
		}
	}
	/// 以index, 找出reqs中相等於target的rec
	pub fn find_ord(&self, table_name: &str, key_index: usize, target: &str) {
		let mut found = false;
		for (key, list) in &self.ords {
			match list.back() {
				Some(rec) => {
					if  rec.reqs_vec.len() < 3 || rec.reqs_vec[2] != table_name {
						continue;
					}
					if rec.reqs_vec.len() > key_index {
						if rec.reqs_vec[key_index] == target.to_string() {
							self.print_ord(&rec.reqs_vec[1]);
							found = true;
						}
					} else {
						println!("line:{}, \n fields miss match.", rec.line);
					}
				},
				_=>println!("{} is empty", key),
			}
		}
		if !found {
			println!("{} not found", target);
		}
	}

	pub fn check_req_data(&self, table_name: &str, field_name: &str, search_target: &str) {
		println!("checking {}, {}", field_name, search_target);
		for (_, tab) in &self.tables {
			tab.print();
		}
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

// 4. 解析管理
pub struct Parser {
	ord_rec : OrderRec,
	info    : String,
	prevkey : (&'static str, String)
}

/// 陣列的操作函式
impl Parser {
	pub fn new()->Parser {
		Parser{ 
			ord_rec: OrderRec::new(),
			info   : String::new(),
			prevkey: ("", "".to_string()),
		}
	}

	///取得統計資訊
	pub fn get_info(&mut self) -> &str {
		if self.info.is_empty() {
			let mut deals = 0;
			let mut fails = 0;
			// 掃描req列表，統計
			for (_, req) in &self.ord_rec.reqs {
				if req.reqs_vec[4] == "10" || req.reqs_vec[4] == "11" {
					deals = deals + 1;
				}
			}
			// 掃描req列表，統計
			for (_, ord) in &self.ord_rec.ords {
				if ord.back().unwrap().reqs_vec[7] == "99" {
					fails = fails + 1;
				}
			}
			self.info = format!("tables:\t{}\nreqs:\t{}\nords:\t{}\ndeals:\t{}\nfailed:\t{}\n", 
				self.ord_rec.tables.len(), self.ord_rec.reqs.len(), self.ord_rec.ords.len(),
				deals, fails);
			&self.info
		}
		else {
			&self.info
		}
	}

	/// 解析每一行的內容, 並儲存到HashMap
	pub fn parse_line(&mut self, line: &str, log: &str) {
		let toks : Vec<String> = line.to_string().split('\x01').map(|s| s.to_string()).collect();

		if toks.len() > 3 {
			self.prevkey = self.ord_rec.insert_rec(toks, line, log);
		} else {
			//println!("log line: {}", line);
		}
	}
	
	/// 輸入 表名/欄位名/值 來尋找目標
	pub fn find_by_field(&mut self, table_name: &str, field_name: &str, search_target: &str) {
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
use structopt::StructOpt;
use std::io::*;
use std::io::{BufReader};
//use std::io::prelude::*;
use std::fs::File;

mod parser;
use crate::parser::*;

mod fileread;
use crate::fileread::*;

/// SorReqOrd Parser
/// 可從檔案中, 取得與指定欄位值相符的記錄

// 1.參數取得
#[derive(StructOpt)]
struct Options {
	/// 要解析的SorReqOrd.log
	filepath: String, // Log檔路徑
	/// 指定TableName:FieldName:SearchValue; 例如 -f TwsNew:SorRID:100001
	#[structopt(short="f", long="field", default_value = "")]
	field   : String,
}

/// 第一參數指定檔案
/// 將其讀入陣列以便解析
fn main() -> Result<()> {
	let options    = Options::from_args();

	let f          = File::open(options.filepath)?;
	let mut reader = BufReader::new(f);
	let mut parser = Parser::new();


	// 依每行解析
	read_data_log(&mut reader, &mut parser);

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


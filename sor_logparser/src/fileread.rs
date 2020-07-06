use std::io::*;
use std::io::{BufRead, BufReader};
use encoding::{Encoding, DecoderTrap};
use encoding::all::{ BIG5_2003, GB18030, ISO_2022_JP };

use crate::parser::Parser;

pub enum LineType<T> {
	EndOfFile,
	Rec(T),
	Log(T),
	Empty,
}

#[derive(PartialEq)]
enum EncodingType {
	BIG5,
	JP,
	GB,
	UTF8,
}

/// get line from reader
#[allow(dead_code)]
fn get_reader_line<R: Read>(reader: &mut BufReader<R>, encoding: &EncodingType) -> LineType<String> {
	let mut line_buf = Vec::<u8>::new();
	let mut line = String::new();
	// 讀第一行
	line_buf.clear();
	match reader.read_until(b'\n', &mut line_buf) {
		Ok(sz_line) => {
			if sz_line == 0 {
				return LineType::EndOfFile;
			}
			let mut dont_need_utf8 = true;
			if encoding != &EncodingType::UTF8 {
				dont_need_utf8 = match encoding {
					EncodingType::BIG5 => BIG5_2003.decode_to(&mut line_buf, DecoderTrap::Strict, &mut line).is_ok(),
					EncodingType::JP => ISO_2022_JP.decode_to(&mut line_buf, DecoderTrap::Strict, &mut line).is_ok(),
					EncodingType::GB => GB18030.decode_to(&mut line_buf, DecoderTrap::Strict, &mut line).is_ok(),
					_ => false,
				};
			}
			if !dont_need_utf8 {
				line = String::from_utf8_lossy(&line_buf).to_string();
			}
			line = line.trim().to_string();
			if sz_line < 2 || line.len() < 2 {
				return LineType::Empty;
			}
			if line.as_bytes()[0] == ':' as u8 {
				LineType::Log(line)
			} else {
				LineType::Rec(line)
			}
		},
		Err(_)=> LineType::EndOfFile,
	}
}

fn get_encoding_constant(encoding_opt: &str) -> EncodingType {
	match encoding_opt {
		"BIG5" => EncodingType::BIG5,
		"GB" => EncodingType::GB,
		"JP" => EncodingType::JP,
		_  => { println!("decod use UTF8"); EncodingType::UTF8 },
	}
}

/// line by line with log 解析
pub fn read_data_log<R: Read>(reader: &mut BufReader<R>, parser: &mut Parser, encoding_opt: &str) {
	let mut str_tmp: Option<String> = None;
	let encoding = get_encoding_constant(encoding_opt);
	loop {
		match get_reader_line(reader, &encoding) {
			LineType::Rec(line) => {
				match str_tmp {
					Some(rec) => parser.parse_line(&rec, ""),
					None      => (),
				};
				str_tmp = Some(line);
			},
			LineType::Log(log) => {
				match str_tmp {
					Some(rec) => parser.parse_line(&rec, &log),
					None => (),
				};
				str_tmp = None;
			},
			LineType::Empty     =>  continue,
			LineType::EndOfFile =>  break,
		};
	};
	match str_tmp {
		Some(rec) => parser.parse_line(&rec, ""),
		None      => (),
	};
}
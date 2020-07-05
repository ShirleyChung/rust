use std::io::*;
use std::io::{BufRead, BufReader};
use encoding::{Encoding, DecoderTrap};
use encoding::all::BIG5_2003;

use crate::parser::Parser;

pub enum LineType<T> {
	EndOfFile,
	Rec(T),
	Log(T),
	Empty,
}

/// get line from reader
#[allow(dead_code)]
pub fn get_reader_line<R: Read>(reader: &mut BufReader<R>) -> LineType<String> {
	let mut line_buf = Vec::<u8>::new();
	let mut line = String::new();
	// 讀第一行
	line_buf.clear();
	match reader.read_until(b'\n', &mut line_buf) {
		Ok(sz_line) => {
			if sz_line == 0 {
				return LineType::EndOfFile;
			}
			if !BIG5_2003.decode_to(&mut line_buf, DecoderTrap::Strict, &mut line).is_ok() {
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

/// line by line with log 解析
pub fn read_data_log<R: Read>(reader: &mut BufReader<R>, parser: &mut Parser) {
	let mut str_tmp: Option<String> = None;
	loop {
		match get_reader_line(reader) {
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
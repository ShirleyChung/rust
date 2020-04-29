use std::process::Command;   // 為了執行程式
use assert_cmd::prelude::*;  // 讓指令加上方法

#[test]
fn run_with_defaults()
-> Result<(), Box<dyn std::error::Error>>
{
	Command::cargo_bin("catsay")
	.expect("binary exists")
	.assert()
	.success();
	Ok(());
}


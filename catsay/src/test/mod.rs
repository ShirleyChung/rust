use std::process::Command;
use assert_cmd::prelude::*;

#[test]
fn run_with_defaults()
->Result<(), Box<dyn std::error::Error>> {
	Command::cargo_bin("catsay")
		.expect("binary exist")
		.assert()
		.success();
	Ok(())
}

pub fn do_plus(a: i32, b: i32) -> i32 {
	a + b
}

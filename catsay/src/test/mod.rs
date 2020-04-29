#![allow(unused_imports)]

use std::process::Command;
use assert_cmd::prelude::*;

pub fn say_some() {
	println!("some!");
}

#[test]
fn run_with_defaults()
->Result<(), Box<dyn std::error::Error>> {
	Command::cargo_bin("catsay")
		.expect("binary exist")
		.assert()
		.success();
	Ok(())
}

#[test]
fn run_with_v()
->Result<(), Box<dyn std::error::Error>> {
	Command::cargo_bin("catsay")
		.expect("binary exist")
		.args(&["--", "-V"])
		.assert()
		.success();
	Ok(())
}

#[test]
fn fail_on_nonexist_file()
->Result<(), Box<dyn std::error::Error>> {
	Command::cargo_bin("catsay")
		.expect("binary exist")
		.args(&["-f", "no_such_file.txt"])
		.assert()
		.failure();
	Ok(())
}

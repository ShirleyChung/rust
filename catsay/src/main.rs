use structopt::StructOpt;
use colored::*;

#[derive(StructOpt)]
struct Options {
#[structopt(default_value="I am rust meow~")]
/// what does cat say?
	message: String,
#[structopt(short="d", long="dead")]
/// make cat appear dead
	dead: bool,
#[structopt(short="f", long="file", parse(from_os_str))]
/// load the cat picture from the specified file
	catfile: Option<std::path::PathBuf>,
}

fn say_hello(msg: &String, dead: bool) {
	let msg = msg.yellow().underline();
	println!("{}", msg);
	println!(" \\");
	println!("  \\");
	println!("     /\\_/\\");
	let eye = if dead { "x" } else { "o" };
	println!("    ( {eye} {eye} )", eye=eye.red().bold());
	println!("    =( I )=");
}

fn main() {
	let options = Options::from_args();
	let message = options.message;
	if message == "woof" {
		eprintln!("a cat shouldn't bark like a dog!");
	}
	say_hello(&message, options.dead);
}
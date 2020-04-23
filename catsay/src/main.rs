use structopt::StructOpt;
use colored::*;

mod test;
use test::*;

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
	println!(" {0}+ {1} = {2}", 10, 20, do_plus(10, 20));
}

fn main() -> std::io::Result<()> {
	let options = Options::from_args();
	let message = options.message;
	if message == "woof" {
		eprintln!("a cat shouldn't bark like a dog!");
	}
	match &options.catfile {
		Some(path) => {
			let cat_template = std::fs::read_to_string(path)?;
			println!("{}", message);
			println!("{}", &cat_template);
		},
		None => {
			say_hello(&message, options.dead);
		}
	}
	Ok(())
}
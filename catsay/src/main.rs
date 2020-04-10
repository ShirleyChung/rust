use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
#[structopt(default_value="I am rust meow~")]
/// what does cat say?
	message: String,
#[structopt(short="d", long="dead")]
/// make cat appear dead
	dead: bool,
}

fn say_hello(msg: &String, dead: bool) {
	println!("{}", msg);
	println!(" \\");
	println!("  \\");
	println!("     /\\_/\\");
	let eye = if dead { 'x' } else { 'o' };
	println!("    ( {0} {0} )", eye);
	println!("    =( I )=");
}

fn main() {
	let options = Options::from_args();
	say_hello(&options.message, options.dead);
}
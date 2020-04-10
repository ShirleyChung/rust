use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
#[structopt(default_value="I am rust meow~")]
/// what does cat say?
	message: String
}

fn say_hello(msg: &String) {
	println!("{}", msg);
	println!(" \\");
	println!("  \\");
	println!("     /\\_/\\");
	println!("    ( o o )");
	println!("     =(I)=");
}

fn main() {
	let options = Options::from_args();
	let message = options.message;
	say_hello(&message);
}

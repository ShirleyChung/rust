fn long<'a>(a:&'a str, b:&'a str) -> &'a str {
	if a.len() > b.len() {
		a
	} else {
		b
	}
}

fn main() {
	let msg1 = String::from("123555");
	let longerstr;
	{
		let msg2 = String::from("1234");
		longerstr = long(&msg1, &msg2);
    println!("Hello, world! {}", longerstr);
	}
}


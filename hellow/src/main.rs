/* 展示lifetime的用法 */
fn long<'a>(a:&'a str, b:&'a str) -> &'a str {
	if a.len() > b.len() {
		a
	} else {
		b
	}
}

/* msg1的生命週期與msg2不同 */
/* 所以函式中，要指明以msg1的生命週期為主 */
fn main() {
	let msg1 = String::from("123555");
	for (i, c) in msg1.chars().enumerate() {
		println!("{} = {}", i, c);
	}
	let longerstr;
	{
		let msg2 = String::from("1234");
		longerstr = long(&msg1, &msg2);
    println!("Hello, world! {}", longerstr);
	}
}


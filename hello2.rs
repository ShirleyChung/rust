mod hello2 {

	fn is_true()->i32{ 100 }
	pub fn select()->fn()->i32{
		is_true
	}

	pub fn hello(x :i32)->i32 {
		println!("{:} hello!", x);
		32
	}

}

fn main() {
	let a:i32 = 10;
	let b:i32 = a;
	let c: &i32 = &b;
	println!("let c = {:}", c);

	hello2::hello(4);
	println!("what is select? {}", hello2::select()());
}

extern crate mytool;

struct Circle
{
	radius: f64,
}

trait Shape {
	fn area(&self)->f64;
}

impl Shape for Circle
{
	fn area(&self)->f64{
		std::f64::consts::PI*self.radius*self.radius
	}
}

fn main(){
	let x = 5;
	println!("Hello! Rust x ={}", x);
	let mut y = 3;
	y = y + 1;
	println!("hello! y = {}", y);
	let months = ["January", "febrary", "march", "April", "may", "June"];
	let c: Circle = Circle{
		radius:10.0,
	};
	let n = 12;
	for m in &months
	{
		println!("=>{}, {}, fab({})={}",m, c.area(), n, mytool::fabonicii(n));
	}
}

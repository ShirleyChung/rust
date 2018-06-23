struct Circle
{
	radius: f64,
}

impl Circle
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
	for m in &months
	{
		println!("=>{0}, {1}",m, c.area());
	}
}

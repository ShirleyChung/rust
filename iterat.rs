trait Order{
	fn set_price(&mut self, p:i32);
}

struct TFutOrderData{
	quantity: i32,
	price   : i32,
}

impl Order for TFutOrderData{
	fn set_price(&mut self, p:i32){
		self.price = p;
	}
}

fn math_test<F:Fn()->i32>(op :F){
	println!("magic num:{}", op());
}

fn main(){
	math_test(||32);

	let mut order:TFutOrderData = TFutOrderData {
		quantity: 1,
		price: 20
	};
	order.set_price(100);

	println!("order price = {}", order.quantity);
}

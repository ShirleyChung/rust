mod app {
	/// 若是3的倍數則輸出fizz, 若是5的倍數則輸出buzz,
    /// 若是3和5的倍數則輸出fizz buzz
	pub fn fizz_buzz(num :i32)->String {
		if num % 15 == 0 {
			return "fizz_buzz".to_string();
		}else if num % 3 == 0 {
			return "fizz".to_string();
		}else if num % 5 == 0 {
			return "buzz".to_string();
		}else {
			return num.to_string();
		}
	}
}

fn main() {
	println!("15 is {}", app::fizz_buzz(15));
}

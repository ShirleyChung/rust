fn range() {
	let r = 1..=6;
	for i in r {
		println!("{},", i);
	}
	
	let v:[i32; 3] = [4, 4, 2];
	for i in &v {
		println!("{}", i);
	}
}

fn main(){
	let mut v = vec![1,2,3,4,5];
/*	loop {
		match v.pop() {
		Some(x)=>println!("{}..", x),
		None   =>break,
		}
	}
*/
	while let Some(x) = v.pop() {
		println!("while let, next is {}", x);
	}

	range();
}


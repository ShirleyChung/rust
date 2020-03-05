fn main(){
	let mut x :i32 = 0;
	let ptr_x = &mut x as *mut i32;
	unsafe {
		*ptr_x = 10;
	}
	println!("x = {}", x);

	let mut arr_v = [1,2,3,4,5];
	println!("{:?}", arr_v);

	let ptr_v = &mut arr_v as *mut [i32;5];
	
	println!("{:?}, {:?}", arr_v, ptr_v);

}

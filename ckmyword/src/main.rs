fn main() ->std::result::Result<(), std::io::Error> {
	/* 讀取檔案 */
	let data = std::fs::read_to_string("test")?;
    println!("Hello, world! {:?}", data);

	/* 讀出每個字元 */
	for ch in data.chars() {
		print!("{},", ch);
	}
	
	println!("");

	/* 印出每個字元的索引 */
	for (i, ch) in data.chars().enumerate() {
		print!("({}:{}),", i, ch);
	}

	println!("");
	
	/* 準備出字串的陣列 */
	/* 因為String沒有實作Copy的trait, 所以要指定為slice的型別, 儲存字串的切片 */
	let mut strarr = Vec::<&str>::new();
	
	/* 將讀入的字串以3的大小切片 */
	let mut res :String = String::new();
	let mut spos = 0;
	let mut epos = 3;
	while epos < data.len() {
		res.push_str(&data[spos..epos]);
		strarr.push(&data[spos..epos]);
		spos = epos;
		epos = epos + 3;
		println!("{}", res);
	}
	
	Ok(())
}

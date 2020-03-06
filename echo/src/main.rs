use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

/// 一個Client的角色
struct Client<'a> {
	ip: &'a str,
}

/// Client的方法，new, connect, close
impl<'a> Client<'a> {
	fn new(ip: &'a str) -> Client {
		Client { ip: ip }
	}
	fn connect(&self) -> io::Result<()> {
		let client = TcpStream::connect(self.ip)?;
		println!("{:?} connected.", client);
		Ok(())
	}
	fn close(&self) -> io::Result<()> {
		Ok(())
	}
}

/// Echo Server，監聽一個port
struct Server {
	port: i32,
}

/// Echo Server的function
impl Server {
	/// 建立一個新的Server
	fn new(p: i32) -> Server {
		Server { port: p }
	}
	/// 處理進來的client
	fn handle_client(stream: TcpStream) -> std::io::Result<()> {
		println!("client {:?} connected!", stream);
		let mut reader = BufReader::new(&stream);
		let mut line   = String::new();
		while line != "exit" {
			line.clear();
			println!("waiting client input...");
			let _len = reader.read_line(&mut line)?;
			if line.ends_with('\n') {
				line.pop();
			}
			if line.ends_with('\r') {
				line.pop();
			}
			println!("client: {}", line);
		};
		Ok(())
	}
	/// 開始監聽
	fn listen(&self) -> io::Result<()> {
		let url = format!("127.0.0.1:{}", self.port);
		println!("begin listening {}", url);
		let listener = TcpListener::bind(url)?;
		for stream in listener.incoming() {
			Server::handle_client(stream?)?;
		}
		Ok(())
	}
}

/// 執行
fn main() -> io::Result<()> {

	let args: Vec<String> = env::args().collect();
	
	if args.len() > 1 {
		let target = &args[0];
		if let Some(_c) = target.find(':') {
			let echo_cli = Client::new(&target);
			println!("connect to {}", target);
			echo_cli.connect()?;
			echo_cli.close()?;
		} else if let Ok(p) = args[1].parse() {
			let echo_svr = Server::new(p);
		    echo_svr.listen()?;
		}
	} else {
		println!("please give 'port' as server or ip:port as client");		
		println!("for example: echo 8000");
	}
	Ok(())
}

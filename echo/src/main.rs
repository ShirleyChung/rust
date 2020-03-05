use std::net::{TcpListener, TcpStream};
use std::io;
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
		let _client = TcpStream::connect(self.ip)?;
		Ok(())
	}
	fn close() -> io::Result<()> {
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
	fn handle_client(_stream: TcpStream) -> std::io::Result<()> {
		Ok(())
	}
	/// 開始監聽
	fn listen(&self) -> io::Result<()> {
		let url = format!("127.0.0.1:{}", self.port);
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
	
	if args.len() > 2 {
		let target = &args[1];
		if let Some(_c) = target.find(':') {
			let echo_cli = Client::new(&target);
			println!("connect to {}", target);
			echo_cli.connect()?;
		} else if let Ok(p) = args[1].parse() {
			let echo_svr = Server::new(p);
			println!("listening on {}", p);
		    echo_svr.listen()?;
		}
	} else {
		println!("please give 'port' as server or ip:port as client");		
		println!("for example: echo 8000");
	}
	Ok(())
}
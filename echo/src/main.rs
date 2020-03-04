use std::net::{TcpListener, TcpStream};
		
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
	fn listen(&self) -> std::io::Result<()> {
		let url = format!("127.0.0.1:{}", self.port);
		let listener = TcpListener::bind(url)?;
		for stream in listener.incoming() {
			Server::handle_client(stream?)?;
		}
		Ok(())
	}
}

/// 執行
fn main() -> std::io::Result<()> {
	let echo_svr = Server::new(8080);
    echo_svr.listen()?;
    println!("Hello, world!");
    Ok(())
}
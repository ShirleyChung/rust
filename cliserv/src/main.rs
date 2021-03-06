use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

/// 處理進來的連線
fn handle_client(stream: TcpStream){
    println!("stream: {:?}", stream);
}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:31212").unwrap();

    for stream in listener.incoming(){
        handle_client(stream?);
        println!("Hello, world!");
    }
    Ok(())
}

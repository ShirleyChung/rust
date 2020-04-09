use std::net::{TcpListener, TcpStream};


fn handle_client(stream: TcpStream) {

}

fn main() {

    let listener = TcpListener::bind("127.0.0.1:1020").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => { }
        }
    }

}

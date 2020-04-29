#![allow(unused_imports)]
#![allow(dead_code)]

mod server;
use crate::server::*;

mod client;
use crate::client::*;

mod log;
use crate::log::*;

#[cfg(test)]
mod tests {
use crate::server::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
	
	#[test]
	fn test_connect() {
		server_listen();

	}
}

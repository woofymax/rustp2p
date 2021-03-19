#![feature(split_inclusive)]
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct Server {
	pub port: u32,
	peers: Vec<String>,
}

impl Server {
	pub fn init(port: u32, possible_peers: &mut Vec<String>) -> Server {
		let mut p: Vec<String> = vec![String::from(format!("127.0.0.1:{}", port))];
		p.append(possible_peers);
		return Server {
			port: port,
			peers: p,
		};
	}
	pub fn start(&mut self) {
		self.host();
	}
	pub fn updatePeers(&mut self, new: &mut Vec<String>) {
		self.peers.append(new);
	}

	fn host(&mut self) {
		let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).unwrap();
		for stream in listener.incoming() {
			let stream = stream.unwrap();

			self.handle_connection(stream);
		}
	}
	fn handle_connection(&mut self, mut stream: TcpStream) {
		let mut buffer = [0; 1024];
		stream.read(&mut buffer).unwrap();

		let send = b"send:";
		
		if buffer.starts_with(send) {
			self.message(&buffer);
		}


		let response = "HTTP/1.1 200 OK\r\n\r\n";

		stream.write(response.as_bytes()).unwrap();
		stream.flush().unwrap();
	}

	fn message(&mut self, buffer: &[u8]) {

		let message = buffer.split_inclusive(|num| num == ":".bytes().collect());
		
		println!("message {}", std::str::from_utf8(&buffer).unwrap());
	}
}

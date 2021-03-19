mod RServ;
use RServ::Server;

fn main() {
	let mut peers: Vec<String> = vec![String::from("127.0.0.1:1337")];
	let mut server = Server::init(31337, &mut peers);
	server.start();
}

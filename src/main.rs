use std::env;
use std::io::Read;
use std::net::{Shutdown, TcpStream};

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = if args.len() == 2 { &args[1] } else { "127.0.0.1:4325" };

    let mut stream = TcpStream::connect(address).expect("connect error");

    let mut buffer = [0; 1];
    let read = stream.read(&mut buffer).expect("read error");

    stream.shutdown(Shutdown::Both).expect("shutdown error");

    if read == 1 && buffer[0] == 0 {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}
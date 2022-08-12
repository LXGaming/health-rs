use std::{env, str};
use std::io::Read;
use std::net::{Shutdown, TcpStream};

const HEALTHY: u8 = 0;
const UNHEALTHY: u8 = 1;
const MAXIMUM_STRING_SIZE: u32 = 4096; // 4 KiB

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = if args.len() == 2 { &args[1] } else { "127.0.0.1:4325" };

    let mut stream = TcpStream::connect(address).expect("connect error");
    let mut buffer = Vec::new();
    let mut handle = stream.by_ref().take(u64::from(1 + MAXIMUM_STRING_SIZE));
    let read = handle.read_to_end(&mut buffer).expect("read error");

    stream.shutdown(Shutdown::Both).expect("shutdown error");

    let state: bool;
    let mut message: Option<&str> = None;
    match read {
        0 => state = false,
        1 => state = buffer[0] == HEALTHY,
        _ => {
            state = buffer[0] == HEALTHY;
            message = Some(str::from_utf8(&buffer[1..]).expect("Invalid UTF-8 sequence"));
        }
    }

    if message != None {
        print!("{}", message.unwrap());
    }

    if state {
        std::process::exit(i32::from(HEALTHY));
    } else {
        std::process::exit(i32::from(UNHEALTHY));
    }
}
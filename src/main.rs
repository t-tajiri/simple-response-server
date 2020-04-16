use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listner = TcpListener::bind("localhost:7878").unwrap();
    // listner.set_nonblocking(true).expect("can not set non-blocking");

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];


    stream.read(&mut buffer).unwrap();

    println!("Request {}", String::from_utf8_lossy(&buffer[..]));
}

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listner = TcpListener::bind("localhost:7878").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];


    stream.read(&mut buffer).unwrap();
    let response_info = String::from_utf8_lossy(&buffer[..]);

    stream.write("HTTP/1.1 200 OK\n".as_bytes()).unwrap();
    stream.write("Content-Type: text/html; charset=UTF-8\n".as_bytes()).unwrap();
    stream.write("Content-Length: 1000\n\n".as_bytes()).unwrap();
    let ip = format!("{}", &stream.peer_addr().unwrap());
    stream.write(ip.as_bytes()).unwrap();
    stream.write("\n".as_bytes()).unwrap();
    stream.write("\n".as_bytes()).unwrap();
    stream.write(response_info.as_bytes()).unwrap();
    stream.write("\n".as_bytes()).unwrap();
    stream.write("\n".as_bytes()).unwrap();
    stream.flush().unwrap();
}

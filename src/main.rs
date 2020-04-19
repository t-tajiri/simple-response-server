extern crate chrono;

use chrono::prelude::*;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

pub fn main() {
    let listner = TcpListener::bind("localhost:7878").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        thread::spawn(move || {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    send_response(&stream, &buffer);
}

fn send_response(mut stream: &TcpStream, buffer: &[u8; 1024]) {
    let response_info = String::from_utf8_lossy(&buffer[..]);
    let ip = format!("{}", &stream.peer_addr().unwrap());
    let date = Utc::now()
        .with_timezone(&FixedOffset::east(9 * 3600))
        .format("%Y年%m月%d日 %H時%M分%S秒");

    let content = format!(
        "<!doctype html>
                           <html>
                                <head></head>
                                <body>
                                    <h1>request info</h1>
                                    <h2>remote access address</h2>
                                    </div>{}</div>
                                    <h2>request date</h2>
                                    <div>{}</div>
                                    <h2>header info</h2>
                                    <div>{}</div>
                                </body>
                           </html>\n\n",
        ip, date, response_info
    );

    create_response_header(&stream, &content);

    create_response_content(&stream, &content);

    stream.flush().unwrap();
}

fn create_response_header(mut stream: &TcpStream, content: &String) {
    stream.write("HTTP/1.1 200 OK\n".as_bytes()).unwrap();

    stream
        .write("Content-Type: text/html; charset=UTF-8;\n".as_bytes())
        .unwrap();

    stream.write("Content-Length: ".as_bytes()).unwrap();

    // FIXME refactor converting usize to &[u8]
    let content_length = format!("{}", content.len());
    stream.write(content_length.as_bytes()).unwrap();

    stream.write(";\n\n".as_bytes()).unwrap();
}

fn create_response_content(mut stream: &TcpStream, content: &String) {
    stream.write(content.as_bytes()).unwrap();
}

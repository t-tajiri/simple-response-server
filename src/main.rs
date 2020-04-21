extern crate chrono;

use chrono::prelude::*;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

pub fn main() {
    let listner = TcpListener::bind("localhost:7878").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1];

    stream.read(&mut buffer).unwrap();

    send_response(&stream, &buffer);
}

fn send_response(mut stream: &TcpStream, buffer: &[u8; 1]) {
    let response_info = String::from_utf8(buffer[..].to_vec()).unwrap();
    let response_info = response_info.trim_matches(char::from(0));

    let ip = format!("{}", &stream.peer_addr().unwrap());

    let date = Utc::now()
        .with_timezone(&FixedOffset::east(9 * 3600))
        .format("%Y年%m月%d日 %H時%M分%S秒");

    let content = format!(
        "<!DOCTYPE html>\r\n<html>\r\n<head></head>\r\n
                <body>\r\n
                    <h1>request info</h1>\r\n
                    <h2>remote access address</h2>\r\n
                    <div>{}</div>\r\n
                    <h2>request date</h2>\r\n
                    <div>{}</div>\r\n
                    <h2>header info</h2>\r\n
                </body>\r\n
            </html>\r\n",
        ip, date
        // , response_info
    );

    create_response_header(&stream, &content);

    create_response_content(&stream, &content);

    stream.flush().unwrap();
}

fn create_response_header(mut stream: &TcpStream, content: &String) {
    stream.write("HTTP/1.1 200 OK\r\n".as_bytes()).unwrap();

    stream.write("Content-Type: text/html; charset=utf-8\r\n".as_bytes()).unwrap();

    stream.write("Content-Length: ".as_bytes()).unwrap();

    // FIXME refactor converting usize to &[u8]
    let content_length = format!("{}", content.len());
    stream.write(content_length.as_bytes()).unwrap();
    stream.write("\r\n".as_bytes()).unwrap();

    stream.write("Connection: close;\r\n".as_bytes()).unwrap();

    stream.write("\r\n\r\n".as_bytes()).unwrap();
}

fn create_response_content(mut stream: &TcpStream, content: &String) {
    stream.write(content.as_bytes()).unwrap();
}

use std::net::TcpListener;

fn main() {
    let listner = TcpListener::bind("localhost:7878").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        println!("hello");
    }
}

use std::net::TcpListener;

const HOST:&str = "127.0.0.1:7878";

fn main() {
    let listener = TcpListener::bind(HOST).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}

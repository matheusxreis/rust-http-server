use http_server::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread::{self, Thread},
    time::Duration,
};

const HOST: &str = "127.0.0.1:7878";

fn main() {
    let listener = TcpListener::bind(HOST).unwrap();

    let mut pool: Option<ThreadPool> = None;

    if let Ok(p) = ThreadPool::build(13) {
        pool = Some(p);
    }
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        match pool {
            Some(ref p) => {
                p.execute(|| {
                    handle_connection(stream);
                });
            }
            _ => {}
        }
    } // with take(2), after 2 items the looping will stop
    // and the ThreadPool will go out of scope 
    // and then ThreadPool Drop implementation will execute
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "templates/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "templates/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "templates/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

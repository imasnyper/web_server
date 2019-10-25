use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

use web_server::ThreadPool;

static BASE_DIRECTORY: &str = "C:/Users/Shop/Documents/scripts/rust/web_server/";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let hello = Path::new(BASE_DIRECTORY).join("src/hello.html");
    let error = Path::new(BASE_DIRECTORY).join("src/404.html");

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", hello)
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", hello)
    } else {
        ("HTTP/1.1 400 NOT FOUND\r\n\r\n", error)
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
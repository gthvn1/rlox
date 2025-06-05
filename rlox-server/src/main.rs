use std::io::{BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

use rlox_server::http_message::{request::Request, response::to_response};

fn handle_connection(stream: TcpStream) {
    // Please note that each call to read() may involve a system call, and therefore,
    // using something that implements BufRead, such as BufReader, will be more efficient.
    let reader = stream.try_clone().expect("failed to clone stream");
    let mut stream_writer = stream;
    let mut stream_reader = BufReader::new(reader);

    let response = match Request::from_stream(&mut stream_reader) {
        Ok(http_msg) => to_response(http_msg),
        Err(_) => b"HTTP/1.1 500 Internal Server Error\r\n\r\n".to_vec(),
    };

    stream_writer
        .write_all(&response)
        .expect("failed to write response");
}

fn main() {
    const URL: &str = "127.0.0.1:1234";

    println!("Listen on {}", URL);

    let listener = TcpListener::bind(URL).expect("failed to bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread;

use rlox_server::http_message::HttpMessage;

fn handle_connection(stream: TcpStream) {
    // Please note that each call to read() may involve a system call, and therefore,
    // using something that implements BufRead, such as BufReader, will be more efficient.
    let mut stream_reader = BufReader::new(stream);

    match HttpMessage::from_request(&mut stream_reader) {
        Ok(_) => println!("Successfuly read HttpMessage"),
        Err(_) => println!("Failed ti read HttpMessage"),
    }
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

// https://datatracker.ietf.org/doc/html/rfc9112
//   HTTP-message   = start-line CRLF
//                    *( field-line CRLF )
//                    CRLF
//                    [ message-body ]

use std::error::Error;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

const CRLF: &str = "\r\n";

pub struct HttpMessage {
    start_line: String,
    field_line: Vec<String>, // can be empty
    msg_body: Vec<u8>,       // Body can be gzip for example
}

pub enum HttpMessageError {
    StartLineNotFound,
}

impl HttpMessage {
    pub fn from_request(req: &mut BufReader<TcpStream>) -> Result<Self, HttpMessageError> {
        // An HTTP/1.1 message consists of a start-line followed by a CRLF.
        // The normal procedure for parsing an HTTP message is to read the
        // start-line into a structure.
        let mut start_line = String::new();
        let sz = match req.read_line(&mut start_line) {
            Err(e) => {
                eprintln!("Failed to read start_line: {}", e);
                return Err(HttpMessageError::StartLineNotFound);
            }
            Ok(sz) => sz,
        };

        println!("Read {} bytes: <{:?}>", sz, start_line);

        Ok(HttpMessage {
            start_line,
            field_line: Vec::new(),
            msg_body: Vec::new(),
        })
    }
}

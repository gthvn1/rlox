// https://datatracker.ietf.org/doc/html/rfc9112
//   HTTP-message   = start-line CRLF
//                    *( field-line CRLF )
//                    CRLF
//                    [ message-body ]

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

const CRLF: &str = "\r\n";

pub struct HttpMessage {
    start_line: String,
    fields: HashMap<String, String>, // can be empty
    body: Vec<u8>,                   // Body can be gzip for example
}

pub enum HttpMessageError {
    StartLineNotFound,
    StartLineWrongEnd,
    FieldLineWrongEnd,
    FieldLineParseError,
    BodyLenReadError,
    BodyLenParseError,
}

impl HttpMessage {
    pub fn from_request(req: &mut BufReader<TcpStream>) -> Result<Self, HttpMessageError> {
        // An HTTP/1.1 message consists of a start-line followed by a CRLF.
        // The normal procedure for parsing an HTTP message is to read the
        // start-line into a structure.
        let mut start_line = String::new();
        let sz = req.read_line(&mut start_line).map_err(|e| {
            eprintln!("Failed to read start_line: {}", e);
            HttpMessageError::StartLineNotFound
        })?;

        println!("Read {} bytes: <{:?}>", sz, start_line);

        // Sanity check: ensure that we read start-line
        if !start_line.ends_with(CRLF) {
            return Err(HttpMessageError::StartLineWrongEnd);
        }

        let mut fields: HashMap<String, String> = HashMap::new();
        loop {
            let mut field_line = String::new();
            let _sz = req.read_line(&mut field_line).unwrap_or(0);

            if field_line.is_empty() || field_line == CRLF {
                // We reached the end of fields
                break;
            }

            if !field_line.ends_with(CRLF) {
                return Err(HttpMessageError::FieldLineWrongEnd);
            }

            let (f_name, f_value) = field_line
                .split_once(':')
                .ok_or(HttpMessageError::FieldLineParseError)
                .map(|(s1, s2)| (s1.trim().to_lowercase().to_string(), s2.trim().to_string()))?;

            // TODO: A server MUST reject, with a response status code of 400 (Bad Request),
            // any received request message that contains whitespace between a header field
            // name and colon. So don't trim as we did and check... but currently we don't
            // care.
            println!("Field added: <{}>/<{}>", f_name, f_value);
            fields.insert(f_name, f_value);
        }

        // There is a body if content-lenght is present. Otherwise just return
        // an empty vec.
        let body: Vec<u8> = if let Some(value) = fields.get("content-length") {
            let body_len = value
                .parse::<usize>()
                .map_err(|_| HttpMessageError::BodyLenParseError)?;

            let mut body = vec![0u8; body_len];
            req.read_exact(&mut body)
                .map_err(|_| HttpMessageError::BodyLenReadError)?;
            body
        } else {
            Vec::new()
        };

        Ok(HttpMessage {
            start_line,
            fields,
            body,
        })
    }
}

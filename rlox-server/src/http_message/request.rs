// https://datatracker.ietf.org/doc/html/rfc9112
//   HTTP-message   = start-line CRLF
//                    *( field-line CRLF )
//                    CRLF
//                    [ message-body ]

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

const CRLF: &str = "\r\n";

#[allow(dead_code)]
pub struct Request {
    request_line: String,
    fields: HashMap<String, String>, // can be empty
    body: Vec<u8>,                   // Body can be gzip for example
}

pub enum RequestError {
    StartLineNotFound,
    StartLineWrongEnd,
    FieldLineWrongEnd,
    FieldLineParseError,
    BodyLenReadError,
    BodyLenParseError,
}

impl Request {
    pub fn from_stream(req: &mut BufReader<TcpStream>) -> Result<Self, RequestError> {
        // An HTTP/1.1 message consists of a start-line followed by a CRLF.
        // The normal procedure for parsing an HTTP message is to read the
        // start-line into a structure.
        let mut request_line = String::new();
        let sz = req.read_line(&mut request_line).map_err(|e| {
            eprintln!("Failed to read request_line: {}", e);
            RequestError::StartLineNotFound
        })?;

        println!("Read {} bytes: <{:?}>", sz, request_line);

        // Sanity check: ensure that we read start-line
        if !request_line.ends_with(CRLF) {
            return Err(RequestError::StartLineWrongEnd);
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
                return Err(RequestError::FieldLineWrongEnd);
            }

            let (f_name, f_value) = field_line
                .split_once(':')
                .ok_or(RequestError::FieldLineParseError)
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
                .map_err(|_| RequestError::BodyLenParseError)?;

            let mut body = vec![0u8; body_len];
            req.read_exact(&mut body)
                .map_err(|_| RequestError::BodyLenReadError)?;
            body
        } else {
            Vec::new()
        };

        Ok(Request {
            request_line,
            fields,
            body,
        })
    }

    // A request-line begins with a method token, followed by a single space (SP),
    // the request-target, and another single space (SP), and ends with the protocol
    // version.
    //
    // request-line   = method SP request-target SP HTTP-version

    pub fn get_method(&self) -> Option<&str> {
        self.request_line.split_whitespace().next()
    }

    pub fn get_path(&self) -> Option<&str> {
        self.request_line.split_whitespace().nth(1)
    }
}

use std::fs;

use crate::http_message::request::Request;

pub fn to_response(req: Request) -> Vec<u8> {
    match req.get_path() {
        Some("/css/style.css") => serve_file("./static/css/style.css", "text/css"),
        Some("/rlox_web.js") => serve_file("./static/rlox_web.js", "text/javascript"),
        Some("/rlox_web_bg.wasm") => serve_file("./static/rlox_web_bg.wasm", "application/wasm"),
        Some("/favicon.ico") => serve_file("./static/favicon/favicon-32x32.png", "image/png"),
        Some("/") | Some("/index.html") => serve_file("./static/index.html", "text/html"),
        Some(path) => {
            eprintln!("Cannot serve path ({})", path);
            todo!()
        }
        None => {
            eprintln!("Cannot serve path (None)");
            todo!()
        }
    }
}

fn serve_file(path: &str, ty: &str) -> Vec<u8> {
    let mut res = Vec::new();

    match fs::read(path) {
        Ok(bytes) => {
            // res line
            res.extend_from_slice(b"HTTP/1.1 200 OK\r\n");
            // Fields
            res.extend_from_slice(format!("Content-type: {}\r\n", ty).as_bytes());
            res.extend_from_slice(format!("Content-Length: {}\r\n", bytes.len()).as_bytes());
            // End of headers
            res.extend_from_slice(b"\r\n");
            // Body
            res.extend_from_slice(&bytes);

            res
        }
        Err(_) => {
            let body = b"404 Not Found";
            res.extend_from_slice(b"HTTP/1.1 404 Not Found\r\n");
            res.extend_from_slice(b"Content-Type: text/plain\r\n");
            res.extend_from_slice(format!("Content-Length: {}\r\n", body.len()).as_bytes());
            res.extend_from_slice(b"\r\n");
            res.extend_from_slice(body);
            res
        }
    }
}

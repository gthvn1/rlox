use std::fs;

use crate::http_message::request::Request;

pub const PATH_INDEX_HTML: &str = "/index.html";
pub const PATH_ROOT: &str = "/";
pub const PATH_STYLE_CSS: &str = "/css/style.css";
pub const PATH_JS: &str = "/rlox_web.js";
pub const PATH_WASM: &str = "/rlox_web_bg.wasm";
pub const PATH_FAVICON: &str = "/favicon.ico";

pub const FILE_INDEX_HTML: &str = "./static/index.html";
pub const FILE_STYLE_CSS: &str = "./static/css/style.css";
pub const FILE_JS: &str = "./static/rlox_web.js";
pub const FILE_WASM: &str = "./static/rlox_web_bg.wasm";
pub const FILE_FAVICON: &str = "./static/favicon/favicon-32x32.png";

pub const TYPE_HTML: &str = "text/html";
pub const TYPE_CSS: &str = "text/css";
pub const TYPE_JS: &str = "text/javascript";
pub const TYPE_WASM: &str = "application/wasm";
pub const TYPE_PNG: &str = "image/png";

pub fn to_response(req: Request) -> Vec<u8> {
    match req.get_path() {
        Some(PATH_ROOT) | Some(PATH_INDEX_HTML) => serve_file(FILE_INDEX_HTML, TYPE_HTML),
        Some(PATH_STYLE_CSS) => serve_file(FILE_STYLE_CSS, TYPE_CSS),
        Some(PATH_JS) => serve_file(FILE_JS, TYPE_JS),
        Some(PATH_WASM) => serve_file(FILE_WASM, TYPE_WASM),
        Some(PATH_FAVICON) => serve_file(FILE_FAVICON, TYPE_PNG),
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

use crate::http_message::request::Request;

const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Welcome</title>
</head>
<body>
  <h1>Hello from RLox!</h1>
</body>
</html>
"#;

pub fn to_response(req: Request) -> Vec<u8> {
    let mut response: Vec<u8> = Vec::new();

    // response line
    response.extend_from_slice(b"HTTP/1.1 200 OK\r\n");

    // Fields
    response.extend_from_slice(b"Content-type: text/html\r\n");
    response.extend_from_slice(format!("Content-Length: {}\r\n", INDEX_HTML.len()).as_bytes());

    // End of headers
    response.extend_from_slice(b"\r\n");

    // Body
    response.extend_from_slice(INDEX_HTML.as_bytes());

    // Before returning just print some info from req
    println!("method: {}", req.get_method().unwrap());
    println!("path  : {}", req.get_path().unwrap());

    response
}

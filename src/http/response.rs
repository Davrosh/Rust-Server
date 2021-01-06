use std::io::{Result as IoResult, Write};
use std::net::TcpStream;


use super::StatusCode;
#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    // can use dyn Write and impl Write
    // dyn Write resolves the correct write method to call at runtime using a
    // v-table
    // impl Write looks at our codebase and for every call to send with a
    // parameter of some type, it generates a new function with the stream
    // argument as that type - resolves at compile time
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}



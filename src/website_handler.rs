use std::fs;

use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        WebsiteHandler { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);


        // prevent an attacker from including .. in their file path and get
        // access to our server files: resolve the final path specified and see
        // that it starts with the path to our public folder
        match fs::canonicalize(path) {
            Ok(path) => {
                // canonicalize again fixes on windows
                if path.starts_with(fs::canonicalize(&self.public_path).unwrap()) {
                    // convert Ok to Some and Err to None
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }

    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => {
                    // Response::new(StatusCode::Ok,
                    // Some("<h1>Welcome</h1>".to_string()))

                    Response::new(StatusCode::Ok, self.read_file("index.html"))
                }
                "/hello" => {
                    // Response::new(StatusCode::Ok,
                    // Some("<h1>Hello</h1>".to_string()))

                    Response::new(StatusCode::Ok, self.read_file("hello.html"))
                }
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None),
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }

        // Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
    }
}

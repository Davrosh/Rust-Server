// modules are like c++ namespaces
// everything in a module, including sub-modules, structs and functions is
// private by default

// use in mod server;

// crate is the root
use crate::http::{Request, ParseError, Response, StatusCode};
use std::convert::TryFrom;
use std::convert::TryInto;

use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    // default implementation - may be overridden
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}
pub struct Server {
    addr: String,
}

impl Server {
    // Self is an alias for the struct name - Server
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    // borrow self and destroy the instance after running the function
    // in our case, we just want run() to run forever so we don't care
    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        // if the result is o.k, unwrap returns it.
        // otherwise, unwrap terminates the program
        let listener = TcpListener::bind(&self.addr).unwrap();

        // break from outer loop in rust
        // 'outer: loop {
        //     loop {
        //         break 'outer;
        //     }
        // }

        loop {
            // hangs until connection is established
            match listener.accept() {
                Ok((mut stream, _)) => {
                    // init an array of size 1024 with initial value of 0
                    let mut buffer = [0; 1024];

                    // pass mutable reference to self with mut stream
                    // pass (mutable) reference to array to not have to specify
                    // length
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            // lossy cannot fail, it will convert the buffer to
                            // a string and whatever character it cannot, it
                            // will convert to ?
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            // we need to convert to slice with [..]
                            let response = match Request::try_from(&buffer[..]) {
                                // request has lifetime of buffer
                                Ok(request) => {
                                    // cannot modify buffer because request borrows
                                    // it and uses it later on
                                    // buffer[0] = 1;
                                    // let a = request;

                                    dbg!(&request);
                                    // Response::new(
                                    //     StatusCode::Ok,
                                    //     Some("<h1>It Works!</h1>".to_string()),
                                    // )
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                    // Response::new(StatusCode::BadRequest, None)
                                    handler.handle_bad_request(&e)
                                },
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to parse a request: {}", e);
                            }

                            // compiler cannot guess what type try_into's
                            // implementation is using so we tell it that it's
                            // a request explicitly
                            // let res: Result<Request, _> = buffer[..].try_into();
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}

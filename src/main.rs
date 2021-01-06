// suppress dead code warnings
// ! denotes that this applies to the whole module and all sub-modules
#![allow(dead_code)]

use http::Request;
mod http;

use server::Server;
mod server;

use std::env;

use website_handler::WebsiteHandler;
mod website_handler;

fn main() {
    // this is a String, which has a pointer to the string allocated on the heap
    // and a length and capacity (slots allocated)
    let string = String::from("127.0.0.1:8080");

    // this is a &str, a slice of a string (a string view) which has a pointer
    // to the part of the string the slice corresponds to and a length - it it a
    // borrow
    let string_slice = &string[10..];

    // in Rust, we can implicitly convert &String to &str, even by simply
    // passing a &String to a function receiving a &str
    let string_borrow: &str = &string;

    // treated as a &str - a slice to the entire string
    let string_literal = "1234";


    // need to pass it as a reference because string_slice is borrowing it
    dbg!(&string);
    dbg!(string_slice);
    dbg!(string_borrow);
    dbg!(string_literal);

    // a slice range uses bytes and not chars, so we might slice the middle of a
    // char and get an error
    // let emoji = String::from("🏠☝ש⚾");
    // let emoji_slice = &emoji[..10];
    // dbg!(emoji_slice);
    
    // add /public to the end of the path
    // env! allows us to access several predefined environment variables
    // CARGO_MANIFEST_DIR is the directory of cargo.toml
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

    // allows us to specify an environment variable of our own, defaults to the
    // above value if it was not specified
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);

    // convert &str to String
    let server = Server::new("127.0.0.1:3000".to_string());
    server.run(WebsiteHandler::new(public_path));

    
}



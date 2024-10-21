
mod http;

use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};
use std::str::FromStr;
use crate::http::request::{Method, Request};

fn main() {

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.expect("Failed to read line"))
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {http_request:#?}");


    let request = http::request::Request::from_str(http_request[0].as_str()).unwrap();
    println!("Request: {:#?}", request);

    let mut status_line = "";
    let mut filepath = "";

    let routes = vec![
        ("/", "src/index.html"),
        ("/about", "src/about.html"),
        ("/contact", "src/contact.html"),
    ];

    let contents = fs::read_to_string(filepath).unwrap();
    let length = contents.len();
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

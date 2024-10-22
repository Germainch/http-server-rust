
mod http;

use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}};
use std::cmp::PartialEq;
use std::str::FromStr;
use crate::http::request::{Method, Request};
use crate::http::request::Method::{GET, POST};
use crate::http::response;
use crate::http::response::Response;
use crate::http::route::Route;

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

fn handle_get_index(request: Request, mut response: Response){
    println!("{request:#?}");
    response = Response::from_file(200, "src/index.html")
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.expect("Failed to read line"))
        .take_while(|line| !line.is_empty())
        .collect();
    // println!("Request: {http_request:#?}");


    let request = Request::from_str(http_request[0].as_str()).unwrap();
    let route = (request.method.clone(), request.uri.as_str());
    let response = Response::new(200, "Hello, World!".to_string());


    let mut filepath = "src/index.html";

    let routes: Vec<Route> = vec![
        Route::get("/", handle_get_index),
    ];

    for r in routes {
        if r.method == route.0 && r.uri == route.1 {
            (r.handler)(request, response);
            return;
        }
    }

    stream.write_all(response.into_string().as_bytes()).unwrap();
}

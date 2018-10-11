extern crate hyper;
extern crate pretty_env_logger;

use hyper::{Client, Server};
use hyper::{Body, Method, Request, Response, StatusCode};
use hyper::client::ResponseFuture;
use hyper::service::service_fn;
use hyper::rt::{self, Future};
use std::net::SocketAddr;

use futures::future;

static INDEX: &[u8] = b"<html><body><form action=\"post\" method=\"post\">Name: <input type=\"text\" name=\"name\"><br>Number: <input type=\"text\" name=\"number\"><br><input type=\"submit\"></body></html>";
static NOTFOUND: &[u8] = b"Not Found";

fn main() {
    pretty_env_logger::init();

    let in_addr = ([127, 0, 0, 1], 30001).into();
    let server_addr: SocketAddr = ([127, 0, 0, 1], 8000).into();

    let client_main = Client::new();

    let server_addr_clone = server_addr.clone();
    // new_service is run for each connection, creating a 'service'
    // to handle requests for that specific connection.
    let new_service = move || {
        let client = client_main.clone();
        // This is the `Service` that will handle the connection.
        // `service_fn_ok` is a helper to convert a function that
        // returns a Response into a `Service`.
        service_fn(move |mut req| {
            let uri_string = format!("http://{}/{}",
                server_addr_clone,
                req.uri().path_and_query().map(|x| x.as_str()).unwrap_or(""));
            let uri = uri_string.parse().unwrap();
            *req.uri_mut() = uri;
            //println!("{:?}", req);
            //println!("{:?}", req.method());
            match req.method() {
                &Method::POST => {println!("{:?}", req.method()); client.request(req)},
                &Method::GET => {client.request(req)},
                _ => {
                    // Return 404 not found response.
                    let body = Body::from(NOTFOUND);
                    let resp404 = Request::builder()
                                     //.status(StatusCode::NOT_FOUND)
                                     .body(body)
                                     .unwrap();
                    println!("{:?}", req.method());
                    client.request(resp404)
                    }


            }
            //print!("{:?}", req.type);
            //client.request(req)
        })
    };

    let server = Server::bind(&in_addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", in_addr);
    println!("Proxying on http://{}", server_addr);

    rt::run(server);
}

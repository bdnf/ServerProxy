extern crate regex;
use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::fs::File;
use regex::Regex;

extern crate time;
use time::*;

mod threadpool;
use threadpool::ThreadPool;

extern crate futures;
extern crate hyper;
extern crate tokio_core;

use futures::{Future};
use hyper::{Client, Uri};
use tokio_core::reactor::Core;

enum Verb {
    GET,
    POST,
    OPTIONS,
    PUT,
    DELETE
}
struct Header {
    key: String,
    value: String
}

struct Request {
    verb: String,
    hostname: String,
    path: String,
    protocol: String,
    client_headers: Vec<Header>
}

fn read_request(stream: &mut TcpStream) -> Request{
    let mut buf: [u8; 1024] = [0; 1024]; // if this is declared mutable here, why below too?
    let resp_bytes = b"HTTP/1.0 200 OK\r\nContent-Type: text/plain\r\n\r\nOK";
    stream.read(&mut buf);
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if resp_bytes.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let client_headers_buf = String::from_utf8_lossy(&mut buf);
    let mut request_iterator = client_headers_buf.split("\r\n");
    let request_line: &str = request_iterator.next().unwrap();

    let request_tokens: Vec<&str> = request_line.split(" ").collect();
    let url: &str = request_tokens[1];
    let version: &str = request_tokens[2];

    let re = Regex::new(r"(\w*?)://(.*?)/(.*)").unwrap();
    let caps = re.captures(url).unwrap();
    let host = caps.get(2);
    let mut request = Request {
        verb: request_tokens[0].to_string(),
        hostname: match caps.get(2) {
            Some(host) => {caps.get(2).unwrap().as_str().to_string()}
            None => { "".to_string() }
        },
        path: match caps.get(3) {
            Some(host) => {caps.get(3).unwrap().as_str().to_string()}
            None => { "".to_string() }
        },
        protocol: match caps.get(1){
            Some(host) => {caps.get(1).unwrap().as_str().to_string()}
            None => { "".to_string() }
        },
        client_headers: Vec::new()
    };

    // Not a big fan of this; would rather define the request once and somehow point the
    // structures vector at this one.
    for header in request_iterator { // iterator is already past request line
        if header.len() > 0 {
            let tokens: Vec<&str> = header.splitn(2, ": ").collect();
            if tokens.len() == 2 {
                request.client_headers.push(Header {
                    key: tokens[0].to_string(),
                    value: tokens[1].to_string()
                });
            }
        }
    }
    return request;
}

fn log_request(request: &Request) {
    let t = now();
    print!("{:?}", t);
    println!("[{}-{:02}-{:02} {:02}:{:02}:{:02}.{:04}] {} {} \"/{}\"",
             t.tm_year + 1900,
             t.tm_mon + 1,
             t.tm_mday,
             t.tm_hour,
             t.tm_min,
             t.tm_sec,
             t.tm_nsec,
             request.verb,
             request.hostname,
             request.path);
}

fn send_request(request: &Request, stream: &mut TcpStream) {
    // Send actual request
    let request_line = format!("{} /{} HTTP/1.1\r\n", request.verb, request.path);
    stream.write(&request_line.into_bytes());

    // Send all client headers
    for header in request.client_headers.iter() {
        let outbound_header = format!("{}: {}\r\n", header.key, header.value);
        stream.write(&outbound_header.into_bytes());
    }

    stream.write(b"Connection: close\r\n");
    stream.write(b"\r\n");
}

fn handle_connection(mut incoming_stream: TcpStream) {

    //https://stackoverflow.com/questions/50312240/get-request-to-a-website-with-vanilla-rust

    //let request = read_request(&mut incoming_stream);
    //log_request(&request);
    println!("Local port addr equal {:?}", incoming_stream.local_addr().unwrap().port() );
    let request_ip = incoming_stream.local_addr().unwrap().ip();
    let request_port = incoming_stream.local_addr().unwrap().port();

    let address_string = format!("{}:{}", request_ip, 8000);
    print!("{:?}", &address_string );
    let mut server_stream = TcpStream::connect(&address_string).unwrap();
    println!("{:?}",&server_stream );

    // println!("Incoming stream ..." );
    // let mut reader = BufReader::new(&incoming_stream);
    // let mut response = String::new();
    // reader.read_line(&mut response).expect("Could not read");
    // println!("Player received >{}<", response.trim());
    //
    // let mut writer = BufWriter::new(&incoming_stream);
    // let req = writer.write_all("NAME\n".as_bytes()).expect("Could not write");
    // println!("request = {:?}", req);


    //handle_client(server_stream);


    let mut request_data = String::new();
    request_data.push_str("GET / HTTP/1.0");
    request_data.push_str("\r\n");
    request_data.push_str("Host: 127.0.0.1");
    request_data.push_str("\r\n");
    request_data.push_str("Connection: close"); // <== Here!
    request_data.push_str("\r\n");
    request_data.push_str("\r\n");

    println!("request_data = {:?}", &request_data);
    let request = server_stream.write_all(request_data.as_bytes()).unwrap();
    println!("request = {:?}", request);

    let mut buf = String::new();
    let result = server_stream.read_to_string(&mut buf).unwrap();
    println!("result = {}", result);
    println!("buf = {}", buf);


    //send_request(&request, &mut server_stream);
    //
    // let mut content_buffer: Vec<u8> = Vec::new();
    // let content_size = server_stream.read_to_end(&mut content_buffer).unwrap();
    // print!("{:?}", &content_buffer);
    // incoming_stream.write(&content_buffer);

      //let mut req_buffer = [0; 512];
      //let mut req_buffer = incoming_stream;
     // stream.read(&mut req_buffer).unwrap();
     //
     // let get = b"GET / HTTP/1.1\r\n";
     // let (status_line, filename) = if req_buffer.starts_with(get) {
     //     ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
     // } else {
     //     ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
     // };
     //
     // let mut file = File::open("serve.html").unwrap();
     // let mut contents = String::new();
     // file.read_to_string(&mut contents).unwrap();
     //
     // let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
     //
     // stream.write(response.as_bytes()).unwrap();
     // stream.flush().unwrap();
}

fn print_req(){
    let mut core = Core::new().unwrap();

      let client = Client::new(&core.handle());

      let url : Uri = "http://httpbin.org/response-headers?foo=bar".parse().unwrap();
      assert_eq!(url.query(), Some("foo=bar"));

      let request = client.get(url)
          .map(|res| {
              assert_eq!(res.status(), hyper::Ok);
          });

      // request is a Future, futures are lazy, so must explicitly run
      //core.run(request).unwrap();
}

fn handle_client(stream: TcpStream) {
        println!("Client connected");

        let mut writer = BufWriter::new(&stream);
        writer.write_all("Red\n".as_bytes()).expect("could not write");
        writer.flush().expect("could not flush");

        let mut reader = BufReader::new(&stream);
        let mut response = String::new();
        reader.read_line(&mut response).expect("could not read");
        println!("Server received {}", response);
        print!("{:?}",reader );
    }

fn main() {
    let addr: String = "127.0.0.1:8888".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();
    //let pool = ThreadPool::new(4);

    println!("Listening on 127.0.0.1:{}",
             &listener.local_addr().unwrap().port());


    for stream in listener.incoming() {
        //println!("Listening on 127.0.0.1");

     //let stream = stream.unwrap();

     //handle_connection(stream);
     match stream {

                 Ok(stream) => {
                            println!("Current stream is {:?}",&stream );
                            handle_connection(stream);
                            //handle_client(stream);
                            //print_req();
                             println!("no err");

                 }
                 Err(_) => { /* connection failed */
                  print!("Some err");}
                }

    }
    println!("Shutting down server!");
}

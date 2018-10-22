use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs::File;

mod threadpool;
use threadpool::ThreadPool;

fn heavy_work() -> String {
    let duration = std::time::Duration::from_millis(200);
    std::thread::sleep(duration);
    "done".to_string()
}

fn handle_connection(mut stream: TcpStream) {
     let mut req_buffer = [0; 512];
     stream.read(&mut req_buffer).unwrap();

     let get = b"GET / HTTP/1.1\r\n";
     println!("{:?}", &stream );

     // let (status_line, filename) = if req_buffer.starts_with(get) {
     //     ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
     //
     // } else {
     //     ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
     // };

     if req_buffer.starts_with(get) {
         ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
         //build_get_req(&stream);
     } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        //build_error_resp(stream);
     };


     let mut file = File::open("serve.html").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap();
     let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

     stream.write(response.as_bytes()).unwrap();
     stream.flush().unwrap();

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
         let stream = stream.unwrap();

         //handle_connection(stream);
         //and multithreaded
         pool.execute(|| {
                handle_connection(stream);
         });
    }
    println!("Shutting down server!");
}

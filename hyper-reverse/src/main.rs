extern crate futures;
extern crate hyper;
extern crate hyper_reverse_proxy;
extern crate tokio_core;

use futures::Stream;
use hyper::Client;
use hyper::server::Http;
use hyper_reverse_proxy::ReverseProxy;
use tokio_core::net::TcpListener;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use std::net::{SocketAddr, Ipv4Addr};


fn run() -> hyper::Result<()> {
    // Set up the Tokio reactor core
    let mut core = Core::new()?;
    let handle = core.handle();

    // Set up a TCP socket to listen to
    let listen_addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 8080);
    //let listen_addr = listen_addr.parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&listen_addr, &handle)?;

    // Listen to incoming requests over TCP, and forward them to a new `ReverseProxy`
    let http = Http::new();

    //let server_addr = env::args().nth(2).unwrap_or("127.0.0.1:8000".to_string());
    //let server_addr = server_addr.parse::<SocketAddr>().unwrap();
    let server_addr = SocketAddr::new(Ipv4Addr::new(127, 0, 0, 1).into(), 8000);
    //let listen_addr = listen_addr.parse::<SocketAddr>().unwrap();
    let client = TcpStream::connect(&server_addr, &handle);
  //
  //   let done = socket.incoming().for_each(move |(client, client_addr)| {
  //     let server = TcpStream::connect(&server_addr, &handle);
  //     let amounts = server.and_then(move |server| {
  //         // Create separate read/write handles for the TCP clients that we're
  //         // proxying data between. Note that typically you'd use
  //         // `AsyncRead::split` for this operation, but we want our writer
  //         // handles to have a custom implementation of `shutdown` which
  //         // actually calls `TcpStream::shutdown` to ensure that EOF is
  //         // transmitted properly across the proxied connection.
  //         //
  //         // As a result, we wrap up our client/server manually in arcs and
  //         // use the impls below on our custom `MyTcpStream` type.
  //         let client_reader = MyTcpStream(Arc::new(client));
  //         let client_writer = client_reader.clone();
  //         let server_reader = MyTcpStream(Arc::new(server));
  //         let server_writer = server_reader.clone();
  //
  //         // Copy the data (in parallel) between the client and the server.
  //         // After the copy is done we indicate to the remote side that we've
  //         // finished by shutting down the connection.
  //         let client_to_server = copy(client_reader, server_writer)
  //             .and_then(|(n, _, server_writer)| {
  //                 shutdown(server_writer).map(move |_| n)
  //             });
  //
  //         let server_to_client = copy(server_reader, client_writer)
  //             .and_then(|(n, _, client_writer)| {
  //                 shutdown(client_writer).map(move |_| n)
  //             });
  //
  //         client_to_server.join(server_to_client)
  //     });
  //
  //     let msg = amounts.map(move |(from_client, from_server)| {
  //         println!("client at {} wrote {} bytes and received {} bytes",
  //                  client_addr, from_client, from_server);
  //     }).map_err(|e| {
  //         // Don't panic. Maybe the client just disconnected too soon.
  //         println!("error: {}", e);
  //     });
  //     handle.spawn(msg);
  //
  //     Ok(())
  // });

    let server = listener.incoming().for_each(|(socket, addr)| {
        //let client = TcpStream::connect(&server_addr, &handle);
        let client = Client::new(&handle);
        let service = ReverseProxy::new(client, Some(server_addr.port()));
        http.bind_connection(&handle, socket, server_addr, service);
        Ok(())
    });

    // Start our server on the reactor core
    core.run(server).unwrap();
    //core.run(done).unwrap();
    Ok(())
}

fn main() {
    use std::io::{self, Write};

    if let Err(error) = run() {
        write!(&mut io::stderr(), "{}", error).expect("Error writing to stderr");
        std::process::exit(1);
    }
}

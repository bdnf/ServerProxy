extern crate hyper;
extern crate pretty_env_logger;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use hyper::{Body, Chunk, Client, Method, Request, Response, Server, StatusCode, header};
use hyper::client::HttpConnector;

use hyper::client::ResponseFuture;
use hyper::service::service_fn;
//use hyper::rt::{self, Future};
use std::net::SocketAddr;

extern crate futures;
use futures::{future, Future, Stream};
//use futures::future;


use std::collections::HashMap;

extern crate url;
use url::form_urlencoded;

extern crate jsonwebtoken as jwt;
use jsonwebtoken::{encode, decode,decode_header, Header, Algorithm, Validation};
use jsonwebtoken::errors::{ErrorKind};


#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: String,
    username: String,
    profileAccessLevel: String
}
#[derive(Serialize, Deserialize, Debug)]
struct User2 {
    username: String
}

mod models;
mod parser;
//use crate::parser::resources::INDEX;
use crate::parser::resources:: *;
//mod resources;

fn response_examples(mut req: Request<Body>, client: &Client<HttpConnector>, server_addr: SocketAddr)
    -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {

    let key: String = "jhfdlksjlkfjlksdjfljsdlj".to_owned(); //jwt token secret key

    let server_addr_clone = server_addr.clone();
    let uri_string = format!("http://{}{}",
        server_addr_clone,
        req.uri().path_and_query().map(|x| x.as_str()).unwrap_or(""));
    let uri = uri_string.parse().unwrap();
    *req.uri_mut() = uri;

    //copy of request may be neede in future
   //  let (method, uri, version, headers, body) = req.deconstruct();
   // let total = body.concat2().wait().unwrap();
   // println!("{:?}", total);
    let uri2 = req.uri().clone();
    let method2 = req.method().clone();
    let version2 = req.version().clone();
    let headers = req.headers().clone();
    let body = req.body().clone();

    let new_res = parser::match_request(req, &client);

    new_res

   }



fn main() {
    // let user = models::User {
    //     name: "Andre".to_string(),
    //     group: Some(models::Group {
    //         group_name: "Admin".to_string(),
    //         allowed_verbs: vec!["GET".to_string(), "POST".to_string(), "DELETE".to_string()],
    //     }),
    //     id: 10001,
    // };
    // let jjson = serde_json::to_string(&user).expect("Couldn't serialize config");
    // println!("{}", jjson);

    //ROUTES.iter().map(|&x| { print!("{:?}", x );});
    for (i,r) in ROUTES.iter().enumerate() {
        print!("{:?} ", r );

    }

    //println!("{}", ROUTES.iter().fold(String::new(), |acc, &arg| acc + arg));



    pretty_env_logger::init();

    let in_addr = ([127, 0, 0, 1], 30001).into();
    let server_addr: SocketAddr = ([127, 0, 0, 1], 1331).into();
    //let backup_addr: SocketAddr = ([127, 0, 0, 1], 1331).into();


    hyper::rt::run(future::lazy(move || {
    let client_main = Client::new();

    // new_service is run for each connection, creating a 'service'
    // to handle requests for that specific connection.

    let new_service = move || {
            // Move a clone of `client` into the `service_fn`.
            let client = client_main.clone();
            service_fn(move |req| {
                response_examples(req, &client, server_addr)
            })
        };

/*
    let new_service = move || {
        let client = client_main.clone();
        // This is the `Service` that will handle the connection.
        // `service_fn_ok` is a helper to convert a function that
        // returns a Response into a `Service`.
        service_fn(move |mut req| {
            let uri_string = format!("http://{}{}",
                server_addr_clone,
                req.uri().path_and_query().map(|x| x.as_str()).unwrap_or(""));
            let uri = uri_string.parse().unwrap();
            *req.uri_mut() = uri;
            //println!("{:?}", req);
            //println!("{:?}", req.method());
            match req.method() {
                &Method::POST => {
                    //println!("{:?}", req.method());
                    //client.request(req);
                    let uri_string = format!("http://{}{}",
                        backup_addr_clone,
                        req.uri().path_and_query().map(|x| x.as_str()).unwrap_or(""));
                    let uri = uri_string.parse().unwrap();


                    let (mut parts, body) = req.into_parts();
                    parts.method = Method::GET;
                    parts.uri = uri;
                    let request = Request::from_parts(parts, body);


                    println!("{:?}", request);
                    client.request(request)
                },
                &Method::GET => {
                    println!("{:?}", req);
                    client.request(req)
                    },
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
*/
    let server = Server::bind(&in_addr)
        .serve(new_service)
        .map_err(|e| eprintln!("server error: {}", e));


        let my_claims = User2 {

              username: "testname8".to_owned(),

          };
          /*
         ISSUE: panicked at 'called `Result::unwrap()` on an `Err` value: Error(ExpiredSignature)'
        let key = "secret".to_owned();

        let token = encode(&Header::default(), &my_claims, key.as_ref()).unwrap();
        println!("{:?}", token);

        let token_data = decode::<User2>(&token, key.as_ref(), &Validation::default()).unwrap();
        //let token_data = decode_header(&token);
        println!("{:?}", token_data);
        //println!("{:?}", token_data.header);
        */

    println!("Listening on http://{}", in_addr);
    println!("Proxying on http://{}", server_addr);

    server
    }));
}

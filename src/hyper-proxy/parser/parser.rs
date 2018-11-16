extern crate futures;
use futures::{future, Future, Stream};
use hyper::{Body, Chunk, Method, Request, Response, StatusCode, header};
use hyper::Client;
use hyper::client::HttpConnector;
use std::collections::HashMap;

extern crate url;
use url::form_urlencoded;

extern crate base64;

use crate::parser::models::{RBAC, User};
use crate::parser::resources::*;


fn render_error_page(msg:  &'static [u8]) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    Box::new(future::ok(Response::builder()
    .status(StatusCode::NOT_FOUND)
    .body(Body::from(msg))
    .unwrap()))
}

pub fn match_request(mut req: Request<Body>, client: &Client<HttpConnector>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {

    // check if req.uri().path() is in ROUTES
    if ROUTES.contains(&req.uri().path()) {
    		println!("The route {} is known.", &req.uri().path());
		}
    // else {
    //     println!("The route {} is not known.", &req.uri().path());
    //     return Box::new(future::ok(Response::builder()
    //                                  .status(StatusCode::NOT_FOUND)
    //                                  .body(Body::from(NOTFOUND))
    //                                  .unwrap()))
    // }
    //if ok then
    match (req.method(), req.uri().path()) {

       (&Method::GET, HOMEPAGE) | (&Method::GET, "/index.html") => {
           //just redirecting to server
           Box::new(client.request(req))

       },
       (&Method::POST, "/uppercase") => {
            //Uppercases the body and returns it back.
            //Prints the body
            let body = Body::wrap_stream(req.into_body().map(|chunk| {
                // uppercase the letters
                let upper = chunk.iter().map(|byte| {
                    //println!(" {:?} ", byte.to_ascii_uppercase() );
                    byte.to_ascii_uppercase()
                })
                    .collect::<Vec<u8>>();
                println!("_ {:?}", String::from_utf8_lossy(&upper).into_owned() );
                Chunk::from(upper)
            }));
            println!("the body {:?}", body );
            Box::new(future::ok(Response::new(body)))
        },
        (&Method::POST, "/post") => {
               println!("{:?}", req);
            Box::new(req.into_body().concat2().map(|b| {
                // Parse the request body. form_urlencoded::parse
                // always succeeds, but in general parsing may
                // fail (for example, an invalid post of json), so
                // returning early with BadRequest may be
                // necessary.
                //
                // Warning: this is a simplified use case. In
                // principle names can appear multiple times in a
                // form, and the values should be rolled up into a
                // HashMap<String, Vec<String>>. However in this
                // example the simpler approach is sufficient.
                let mut params = form_urlencoded::parse(b.as_ref()).into_owned().collect::<HashMap<String, String>>();

                // Validate the request parameters, returning
                // early if an invalid input is detected.
                //println!("{:?}", &params );
                params.insert(1.to_string(), "a".to_string());
                {
                    let name = "\"name\"".to_string();
                        println!("Getting: {:?}", params.get(&name) );
                    for (contact, number) in params.iter() {
                        println!("Calling {}: {}", contact, number);
                    }
                }

                let name = if let Some(n) = params.get("name") {
                    println!("{:?}", &n );
                    n
                } else {
                    println!("cannot parse name");
                    return Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(MISSING.into())
                        .unwrap();
                };
                let number = if let Some(n) = params.get("number") {
                    if let Ok(v) = n.parse::<f64>() {
                        println!("{:?}", &v );
                        v
                    } else {
                        println!("cannot parse number");
                        return Response::builder()
                            .status(StatusCode::UNPROCESSABLE_ENTITY)
                            .body(NOTNUMERIC.into())
                            .unwrap();
                    }
                } else {
                    println!("cannot parse anything");
                    return Response::builder()
                        .status(StatusCode::UNPROCESSABLE_ENTITY)
                        .body(MISSING.into())
                        .unwrap();
                };

                // Render the response. This will often involve
                // calls to a database or web service, which will
                // require creating a new stream for the response
                // body. Since those may fail, other error
                // responses such as InternalServiceError may be
                // needed here, too.
                let body = format!("Hello {}, your number is {}", name, number);
                Response::new(body.into())
            }))
        },
        (&Method::GET, "/test") => {
               // build the request and change the path
               let request_uri = req.uri();
               let upstream_uri = format!("http://{}:{}{}",
                   req.uri().host().unwrap(),
                   req.uri().port().unwrap(),
                   "/"
                   ).parse().unwrap();

                println!("{:?}", upstream_uri);
                *req.uri_mut() = upstream_uri;


                println!("{:?}", req);
                let web_res_future = client.request(req);


                Box::new(web_res_future)

        },
       (&Method::GET, "/test.html") => {
               // Run a web query against the web api below

               // build the request
               let req = Request::builder()
                   .method(Method::POST)
                   //.uri(uri_string) <- need to be passed from server_addr
                   .body(LOWERCASE.into())
                   .unwrap();
               // use the request with client
               let web_res_future = client.request(req);

               Box::new(web_res_future.map(|web_res| {
               //web_res_future.map(|web_res| {
                   // return the response that came from the web api and the original text together
                   // to show the difference
                   let body = Body::wrap_stream(web_res.into_body().map(|b| {
                       Chunk::from(format!("<b>before</b>: {}<br><b>after</b>: {}",
                                           std::str::from_utf8(LOWERCASE).unwrap(),
                                           std::str::from_utf8(&b).unwrap()))
                   }));

                   Response::new(body)
               }))
        },

        (&Method::GET, "/json") => {
               let data = vec!["foo", "bar"];
               let res = match serde_json::to_string(&data) {
                   Ok(json) => {
                       let users: User = serde_json::from_str(&json).unwrap();
                       println!("{:?}", users );
                       // return a json response
                       Response::builder()
                           .header(header::CONTENT_TYPE, "application/json")
                           .body(Body::from(json))
                           .unwrap()
                   }
                   // This is unnecessary here because we know
                   // this can't fail. But if we were serializing json that came from another
                   // source we could handle an error like this.
                   Err(e) => {
                       eprintln!("serializing json: {}", e);

                       Response::builder()
                           .status(StatusCode::INTERNAL_SERVER_ERROR)
                           .body(Body::from("Internal Server Error"))
                           .unwrap()
                   }
               };

               Box::new(future::ok(res))
         },

         (&Method::POST, "/json") => {
               let chunks = vec![
                "hello",
                " ",
                "world",
            ];

            let stream = futures::stream::iter_ok::<_, ::std::io::Error>(chunks);

            let body = Body::wrap_stream(stream);


             Box::new(future::ok(Response::builder()
                 .header(header::CONTENT_TYPE, "application/json")
                 .body(body)
                 .unwrap()
             ))
          },
          (&Method::GET, SIGN_IN) => {
                //let data: Vec<&str> = value.split("/").skip(1).collect();
                let uri2 = req.uri().clone();
                let method2 = req.method().clone();
                let version2 = req.version().clone();
                let headers = req.headers().clone();
                let body = req.body().clone();
                //print!("{:?}", req );
                print!("Headers: {:?}", headers );
               Box::new(client.request(req))
               //client.request(&req)

          },
          (&Method::POST, SIGN_IN) => {


               //println!("{:?}", req.body());
               println!("{:?}", req.headers().get("authorization"));
              // let token = req.headers().get("authorization").unwrap().to_str();
               if let Some(token) = req.headers().get("authorization"){
                   //print!("{:?}", token.to_str() );
                   match token.to_str()  {
                       Ok(x) => {
                           let auth_header: Vec<&str> = x.split_whitespace().collect();
                       let bytes = base64::decode(auth_header[1]).unwrap();
                       println!("{:?}", std::str::from_utf8(&bytes));
                   },
                    Err(e) => print!("{} ", e)
                };
               };

        //println!("{:?}", req.body());
        //        let newBody = req.body_mut().map_err(|_| ()).fold(vec![], |mut acc, chunk| {
        //          acc.extend_from_slice(&chunk);
        //          Ok(acc)
        //      }).and_then(|v| {
        //     let stringify = String::from_utf8(v).unwrap();
        //     println!("{}", stringify);
        //     Ok(stringify)
        // });
            //.and_then(|v| String::from_utf8(v).map_err(|_| ()));

             //let response = Response::new();
        //let mut headers = Headers::new();
        let body = req.body_mut().concat2().map(|chunk| {
        let body = chunk.to_vec();
        println!("{:?}", body);
        body
    });
        //let body: &hyper::Body = &mut req.body_mut();

             // body.concat2()
             //    .and_then(|body| {
             //        let vec = body.iter().cloned().collect();
             //        let stringify = String::from_utf8(vec).unwrap();
             //        println!("{}", stringify);
             //        Ok(stringify)
             //    }).boxed();
                //println!("Body: \n{:?}", body.wait().unwrap());

           /*
               let newBody = req.body_mut().map_err(|_| ()).fold(vec![], |mut acc, chunk| {
                 acc.extend_from_slice(&chunk);
                 Ok(acc)
               }).and_then(|v| String::from_utf8(v).map_err(|_| ()).wait().unwrap());

               Box::new(future::ok(Response::builder()
                   .header(header::CONTENT_TYPE, "application/json")
                   .body(Body::from(newBody))
                   .unwrap()
               ))
           */
            //println!("{:?}",Box::new(client.request(req)));

             // Box::new(future::ok(Response::builder()
             //     .header(header::CONTENT_TYPE, "application/json")
             //     .body(body)
             //     .unwrap()))

              Box::new(client.request(req))
        },
        //matching over complex UNKNOWN routes
        (&Method::GET, value) => {
               println!("{:?}", req.headers().get("authorization"));
               println!("{:?}", req.body());
               let data: Vec<&str> = value.split("/").skip(1).collect();
               //let data = vec!["user", "pass"];
               //let index: Option<usize> = Some(data.iter().position(|&x| x == "user").unwrap());

               if data.len() >=2 {
               match (data[0], data[1])  {
                    ("api","users",) => {
                        println!("{:?}", req);
                        Box::new(client.request(req))},
                    //TODO later
                    // ("api","messages",) => {
                    //     println!("{:?}", req.headers().get("authorization"));
                    //     let mut token = req.headers().get("authorization").unwrap().to_str();
                    //
                    //     match token {
                    //         Ok(x) => {
                    //             // HMAC using SHA-256
                    //             let auth_header: Vec<&str> = x.split_whitespace().collect();
                    //             println!("token for decoding is{}", auth_header[1]);
                    //             let token_data = match decode::<User>(auth_header[1], key.as_ref(), &Validation::new(Algorithm::HS256)) {
                    //                 Ok(c) => c,
                    //                 Err(err) => match *err.kind() {
                    //                     ErrorKind::InvalidToken => panic!(), // Example on how to handle a specific error
                    //                     _ => panic!()
                    //                 }
                    //             };
                    //
                    //             println!("Token data is{:?}", token_data);
                    //         }
                    //         _ => ()
                    //     };
                    //     Box::new(client.request(req))
                    // },

                    (_,_) => {
                        let idx = match data.iter().position(|&x| x == "user" || x =="users") {
                            Some(index) => {

                                println!("You probably a {:?} with id: {:?} \n", data[index], data[index+1] );
                                let s = format!("You probably trying {:?} with id: {:?}" , data[index].to_string(), data[index+1].to_string());
                                &data[index..=index+1]
                            },
                            None => &data,
                        };

                        let res = match serde_json::to_string(idx) {

                            Ok(ref js) if idx.len() == 2 =>
                             Response::builder()
                                .header(header::CONTENT_TYPE, "application/json")
                                .body(Body::from(format!("You probably a {:?} with id: {:?}" , idx[0].to_string(), idx[1].to_string())))
                                .unwrap()
                                ,
                                Ok(ref json) if idx.len() > 2 =>
                                    // return a json response
                                    Response::builder()
                                        .header(header::CONTENT_TYPE, "application/json")
                                        .body(Body::from(json.clone()))
                                        .unwrap()
                                        ,


                             Ok(_) =>
                                 Response::builder()
                                    .header(header::CONTENT_TYPE, "application/json")
                                    .body(Body::from(format!("You probably a requested a /")))
                                    .unwrap()
                                    ,
                            // This is unnecessary here because we know
                            // this can't fail. But if we were serializing json that came from another
                            // source we could handle an error like this.
                            Err(e) => {
                                eprintln!("serializing json: {}", e);

                                Response::builder()
                                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                                    .body(Body::from("Internal Server Error"))
                                    .unwrap()
                            }
                        };

                        Box::new(future::ok(res))
                    }
               }

           }  else { render_error_page(b"Error reaching un-existing route") }
       }
       //wildard from main matcher
       _ => { render_error_page(NOTFOUND) }
      }
}

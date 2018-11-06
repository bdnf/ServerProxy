extern crate futures;
use futures::{future, Future, Stream};
use hyper::{Body, Method, Request, Response, StatusCode, header};

pub mod resources;
use crate::parser::resources:: *;
//use crate::parser::resources::NOTFOUND;

pub fn match_request(req: &Request<Body>) -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {

    // check if req.uri().path() is in ROUTES
    if ROUTES.contains(&req.uri().path()) {
    		println!("The pet {} is known.", &req.uri().path());
		}
    // else {
    //     println!("The pet {} is not known.", &req.uri().path());
    //     return Box::new(future::ok(Response::builder()
    //                                  .status(StatusCode::NOT_FOUND)
    //                                  .body(Body::from(NOTFOUND))
    //                                  .unwrap()))
    // }
    //if ok then
    match (req.method(), req.uri().path()) {

           (&Method::GET, HOMEPAGE) => {
               let body = Body::from(INDEX);
               Box::new(future::ok(Response::new(body)))
           },
           (&Method::GET, "/auth") => {
               let body = Body::from(INDEX);
               Box::new(future::ok(Response::new(body)))
           },
           _ => {
               // Return 404 not found response.
               let body = Body::from(NOTFOUND);
               Box::new(future::ok(Response::builder()
                                            .status(StatusCode::NOT_FOUND)
                                            .body(body)
                                            .unwrap()))
           }
       }
}

extern crate hyper;
extern crate http;
extern crate hyper_proxy;
extern crate futures;
extern crate tokio;
extern crate typed_headers;

use hyper::{Chunk, Client, Request, Method, Uri};
use hyper::client::HttpConnector;
use futures::{Future, Stream};
use hyper_proxy::{Proxy, ProxyConnector, Intercept};
use tokio::runtime::current_thread::Runtime;
use typed_headers::Credentials;

fn main() {
    let mut core = Runtime::new().unwrap();

    let proxy = {
        let proxy_uri = "http://localhost:8000".parse().unwrap();
        let mut proxy = Proxy::new(Intercept::All, proxy_uri);
        //proxy.set_authorization(Credentials::basic("John Doe", "Agent1234").unwrap());
        let connector = HttpConnector::new(4);
        let proxy_connector = ProxyConnector::from_proxy(connector, proxy).unwrap();
        proxy_connector
    };

    // Connecting to http will trigger regular GETs and POSTs.
    // We need to manually append the relevant headers to the request
    let uri: Uri = "https://google.com".parse().unwrap();
    let mut req = Request::get(uri.clone()).body(hyper::Body::from(vec![])).unwrap();
    if let Some(headers) = proxy.http_headers(&uri) {
        req.headers_mut().extend(headers.clone().into_iter());
    }
    let client = Client::builder().build(proxy);
    let fut_http = client.request(req)
        .and_then(|res| res.into_body().concat2())
        .map(move |body: Chunk| ::std::str::from_utf8(&body).unwrap().to_string());

    // Connecting to an https uri is straightforward (uses 'CONNECT' method underneath)
    let uri = "https://google.com".parse().unwrap();
    let fut_https = client
        .get(uri)
        .and_then(|res| res.into_body().concat2())
        .map(move |body: Chunk| ::std::str::from_utf8(&body).unwrap().to_string());

    let futs = fut_http.join(fut_https);

    let (_http_res, _https_res) = core.block_on(futs).unwrap();
}

extern crate futures;
extern crate hyper;

use futures::future::FutureResult;
use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service, Request, Response, Server, NewService};
use hyper::Body;
use std::fmt::Display;
use std::result;

static PHRASE: &'static [u8] = b"Hello World!";

#[derive(Clone, Copy)]
pub struct MyService {}

impl Service for MyService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;
    fn call(&self, _req: Request) -> Self::Future {
        return futures::future::ok(Response::new()
                                       .with_header(ContentLength(PHRASE.len() as u64))
                                       .with_header(ContentType::plaintext())
                                       .with_body(PHRASE));
    }
}

#[derive(Clone)]
pub struct MyServer {}

#[derive(Debug)]
pub struct MyServeError(String);
impl<T: Display> From<T> for MyServeError {
    fn from(e: T) -> MyServeError {
        return MyServeError(format!("{}", e));
    }
}

type Result<T> = result::Result<T, MyServeError>;


impl MyServer {
    pub fn new() -> MyServer {
        return MyServer {};
    }

    fn get_server(&self) -> Result<Server<&MyServer, Body>> {
        let addr = format!("127.0.0.1:8080").parse()?;
        return Ok(Http::new().bind(&addr, self)?);
    }
}

impl NewService for MyServer {
    type Request = Request;
    type Response = Response;
    type Instance = MyService;
    type Error = hyper::Error;

    fn new_service(&self) -> std::io::Result<Self::Instance> {
        let service = MyService {};
        Ok(service)
    }
}

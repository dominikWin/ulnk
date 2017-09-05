use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

pub fn start() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(APIServiceTest)).unwrap();
    server.run().unwrap();
}

struct APIServiceTest;

impl Service for APIServiceTest {
    type Request = Request;
    type Response = Response;
    type Error = ::hyper::Error;
    type Future = ::futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, _req: Request) -> Self::Future {
        info!("Recieved request from {}", _req.remote_addr().unwrap());
        let out = "Hello";
        let rsp = Response::new().with_header(ContentLength(out.len() as u64)).
            with_body(out);
        ::futures::future::ok(rsp)
    }
}

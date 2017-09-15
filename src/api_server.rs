extern crate iron;
use self::iron::prelude::*;
use self::iron::status;

pub fn start() {
    info!("Starting API server");
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello world!")))
    }).http("localhost:3000").unwrap();
}

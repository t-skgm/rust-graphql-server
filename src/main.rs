extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

static SERVER_ADDR: &str = "localhost:3000";

fn root(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello, world")))
}

fn main() {
    let mut router = Router::new();
    router.get("/", root, "root");

    let _server = Iron::new(router).http(SERVER_ADDR).unwrap();
    println!("Starting server on http://{}", SERVER_ADDR)
}

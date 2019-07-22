extern crate iron;
extern crate router;

use iron::prelude::*;
use router::router;

mod handlers;

static SERVER_ADDR: &str = "localhost:3000";

fn main() {
    let router = router!(
        index: get "/" => handlers::index,
        index_name: get "/:name" => handlers::index_name
    );
    let _server = Iron::new(router).http(SERVER_ADDR).unwrap();
    println!("Starting server on http://{}", SERVER_ADDR)
}

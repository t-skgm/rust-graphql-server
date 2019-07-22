use iron::prelude::{Request, Response, IronResult};
use iron::status;
use router::Router;

// get `/`
pub fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello, world")))
}

// get `/:name`
pub fn index_name(req: &mut Request) -> IronResult<Response> {
    let ref name = req.extensions.get::<Router>().unwrap()
      .find("name").unwrap_or("/");
    Ok(Response::with((
      status::Ok,
      format!("Hi, {}!", *name))))
}

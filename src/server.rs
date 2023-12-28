use iron::Iron;
use router::Router;

use crate::{FrostRouter, renderer};


pub struct FrostSever {
  host: String,
  port: u16,
}

impl FrostSever {
  pub fn new(host: &str, port: u16) -> Self {
    Self {
      host: host.to_string(),
      port
    }
  }

  pub fn start(&self, router: FrostRouter) {
    let routes = router.routes();
    let mut router = Router::new();

    routes.iter().for_each(|(route, path)| {
      
      let handler = renderer::render_handler(path.to_string(), route.to_string());
      router.get(route, handler, path);
    });

    Iron::new(router).http(format!("{}:{}", self.host, self.port)).unwrap();
  }
}

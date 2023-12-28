use std::{collections::HashMap, path::Path};
use glob::glob;

type Route = String;
type ViewPath = String;

pub struct FrostRouter {
  routes: HashMap<Route, ViewPath>,
  pub base: String
}

impl FrostRouter {
  pub fn new(base: &str) -> Option<Self> {
    let base = Path::new("./src").join(base);
    let base = base.to_str().unwrap();
    let mut routes = HashMap::new();

    #[cfg(debug_assertions)]
    println!("mapping routes from {base}:");

    for entry in glob(&format!("./{base}/**/index.lua")).expect("Failed to read glob pattern") {
      match entry {
        Ok(path) => {
          let mut route: String = String::from("./");
          route.push_str(&String::from(path.to_str().unwrap()));
          if !route.starts_with(base) {
            return None;
          }
          let route = route.strip_prefix(base).unwrap();
          if !route.ends_with("index.lua") {
            return None;
          }
          let mut route = route.strip_suffix("index.lua").unwrap();
          if route.len() > 1 && route.ends_with('/') {
            route = &route[0..route.len()-1]
          }

          #[cfg(debug_assertions)]
          println!("  {} -> {}", path.to_str().unwrap(), route);

          routes.insert(route.to_string(), String::from(path.to_str().unwrap()));

        },
        Err(e) => println!("{:?}", e),
      }
    }

    Some(Self { routes, base: base.to_string() })
  }

  pub fn routes(&self) -> HashMap<Route, ViewPath> {
    self.routes.clone()
  }


}

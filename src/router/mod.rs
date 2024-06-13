use std::{collections::HashMap, path::Path};
use glob::glob;
use regex::Regex;

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

    parse_routes(&mut routes, base);

    #[cfg(debug_assertions)]
    for (key, value) in routes.clone() {
      println!("  {key} -> {value}");
    }

    Some(Self { routes, base: base.to_string() })
  }

  pub fn routes(&self) -> HashMap<Route, ViewPath> {
    self.routes.clone()
  }


}


pub fn parse_routes(routes: &mut HashMap<String, String>, base: &str) {
  let regex = Regex::new(r"\[([^}]+)\]").unwrap();
  for entry in glob(&format!("./{base}/**/index.lua")).expect("Failed to read glob pattern") {
      match entry {
        Ok(path) => {
          let mut route: String = String::from("./");
          route.push_str(&String::from(path.to_str().unwrap()));
          if !route.starts_with(base) {
            return;
          }
          let route = route.strip_prefix(base).unwrap();
          if !route.ends_with("index.lua") {
            return;
          }
          let mut route = route.strip_suffix("index.lua").unwrap();
          if route.len() > 1 && route.ends_with('/') {
            route = &route[0..route.len()-1]
          }

          let mut new_route = vec![];

          for x in route.split('/').collect::<Vec<&str>>() {
            if !regex.is_match(x) {
              new_route.push(x.to_string());
              continue;
            }
            let route = x.replace("[", ":");
            let route = route.replace("]", "");
            new_route.push(route)
          }

          let route = new_route.join("/");

          routes.insert(route.to_string(), String::from(path.to_str().unwrap()));

        },
        Err(e) => println!("{:?}", e),
      }
    }
}

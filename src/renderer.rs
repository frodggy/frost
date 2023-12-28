use std::fs;

use iron::{Request, IronResult, Response, status, mime::{TopLevel, Mime, SubLevel}, headers::ContentType};
use mlua::Lua;
use router::Router;


const XML: &str = include_str!("lua/xml-generator.lua");

#[derive(Clone)]
pub struct FrostRenderer {
  path: String,
}

impl FrostRenderer {

  pub fn new(path: &str) -> Self {
    Self {
      path: path.to_string(),
    }
  }

  pub fn render(&self) -> String {
    let path = self.path.clone();

    let lua = Lua::new();

    let xml = lua.load(XML).eval::<mlua::Table>().unwrap();

    let table: mlua::Table = lua.create_table().unwrap();
    table.set("xml", xml.get::<&str, mlua::Value>("xml").unwrap()).unwrap();

    lua.globals().set("frost", table).unwrap();

    let contents = fs::read_to_string(&path).unwrap();
    let output = match lua.load(&contents).eval::<mlua::Table>() {
        Ok(output) => output,
        Err(err) => {
          match err {
            mlua::Error::FromLuaConversionError { ref from, .. } => {
              if from == &"nil" {
                panic!("{path} does not reture a page");
              } else {
                panic!("{err}");
              }
            }
            _ => panic!("{err}"),
          }
        },
    };
    let node_to_string = xml.get::<&str, mlua::Function>("node_to_string").unwrap();
    let html = node_to_string.call::<_, mlua::String>((output,)).unwrap();
    let html = html.to_str().unwrap();

    html.to_string()
  }

  pub fn render_with_param(&self, param: &str) -> String {
    let path = self.path.clone();

    let lua = Lua::new();

    let xml = lua.load(XML).eval::<mlua::Table>().unwrap();

    let table: mlua::Table = lua.create_table().unwrap();
    table.set("xml", xml.get::<&str, mlua::Value>("xml").unwrap()).unwrap();
    table.set("param", param).unwrap();

    lua.globals().set("frost", table).unwrap();

    let contents = fs::read_to_string(&path).unwrap();
    let output = match lua.load(&contents).eval::<mlua::Table>() {
        Ok(output) => output,
        Err(err) => {
          match err {
            mlua::Error::FromLuaConversionError { ref from, .. } => {
              if from == &"nil" {
                panic!("{path} does not reture a page");
              } else {
                panic!("{err}");
              }
            }
            _ => panic!("{err}"),
          }
        },
    };
    let node_to_string = xml.get::<&str, mlua::Function>("node_to_string").unwrap();
    let html = node_to_string.call::<_, mlua::String>((output,)).unwrap();
    let html = html.to_str().unwrap();

    html.to_string()
  }
}

pub fn render_handler(path:String, route: String) -> impl Fn(&mut Request) -> IronResult<Response> {

  let handler= move |req: &mut Request| {
    let renderer = FrostRenderer::new(&path.clone());
    let mut response;
    if route.contains(":") {
      let name = route.split(":").last().unwrap();
      let param = req.extensions.get::<Router>().unwrap().find(name).unwrap();
      response = Response::with((status::Ok, renderer.render_with_param(param)));
    } else {
      response = Response::with((status::Ok, renderer.render()));
    }

    response.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
    Ok(response)
  };

  handler
}

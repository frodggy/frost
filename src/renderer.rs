use std::{fs, collections::HashMap};

use iron::{Request, IronResult, Response, status, mime::{TopLevel, Mime, SubLevel}, headers::ContentType};
use mlua::{Lua, LuaSerdeExt};
use router::Router;
use serde::Serialize;

use crate::lua::get_lua;

#[derive(Serialize, Clone)]
struct FrostResponse {
  #[serde(rename = "frst__status")]
  frst_status: String,
  error: String
}


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

  pub fn render(&self, params: HashMap<&str, &str>) -> String {
    let path = self.path.clone();

    let lua = Lua::new();
    let lua_files = get_lua();


    let xml = lua.load(lua_files.xml).eval::<mlua::Table>().unwrap();
    let table_to_string = lua.load(lua_files.table_to_string).eval::<mlua::Function>().unwrap();

    let table: mlua::Table = lua.create_table().unwrap();
    let actions = lua.create_table().unwrap();

    let param_table = lua.create_table().unwrap();
    for (i, param) in params {
      let key = lua.create_string(i).unwrap();
      let value = lua.create_string(param).unwrap();
      param_table.set(key, value).unwrap();
    }

     let request = lua.create_function(|_lua, (url,): (String,)| {
      let response = match reqwest::blocking::get(&url) {
        Ok(resp) => resp,
        Err(err) => {
          let response = FrostResponse {
            frst_status: "error".to_string(),
            error: err.to_string()
          };

          return Ok(_lua.to_value(&response).unwrap());
        }
      };

      let response = match response.json::<serde_json::Value>() {
        Ok(json) => json,
        Err(_) => {
          let response = FrostResponse {
            frst_status: "error".to_string(),
            error: "Failed to parse JSON".to_string()
          };

          return Ok(_lua.to_value(&response).unwrap());
        }
      };

      Ok(_lua.to_value(&response).unwrap())
    }).unwrap();

    let invoke = lua.create_function(|_lua, (name,): (String,)| {
      let func = _lua.create_function(|_lua, (args,): (Vec<mlua::Value>,)| {

        Ok(())
      });
      Ok(func)
    }).unwrap();

    let action = lua.create_function (|_lua, (name,): (String,)| {
      let func = _lua.create_function(|_lua, (func,): (mlua::Function,)| {

        Ok(())
      });
      Ok(func)
    }).unwrap();

    table.set("action", action).unwrap();
    table.set("invoke", invoke).unwrap();
    table.set("table_to_string", table_to_string).unwrap();
    table.set("params", param_table).unwrap();
    table.set("xml", xml.get::<&str, mlua::Value>("xml").unwrap()).unwrap();
    table.set("request", request).unwrap();

    lua.globals().set("frost", table).unwrap();

    let contents = fs::read_to_string(&path).unwrap();
    let output = match lua.load(&contents).eval::<mlua::Table>() {
        Ok(output) => output,
        Err(_) => lua.load("return frost.xml.html").eval::<mlua::Table>().unwrap()
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
    let mut params = HashMap::new();

    for part in route.split('/') {
      if part.starts_with(':') {
        let name = part.strip_prefix(':').unwrap();
        let param = req.extensions.get::<Router>().unwrap().find(name).unwrap();
        params.insert(name, param);
      }
    }
      response = Response::with((status::Ok, renderer.render(params)));

    response.headers.set(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
    Ok(response)
  };

  handler
}

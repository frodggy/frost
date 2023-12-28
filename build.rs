use std::{fs::{File, self}, io, path::Path};


fn main() {
  println!("cargo:rerun-if-changed=build.rs");

  let mut resp = reqwest::blocking::get("https://raw.githubusercontent.com/Frityet/LuaXMLGenerator/main/xml-generator.lua").expect("request failed");
  if !Path::new("src/lua").exists() {
    fs::create_dir("src/lua").expect("failed to create directory");
  }
  let mut out = File::create("src/lua/xml-generator.lua").expect("failed to create file");
  io::copy(&mut resp, &mut out).expect("failed to copy content");
}

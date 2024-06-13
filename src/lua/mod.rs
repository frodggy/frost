pub struct FrostLuaLoader {
  pub xml: String,
  pub table_to_string: String,
}

pub fn get_lua() -> FrostLuaLoader {
  FrostLuaLoader {
    xml: String::from(include_str!("xml-generator.lua")),
    table_to_string: String::from(include_str!("table-to_string.lua"))
  }
}

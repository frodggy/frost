local xml = frost.xml

local resp = frost.request("http://localhost:3000/v1/games/get-games")


return xml.html { charset = "utf-8", lang = "en" } {
  xml.head {
    xml.title "Hello World"
  },
  xml.body {
    xml.h1 "Hello World",

    xml.div { id = "numbers" } {
      (frost.table_to_string(resp))
    }
  }
}

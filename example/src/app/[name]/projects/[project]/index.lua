local xml = frost.xml
local name, project = frost.params.name, frost.params.project

return xml.html { charset = "utf-8", lang = "en" } {
  xml.head {
    xml.title "Hello World"
  },
  xml.body {
    xml.h1("@" .. name .. "/" .. project),
  }
}

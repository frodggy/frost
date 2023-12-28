local xml = frost.xml
local name = frost.param

return xml.html {charset="utf-8", lang="en"} {
  xml.head {
    xml.title "Hello World"
  },
  xml.body {
    xml.h1 ("Hello " .. name .. "!"),
  }
}
